//! # Account
//!
//! The outputs of every binary's `account list` and `account check`
//! commands: the accounts a configuration declares and the backends each
//! one carries, then whether each backend of one account answers. The
//! `account capability` output lives in the capability module.

use std::fmt;

use anyhow::Result;
use schemars::JsonSchema;
use serde::Serialize;

use crate::{
    columns::Columns,
    error::{ErrorCode, NoErrorCode, code_of},
    fields::Fields,
    sanitize,
};

/// The `account list` output, the accounts the configuration declares.
#[derive(Clone, Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AccountListOutput {
    /// The accounts, sorted by name.
    pub accounts: Vec<AccountRow>,
}

impl fmt::Display for AccountListOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut columns = Columns::default();

        for account in &self.accounts {
            let not_built = account
                .not_built
                .iter()
                .map(|name| format!("{name} (not built)"));
            let backends: Vec<String> = account
                .backends
                .iter()
                .map(ToString::to_string)
                .chain(not_built)
                .collect();

            columns.push([
                sanitize::text(&account.name).into_owned(),
                backends.join(","),
                String::from(if account.default { "default" } else { "" }),
            ]);
        }

        write!(f, "{columns}")
    }
}

/// One account of the list.
#[derive(Clone, Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AccountRow {
    /// The name of the `[accounts.<name>]` block.
    pub name: String,
    /// Whether a command with no `-a/--account` runs against it.
    pub default: bool,
    /// The backends it declares a block for that this build serves.
    pub backends: Vec<&'static str>,
    /// The backends it declares a block for that this build leaves out,
    /// which a command reaching one refuses.
    pub not_built: Vec<&'static str>,
}

/// The `account check` output, one outcome per backend.
#[derive(Clone, Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[schemars(rename = "AccountCheckOutput")]
pub struct AccountCheckOutput<C = NoErrorCode> {
    /// The account that was checked.
    pub account: String,
    /// One entry per backend the account declares and `--backend` allows.
    pub backends: Vec<BackendCheck<C>>,
}

impl<C> fmt::Display for AccountCheckOutput<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fields = Fields::default();
        fields.push("Account", sanitize::text(&self.account));

        for check in &self.backends {
            match &check.error {
                None => fields.push(check.backend, "OK"),
                Some(err) => fields.push(check.backend, format!("FAIL ({})", sanitize::text(err))),
            }
        }

        write!(f, "{fields}")
    }
}

/// Outcome of checking one backend's connection.
#[derive(Clone, Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[schemars(rename = "BackendCheck")]
pub struct BackendCheck<C = NoErrorCode> {
    /// The backend that was checked.
    pub backend: &'static str,
    /// Whether it answered.
    pub ok: bool,
    /// The stable code of the failure, `null` when it answered or the
    /// failure carries none.
    pub code: Option<C>,
    /// Why it did not answer, `null` when it did.
    pub error: Option<String>,
}

impl<C: ErrorCode> BackendCheck<C> {
    /// Folds a connection attempt to `backend` into its outcome, coded by
    /// its chain (see `error::with_code` for a code read off a typed error).
    pub fn new(backend: &'static str, result: Result<()>) -> Self {
        match result {
            Ok(()) => Self {
                backend,
                ok: true,
                code: None,
                error: None,
            },
            Err(err) => Self {
                backend,
                ok: false,
                code: code_of(&err),
                error: Some(format!("{err:#}")),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::anyhow;
    use serde::Serialize;
    use serde_json::json;

    use crate::{
        account::*,
        clap::commands::json_schema::output_schema,
        error::{CodedError, with_code},
    };

    #[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, JsonSchema)]
    #[serde(rename_all = "kebab-case")]
    enum TestCode {
        AuthFailed,
    }

    #[test]
    fn accounts_print_one_line_each_with_the_backends_left_out() {
        let output = AccountListOutput {
            accounts: vec![
                AccountRow {
                    name: String::from("home"),
                    default: false,
                    backends: vec!["maildir"],
                    not_built: vec![],
                },
                AccountRow {
                    name: String::from("work"),
                    default: true,
                    backends: vec!["imap"],
                    not_built: vec!["jmap", "smtp"],
                },
            ],
        };

        let expected = "\
home  maildir                                 -
work  imap,jmap (not built),smtp (not built)  default
";
        assert_eq!(output.to_string(), expected);

        let json = serde_json::to_string(&output.accounts[1]).unwrap();
        let expected =
            r#"{"name":"work","default":true,"backends":["imap"],"notBuilt":["jmap","smtp"]}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn a_check_prints_one_field_per_backend() {
        let refused = Err(anyhow!("Bad credentials").context("Cannot open the session"));

        let output = AccountCheckOutput {
            account: String::from("work"),
            backends: vec![
                BackendCheck::<TestCode>::new("imap", Ok(())),
                BackendCheck::new("smtp", refused),
            ],
        };

        let expected = "\
Account:  work
imap:     OK
smtp:     FAIL (Cannot open the session: Bad credentials)
";
        assert_eq!(output.to_string(), expected);

        let json = serde_json::to_string(&output).unwrap();
        let expected = concat!(
            r#"{"account":"work","backends":["#,
            r#"{"backend":"imap","ok":true,"code":null,"error":null},"#,
            r#"{"backend":"smtp","ok":false,"code":null,"#,
            r#""error":"Cannot open the session: Bad credentials"}]}"#,
        );
        assert_eq!(json, expected);
    }

    #[test]
    fn a_failed_check_is_coded_by_its_chain() {
        let coded = anyhow::Error::new(CodedError::new(TestCode::AuthFailed, "Bad credentials"));
        let check = BackendCheck::<TestCode>::new("imap", Err(coded.context("Cannot log in")));
        assert_eq!(check.code, Some(TestCode::AuthFailed));
        assert_eq!(
            check.error.as_deref(),
            Some("Cannot log in: Bad credentials")
        );

        // A code read off a typed error, handed over first.
        let typed = anyhow!("401 Unauthorized").context("Cannot log in");
        let typed = with_code(typed, |_| Some(TestCode::AuthFailed));
        let check = BackendCheck::<TestCode>::new("jmap", Err(typed));
        assert_eq!(check.code, Some(TestCode::AuthFailed));
        assert_eq!(
            check.error.as_deref(),
            Some("Cannot log in: 401 Unauthorized")
        );
    }

    #[test]
    fn untrusted_text_is_defused() {
        let output = AccountCheckOutput {
            account: String::from("a\u{1b}b"),
            backends: vec![BackendCheck::<TestCode>::new(
                "imap",
                Err(anyhow!("evil\u{202e}reply")),
            )],
        };

        let text = output.to_string();
        assert!(text.contains("a\u{fffd}b"), "{text}");
        assert!(text.contains("FAIL (evil\u{fffd}reply)"), "{text}");
    }

    #[test]
    fn the_check_schema_keeps_the_names_and_types_the_code() {
        let schema = output_schema::<AccountCheckOutput<TestCode>>();
        let row = &schema["$defs"]["BackendCheck"];

        assert_eq!(schema["title"], "AccountCheckOutput", "{schema}");
        assert_eq!(
            row["required"],
            json!(["backend", "ok", "code", "error"]),
            "{schema}"
        );
        assert_eq!(schema["$defs"]["TestCode"]["enum"], json!(["auth-failed"]));

        let schema = output_schema::<AccountListOutput>();
        let row = &schema["$defs"]["AccountRow"];
        assert_eq!(
            row["required"],
            json!(["name", "default", "backends", "notBuilt"]),
            "{schema}"
        );
    }
}
