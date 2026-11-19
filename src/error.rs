//! Command failure reporting to stdout.
//!
//! [`ErrorReport`] renders a failure, with its source chain and
//! backtrace, as text or JSON. A binary acting as a machine contract
//! also gives the failures a caller is expected to act on a stable
//! code: it declares its own [`ErrorCode`] vocabulary, raises a
//! [`CodedError`] anywhere in the chain, and evaluates its result with
//! [`ErrorReport::eval_coded`], which prints the code as `code` beside
//! the message. The message stays for humans and may be reworded at any
//! time; the code is the contract.
//!
//! A command that ran to its end but did not fully succeed prints its
//! result, then returns a [`PartialOutcome`]: the process exits with its
//! status, 3 or above, and prints nothing more. A command line the grammar
//! refuses exits 2, its failure object coded `usage-invalid` under
//! `--json` (see [`crate::clap::parse`]).
//!
//! A command acting on several items lists the failure of each single
//! item as an [`ItemError`] in its result's `errors`, then exits 3.

use std::{
    backtrace::{Backtrace, BacktraceStatus},
    borrow::Cow,
    fmt, process,
};

use anyhow::{Error, Result};
use log::{Level, log_enabled};
use schemars::JsonSchema;
use serde::{Serialize, Serializer};
use serde_json::Value;

use crate::{columns::Columns, printer::Printer, sanitize};

/// A binary's vocabulary of stable failure codes.
///
/// Implemented for every type with the needed bounds. The vocabulary
/// belongs to the binary, but its values are kebab-case across the
/// family, so a code enum derives `Serialize` with
/// `#[serde(rename_all = "kebab-case")]`.
pub trait ErrorCode: Clone + fmt::Debug + Serialize + JsonSchema + Send + Sync + 'static {}

impl<T: Clone + fmt::Debug + Serialize + JsonSchema + Send + Sync + 'static> ErrorCode for T {}

/// The empty vocabulary of a binary whose failures carry no code.
#[derive(Clone, Copy, Debug, Serialize, JsonSchema)]
pub enum NoErrorCode {}

/// The codes the family shares, beside every binary's own vocabulary:
/// pimalaya-cli raises `usage-invalid` itself, a binary raises the others.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub enum CommonCode {
    /// The command line does not match the grammar: an unknown flag or
    /// command, a missing argument, a bad value.
    UsageInvalid,
    /// A raw protocol request names a URL off the origin of the API the
    /// account's token is for, refused before anything is sent.
    RequestOrigin,
}

/// The failure of a command line the grammar refuses, coded
/// [`CommonCode::UsageInvalid`], with clap's message stripped of its
/// `error: ` prefix and of its styles.
pub fn usage_error(err: &clap::Error) -> Error {
    let rendered = err.render().to_string();
    let message = rendered.strip_prefix("error: ").unwrap_or(&rendered);
    usage_invalid(message.trim_end())
}

/// A failure coded [`CommonCode::UsageInvalid`] with its own message, for
/// arguments refused before clap sees them.
pub(crate) fn usage_invalid(message: impl Into<String>) -> Error {
    CodedError::new(CommonCode::UsageInvalid, message).into()
}

/// The refusal of a raw request to `url`, coded
/// [`CommonCode::RequestOrigin`]: its origin is not `api`, the origin of
/// the API whose bearer token the request would carry.
///
/// Both are origins as a URL serializes them, `https://host[:port]`. A
/// binary raises it before sending anything, so the token never leaves
/// for another host.
pub fn request_origin(url: &str, api: &str) -> Error {
    let message = format!(
        "Cannot send a request to `{url}`: it is off the API origin `{api}`, \
         which alone receives the account's token"
    );
    CodedError::new(CommonCode::RequestOrigin, message).into()
}

/// A failure carrying an [`ErrorCode`], anywhere in an error chain.
#[derive(Debug)]
pub struct CodedError<C> {
    code: C,
    message: String,
}

impl<C> CodedError<C> {
    /// Builds a failure with its code and its human message.
    pub fn new(code: C, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl<C> fmt::Display for CodedError<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl<C: fmt::Debug> std::error::Error for CodedError<C> {}

/// The exit status of a failed command, which printed the failure
/// object.
pub const EXIT_FAILURE: i32 = 1;

/// The exit status of a command line the grammar refuses, clap's own.
pub const EXIT_USAGE: i32 = 2;

/// The exit status of a command that ran to its end and printed its
/// result, whose outcome is not a full success. A command may document
/// higher statuses for other such outcomes.
pub const EXIT_PARTIAL: i32 = 3;

/// The outcome of a command that printed its result but did not fully
/// succeed, such as a check with a failed backend or a refusal from a
/// server.
///
/// A command returns it as its error once its result is printed:
/// [`ErrorReport::eval_coded`] then exits with its status without
/// printing anything more.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PartialOutcome {
    status: i32,
}

impl PartialOutcome {
    /// Builds the outcome exiting with the given status.
    ///
    /// # Panics
    ///
    /// When the status is below [`EXIT_PARTIAL`], which would read as a
    /// success, a failure object or a usage error.
    pub fn new(status: i32) -> Self {
        assert!(
            status >= EXIT_PARTIAL,
            "partial outcome status {status} < {EXIT_PARTIAL}"
        );
        Self { status }
    }

    /// The exit status.
    pub fn status(&self) -> i32 {
        self.status
    }
}

impl Default for PartialOutcome {
    fn default() -> Self {
        Self::new(EXIT_PARTIAL)
    }
}

impl fmt::Display for PartialOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Command ended with exit status {}", self.status)
    }
}

impl std::error::Error for PartialOutcome {}

/// The [`PartialOutcome`] in the chain, if any.
pub fn partial_outcome_of(err: &Error) -> Option<PartialOutcome> {
    err.chain()
        .find_map(|cause| cause.downcast_ref::<PartialOutcome>())
        .copied()
}

/// The status a failure exits with: [`EXIT_USAGE`] when its chain is
/// coded [`CommonCode::UsageInvalid`], a command line refused after clap
/// parsed it, [`EXIT_FAILURE`] otherwise.
pub fn failure_status(err: &Error) -> i32 {
    match code_of::<CommonCode>(err) {
        Some(CommonCode::UsageInvalid) => EXIT_USAGE,
        Some(CommonCode::RequestOrigin) | None => EXIT_FAILURE,
    }
}

/// `err` as the report reads it, the code `code` finds in its chain
/// carried by an innermost [`CodedError`].
///
/// A binary may read a code off a typed error deep in the chain (a
/// protocol refusal, a bare 403 or 429), which [`code_of`] does not see.
/// `err` is returned unchanged when the chain already carries a code of
/// `C` or of [`CommonCode`], or a [`PartialOutcome`], or when `code` finds
/// none; else it is rebuilt with the same messages, outermost first.
pub fn with_code<C: ErrorCode>(err: Error, code: impl Fn(&Error) -> Option<C>) -> Error {
    let kept = code_of::<C>(&err).is_some()
        || code_of::<CommonCode>(&err).is_some()
        || partial_outcome_of(&err).is_some();
    if kept {
        return err;
    }

    let Some(code) = code(&err) else {
        return err;
    };

    let mut messages: Vec<String> = err.chain().map(ToString::to_string).collect();
    let innermost = messages.pop().unwrap_or_default();
    let coded = Error::new(CodedError::new(code, innermost));

    messages
        .into_iter()
        .rev()
        .fold(coded, |err, message| err.context(message))
}

/// The code of the outermost [`CodedError`] in the chain, if any.
pub fn code_of<C: ErrorCode>(err: &Error) -> Option<C> {
    err.chain()
        .find_map(|cause| cause.downcast_ref::<CodedError<C>>())
        .map(|coded| coded.code.clone())
}

/// The text hint naming `--json` when the text report replaced characters.
const JSON_HINT: &str = "Run with --json to see the exact characters of the error";

/// Wraps an [`anyhow::Error`] to render it, with its source chain,
/// backtrace and code, as text or JSON.
///
/// The text sanitizes the message and each source line by line
/// ([`sanitize::lines`]), naming `--json` when it replaced a character;
/// the JSON keeps them exact.
pub struct ErrorReport<C = NoErrorCode> {
    error: Error,
    code: Option<ReportCode<C>>,
    /// The text hint naming `--log-level` while debug logs are off.
    pub(crate) log_level_hint: &'static str,
}

impl ErrorReport {
    /// Unwraps a result, or prints the error to the printer and exits
    /// with status 1.
    pub fn eval<T>(printer: &mut impl Printer, result: Result<T, Error>) -> T {
        Self::eval_coded(printer, result)
    }
}

impl<C: ErrorCode> ErrorReport<C> {
    /// Unwraps a result, or prints the error to the printer, with the
    /// code of the chain when it carries one, and exits with status 1,
    /// or 2 for a refused command line (see [`failure_status`]).
    ///
    /// A [`PartialOutcome`] in the chain means the result is already
    /// printed: the process exits with its status, printing nothing more.
    pub fn eval_coded<T>(printer: &mut impl Printer, result: Result<T, Error>) -> T {
        match result {
            Ok(res) => res,
            Err(err) => {
                if let Some(outcome) = partial_outcome_of(&err) {
                    process::exit(outcome.status());
                }

                let status = failure_status(&err);
                let _ = printer.out(Self::from(err));
                process::exit(status);
            }
        }
    }

    fn sources(&self) -> impl Iterator<Item = String> + '_ {
        self.error.chain().skip(1).map(ToString::to_string)
    }

    fn suggestions(&self) -> Option<&str> {
        if !log_enabled!(Level::Debug) {
            Some(self.log_level_hint)
        } else {
            None
        }
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        let backtrace = self.error.backtrace();

        if let BacktraceStatus::Captured = backtrace.status() {
            Some(backtrace)
        } else {
            None
        }
    }
}

impl<C: ErrorCode> From<Error> for ErrorReport<C> {
    fn from(error: Error) -> Self {
        let code = code_of(&error)
            .map(ReportCode::Binary)
            .or_else(|| code_of::<CommonCode>(&error).map(ReportCode::Common));

        Self {
            error,
            code,
            log_level_hint: "Run with --log-level to enable more verbose logs",
        }
    }
}

impl<C: ErrorCode> fmt::Display for ErrorReport<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error = self.error.to_string();
        let error = sanitize::lines(&error);
        let mut defused = matches!(error, Cow::Owned(_));
        write!(f, "Error: {error}")?;

        let mut header_printed = false;
        for err in self.sources() {
            let err = sanitize::lines(&err);
            defused |= matches!(err, Cow::Owned(_));

            if !header_printed {
                writeln!(f)?;
                writeln!(f)?;
                write!(f, "Caused by:")?;
                header_printed = true;
            }

            writeln!(f)?;
            write!(f, " - {}", err.trim())?;
        }

        if let Some(backtrace) = self.backtrace() {
            writeln!(f)?;
            writeln!(f)?;
            writeln!(f, "Backtrace:")?;
            write!(f, "{backtrace}")?;
        }

        if let Some(suggestion) = self.suggestions() {
            writeln!(f)?;
            writeln!(f)?;
            writeln!(f, "Suggestions:")?;
            write!(f, " - {suggestion}")?;
        }

        if defused {
            if self.suggestions().is_none() {
                writeln!(f)?;
                writeln!(f)?;
                write!(f, "Suggestions:")?;
            }

            writeln!(f)?;
            write!(f, " - {JSON_HINT}")?;
        }

        writeln!(f)
    }
}

impl<C: ErrorCode> Serialize for ErrorReport<C> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let output = ErrorReportOutput {
            code: self.code.clone(),
            error: self.error.to_string(),
            sources: self.sources().collect(),
            backtrace: self.backtrace().map(ToString::to_string),
        };

        output.serialize(serializer)
    }
}

impl<C: ErrorCode> JsonSchema for ErrorReport<C> {
    fn schema_name() -> Cow<'static, str> {
        "ErrorReport".into()
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        ErrorReportOutput::<C>::json_schema(generator)
    }
}

/// The error of one item a command could not list or act on.
#[derive(Clone, Debug, Serialize, JsonSchema)]
pub struct ItemError<C = NoErrorCode> {
    /// The item's id as the command prints ids, `null` when unknown.
    pub id: Option<String>,
    /// The stable code of the failure, `null` when none was given.
    pub code: Option<C>,
    /// The outermost error message.
    pub error: String,
    /// The messages of the causes, outermost first.
    pub sources: Vec<String>,
}

impl<C: ErrorCode> ItemError<C> {
    /// Builds the error of the item `id` from `err`, coded by the
    /// outermost [`CodedError`] of its chain.
    pub fn new(id: Option<&str>, err: &Error) -> Self {
        Self {
            id: id.map(String::from),
            code: code_of(err),
            error: err.to_string(),
            sources: err.chain().skip(1).map(ToString::to_string).collect(),
        }
    }
}

impl<C: ErrorCode> ItemError<C> {
    /// The id, the code and the message, sanitized, an absent one `-`.
    fn cells(&self) -> [String; 3] {
        let id = sanitize::text(self.id.as_deref().unwrap_or("-")).into_owned();

        let code = match &self.code {
            Some(code) => serde_json::to_value(code).unwrap_or(Value::Null),
            None => Value::Null,
        };
        let code = code.as_str().unwrap_or("-").to_owned();

        [id, code, sanitize::text(&self.error).into_owned()]
    }
}

impl<C: ErrorCode> fmt::Display for ItemError<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [id, code, error] = self.cells();
        write!(f, "{id}  {code}  {error}")
    }
}

/// The item errors a command prints last, one aligned `Error:` line each.
///
/// A command prints them after its output and one blank line, and prints
/// nothing, blank line included, when there are none.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ItemErrors {
    /// The id, code and message cells of each error, in print order.
    pub rows: Vec<[String; 3]>,
}

impl ItemErrors {
    /// Appends the line of one item error.
    pub fn push<C: ErrorCode>(&mut self, error: &ItemError<C>) {
        self.rows.push(error.cells());
    }

    /// Whether no error was pushed.
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// Writes one blank line then the errors, nothing when there are none.
    pub fn write_after(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            return Ok(());
        }

        writeln!(f)?;
        write!(f, "{self}")
    }
}

impl fmt::Display for ItemErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut columns = Columns::default();

        for [id, code, error] in &self.rows {
            columns.push(["Error:", id, code, error]);
        }

        write!(f, "{columns}")
    }
}

/// The code of a failure object: the binary's own, or one pimalaya-cli
/// raises itself.
#[derive(Clone, Debug, Serialize, JsonSchema)]
#[serde(untagged)]
enum ReportCode<C> {
    Binary(C),
    Common(CommonCode),
}

// NOTE: the one shape both the serialization and the schema of
// ErrorReport come from. Its doc comment below is the schema description.

/// The failure object a command prints instead of its output.
#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct ErrorReportOutput<C> {
    /// The stable code of the failure: the binary's own, or one the family
    /// shares (`usage-invalid` for a command line the grammar refuses,
    /// `request-origin` for a request off its API), `null` when none was
    /// given.
    code: Option<ReportCode<C>>,
    /// The outermost error message.
    error: String,
    /// The messages of the causes, outermost first.
    sources: Vec<String>,
    /// The captured backtrace, when backtraces are enabled.
    backtrace: Option<String>,
}

#[cfg(test)]
mod tests {
    use anyhow::{Context, anyhow};
    use serde::Serialize;
    use serde_json::json;

    use crate::{clap::commands::json_schema::output_schema, error::*};

    #[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, JsonSchema)]
    #[serde(rename_all = "kebab-case")]
    enum TestCode {
        BodyPending,
    }

    fn pending() -> Result<(), Error> {
        Err(CodedError::new(TestCode::BodyPending, "not downloaded yet").into())
    }

    /// A code read off a typed error deep in the chain reaches the
    /// report, every message kept; a chain already coded is left alone.
    #[test]
    fn a_code_read_deep_in_the_chain_reaches_the_report() {
        #[derive(Debug)]
        struct Refused;

        impl fmt::Display for Refused {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("403 Forbidden")
            }
        }

        impl std::error::Error for Refused {}

        let code = |err: &Error| {
            err.chain()
                .any(|cause| cause.is::<Refused>())
                .then_some(TestCode::BodyPending)
        };

        let err = Error::new(Refused)
            .context("Cannot fetch")
            .context("Cannot sync");
        let coded = with_code(err, code);

        assert_eq!(code_of(&coded), Some(TestCode::BodyPending));
        let messages: Vec<String> = coded.chain().map(ToString::to_string).collect();
        assert_eq!(messages, ["Cannot sync", "Cannot fetch", "403 Forbidden"]);

        let refused = usage_invalid("both on stdin");
        let kept = with_code(refused, |_| Some(TestCode::BodyPending));
        assert_eq!(code_of::<TestCode>(&kept), None);
        assert_eq!(failure_status(&kept), EXIT_USAGE);
    }

    /// A command line refused after clap parsed it exits 2, as clap's own
    /// refusals do, wherever the usage error sits in the chain; any other
    /// failure exits 1.
    #[test]
    fn a_refused_command_line_exits_two() {
        let conflict = clap::Error::raw(clap::error::ErrorKind::ArgumentConflict, "both on stdin");
        let refused = usage_error(&conflict).context("Cannot apply");

        assert_eq!(failure_status(&refused), EXIT_USAGE);
        assert_eq!(failure_status(&pending().unwrap_err()), EXIT_FAILURE);
    }

    #[test]
    fn a_coded_error_keeps_its_code_under_context() {
        let err = pending().context("Read message 3").unwrap_err();
        assert_eq!(code_of(&err), Some(TestCode::BodyPending));
    }

    #[test]
    fn the_json_report_carries_the_code_beside_the_message() {
        let err = pending().unwrap_err();
        let json = serde_json::to_value(ErrorReport::<TestCode>::from(err)).unwrap();

        assert_eq!(json["code"], "body-pending");
        assert_eq!(json["error"], "not downloaded yet");
    }

    #[test]
    fn an_uncoded_error_prints_a_null_code() {
        let err = anyhow!("boom").context("Read message 3");
        let json = serde_json::to_value(ErrorReport::<TestCode>::from(err)).unwrap();

        assert_eq!(
            json,
            json!({ "code": null, "error": "Read message 3", "sources": ["boom"], "backtrace": null })
        );
    }

    #[test]
    fn the_schema_types_the_code_by_the_vocabulary() {
        let schema = output_schema::<ErrorReport<TestCode>>();
        let required = schema["required"].as_array().unwrap();

        assert!(required.contains(&json!("code")), "{schema}");
        assert!(required.contains(&json!("error")), "{schema}");
        assert_eq!(schema["$defs"]["TestCode"]["enum"], json!(["body-pending"]));
    }

    #[derive(Debug, clap::Parser)]
    struct App {
        #[arg(long)]
        count: u8,
    }

    fn refused(args: &[&str]) -> serde_json::Value {
        let err = <App as clap::Parser>::try_parse_from(args).unwrap_err();
        serde_json::to_value(ErrorReport::<TestCode>::from(usage_error(&err))).unwrap()
    }

    #[test]
    fn a_refused_command_line_is_coded_usage_invalid_without_prefix() {
        for args in [&["app", "--nope"][..], &["app"], &["app", "--count", "x"]] {
            let json = refused(args);
            let error = json["error"].as_str().unwrap();

            assert_eq!(json["code"], "usage-invalid", "{json}");
            assert!(!error.starts_with("error:"), "{error}");
            assert!(!error.contains('\x1b'), "{error}");
        }
    }

    #[test]
    fn the_schema_admits_the_common_codes() {
        let schema = output_schema::<ErrorReport<TestCode>>().to_string();
        assert!(schema.contains("usage-invalid"), "{schema}");
        assert!(schema.contains("request-origin"), "{schema}");
    }

    /// A request off the API is refused coded `request-origin` beside the
    /// binary's own vocabulary, exits 1 and keeps its code under context;
    /// a binary's later read of a code leaves it alone.
    #[test]
    fn a_request_off_the_api_is_coded_request_origin() {
        let api = "https://graph.microsoft.com";
        let err = request_origin("https://evil.example", api).context("Cannot send request");

        assert_eq!(failure_status(&err), EXIT_FAILURE);
        let err = with_code(err, |_| Some(TestCode::BodyPending));
        assert_eq!(code_of::<TestCode>(&err), None);

        let json = serde_json::to_value(ErrorReport::<TestCode>::from(err)).unwrap();
        assert_eq!(json["code"], "request-origin", "{json}");
        assert_eq!(json["error"], "Cannot send request");
        assert_eq!(
            json["sources"][0],
            "Cannot send a request to `https://evil.example`: it is off the API \
             origin `https://graph.microsoft.com`, which alone receives the \
             account's token"
        );
    }

    #[test]
    fn a_partial_outcome_keeps_its_status_under_context() {
        let err = Error::from(PartialOutcome::new(3)).context("Check account");
        assert_eq!(partial_outcome_of(&err).map(|o| o.status()), Some(3));
        assert_eq!(partial_outcome_of(&anyhow!("boom")), None);
    }

    #[test]
    #[should_panic]
    fn a_partial_outcome_cannot_read_as_a_failure() {
        PartialOutcome::new(1);
    }

    fn unreadable() -> Error {
        pending()
            .context("Cannot decode line")
            .context("Cannot read message 34")
            .unwrap_err()
    }

    #[test]
    fn an_item_error_finds_the_code_under_context() {
        let json = serde_json::to_value(ItemError::<TestCode>::new(Some("34"), &unreadable()));

        assert_eq!(
            json.unwrap(),
            json!({
                "id": "34",
                "code": "body-pending",
                "error": "Cannot read message 34",
                "sources": ["Cannot decode line", "not downloaded yet"],
            })
        );
    }

    #[test]
    fn an_item_error_schema_requires_every_key() {
        let schema = output_schema::<ItemError<TestCode>>();
        let required = schema["required"].as_array().unwrap();

        for key in ["id", "code", "error", "sources"] {
            assert!(required.contains(&json!(key)), "{schema}");
        }

        assert_eq!(
            schema["properties"]["id"]["type"],
            json!(["string", "null"])
        );
        assert_eq!(
            schema["properties"]["code"]["anyOf"][1],
            json!({ "type": "null" })
        );
        assert_eq!(schema["$defs"]["TestCode"]["enum"], json!(["body-pending"]));
    }

    #[test]
    fn an_item_error_displays_as_one_line() {
        let coded = ItemError::<TestCode>::new(Some("34"), &unreadable());
        let uncoded = ItemError::<TestCode>::new(None, &anyhow!("Cannot read\nline"));

        assert_eq!(
            coded.to_string(),
            "34  body-pending  Cannot read message 34"
        );
        assert_eq!(uncoded.to_string(), "-  -  Cannot read\u{fffd}line");
    }

    #[test]
    fn a_text_report_defuses_the_chain_and_keeps_its_lines() {
        let err = anyhow!("Expected `=`\nat line 2\x1b]52;c;aGk=\x07")
            .context("Cannot read message nope\x1b[31mRED\x07");
        let report = ErrorReport::<NoErrorCode>::from(err).to_string();

        assert!(!report.contains(['\x1b', '\x07']), "{report:?}");
        assert!(
            report.starts_with("Error: Cannot read message nope\u{fffd}[31mRED\u{fffd}"),
            "{report:?}"
        );
        assert!(
            report.contains(" - Expected `=`\nat line 2\u{fffd}]52;c;aGk=\u{fffd}"),
            "{report:?}"
        );
        assert!(report.contains("--json"), "{report:?}");
    }
}
