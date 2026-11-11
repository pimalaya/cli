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
//! status, 2 or above, and prints nothing more.

use std::{
    backtrace::{Backtrace, BacktraceStatus},
    fmt, process,
};

use anyhow::{Error, Result};
use log::{Level, log_enabled};
use schemars::JsonSchema;
use serde::{Serialize, Serializer};

use crate::printer::Printer;

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

/// The exit status of a command that ran to its end and printed its
/// result, whose outcome is not a full success. A command may document
/// higher statuses for other such outcomes.
pub const EXIT_PARTIAL: i32 = 2;

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
    /// success or as a failure object.
    pub fn new(status: i32) -> Self {
        assert!(
            status >= EXIT_PARTIAL,
            "partial outcome status {status} < 2"
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

/// The code of the outermost [`CodedError`] in the chain, if any.
pub fn code_of<C: ErrorCode>(err: &Error) -> Option<C> {
    err.chain()
        .find_map(|cause| cause.downcast_ref::<CodedError<C>>())
        .map(|coded| coded.code.clone())
}

/// Wraps an [`anyhow::Error`] to render it, with its source chain,
/// backtrace and code, as text or JSON.
pub struct ErrorReport<C = NoErrorCode> {
    error: Error,
    code: Option<C>,
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
    /// code of the chain when it carries one, and exits with status 1.
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

                let _ = printer.out(Self::from(err));
                process::exit(EXIT_FAILURE);
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
        let code = code_of(&error);

        Self {
            error,
            code,
            log_level_hint: "Run with --log-level to enable more verbose logs",
        }
    }
}

impl<C: ErrorCode> fmt::Display for ErrorReport<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {}", self.error)?;

        let mut header_printed = false;
        for err in self.sources() {
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
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "ErrorReport".into()
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        ErrorReportOutput::<C>::json_schema(generator)
    }
}

// NOTE: the one shape both the serialization and the schema of
// ErrorReport come from. Its doc comment below is the schema description.

/// The failure object a command prints instead of its output.
#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct ErrorReportOutput<C> {
    /// The stable code of the failure, `null` when the binary gave it
    /// none.
    code: Option<C>,
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
}
