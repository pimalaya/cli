//! # Account
//!
//! The outputs of every binary's `account list` and `account check`
//! commands: the accounts a configuration declares and the backends each
//! one carries, then whether each backend of the accounts checked answers
//! and the TLS it connected with. The `account capability` output lives in
//! the capability module.

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

/// The `account check` output, one entry per account checked.
#[derive(Clone, Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[schemars(rename = "AccountCheckOutput")]
pub struct AccountCheckOutput<C = NoErrorCode> {
    /// The accounts checked, in the order checked: the one `-a` names for
    /// a binary running against one account.
    pub accounts: Vec<AccountCheck<C>>,
}

impl<C> AccountCheckOutput<C> {
    /// Whether every backend of every account answered, the command
    /// exiting with a partial outcome otherwise.
    pub fn ok(&self) -> bool {
        let mut backends = self.accounts.iter().flat_map(|account| &account.backends);
        backends.all(|check| check.ok)
    }
}

/// One block per account, a blank line apart.
impl<C> fmt::Display for AccountCheckOutput<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, account) in self.accounts.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }

            write!(f, "{account}")?;
        }

        Ok(())
    }
}

/// One account checked, one outcome per backend.
#[derive(Clone, Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[schemars(rename = "AccountCheck")]
pub struct AccountCheck<C = NoErrorCode> {
    /// The account that was checked.
    pub account: String,
    /// One entry per backend the account declares and `--backend` allows.
    pub backends: Vec<BackendCheck<C>>,
}

impl<C> fmt::Display for AccountCheck<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fields = Fields::default();
        fields.push("Account", sanitize::text(&self.account));

        for check in &self.backends {
            let mut outcome = match &check.error {
                None => String::from("OK"),
                Some(err) => format!("FAIL ({})", sanitize::text(err)),
            };

            if let Some(tls) = &check.tls {
                outcome.push_str(", TLS");

                let tls = tls.to_string();
                if !tls.is_empty() {
                    outcome.push(' ');
                    outcome.push_str(&tls);
                }
            }

            fields.push(check.backend, outcome);
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
    /// The TLS the connection was configured with, `null` for a local
    /// backend or a cleartext connection.
    pub tls: Option<BackendCheckTls>,
}

impl<C: ErrorCode> BackendCheck<C> {
    /// Folds a connection attempt to `backend` into its outcome, coded by
    /// its chain (see `error::with_code` for a code read off a typed error).
    ///
    /// `tls` starts `null`: a binary sets it for a connection over TLS.
    pub fn new(backend: &'static str, result: Result<()>) -> Self {
        match result {
            Ok(()) => Self {
                backend,
                ok: true,
                code: None,
                error: None,
                tls: None,
            },
            Err(err) => Self {
                backend,
                ok: false,
                code: code_of(&err),
                error: Some(format!("{err:#}")),
                tls: None,
            },
        }
    }
}

/// The TLS a backend connection is configured with, as the binary's
/// `tls` table resolves it: `null` where the build's default answers.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BackendCheckTls {
    /// The TLS implementation, `rustls` or `native-tls`; `null` for the
    /// build's first.
    pub provider: Option<String>,
    /// The cryptographic provider of rustls, `aws` or `ring`; `null` for
    /// the build's first.
    pub crypto: Option<String>,
    /// The PEM certificate trusted beside the platform's; `null` for the
    /// platform's alone.
    pub cert: Option<String>,
    /// The PEM client certificate presented; `null` for none.
    pub client_cert: Option<String>,
}

/// The configured parts, `native-tls, cert /etc/ca.pem`, empty when the
/// build's defaults answer every one.
impl fmt::Display for BackendCheckTls {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parts = [
            self.provider.clone(),
            self.crypto.clone(),
            self.cert.as_ref().map(|cert| format!("cert {cert}")),
            self.client_cert
                .as_ref()
                .map(|cert| format!("client cert {cert}")),
        ];

        let parts: Vec<String> = parts.into_iter().flatten().collect();
        write!(f, "{}", sanitize::text(&parts.join(", ")))
    }
}

#[cfg(test)]
mod tests {
    use anyhow::anyhow;
    use serde::Serialize;
    use serde_json::{Value, json};

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
            accounts: vec![AccountCheck {
                account: String::from("work"),
                backends: vec![
                    BackendCheck::<TestCode>::new("imap", Ok(())),
                    BackendCheck::new("smtp", refused),
                ],
            }],
        };

        let expected = "\
Account:  work
imap:     OK
smtp:     FAIL (Cannot open the session: Bad credentials)
";
        assert_eq!(output.to_string(), expected);
        assert!(!output.ok());

        let json = serde_json::to_string(&output).unwrap();
        let expected = concat!(
            r#"{"accounts":[{"account":"work","backends":["#,
            r#"{"backend":"imap","ok":true,"code":null,"error":null,"tls":null},"#,
            r#"{"backend":"smtp","ok":false,"code":null,"#,
            r#""error":"Cannot open the session: Bad credentials","tls":null}]}]}"#,
        );
        assert_eq!(json, expected);
    }

    #[test]
    fn several_accounts_print_one_block_each() {
        let output = AccountCheckOutput {
            accounts: vec![
                AccountCheck {
                    account: String::from("home"),
                    backends: vec![BackendCheck::<TestCode>::new("maildir", Ok(()))],
                },
                AccountCheck {
                    account: String::from("work"),
                    backends: vec![BackendCheck::new("imap", Ok(()))],
                },
            ],
        };

        let expected = "\
Account:  home
maildir:  OK

Account:  work
imap:     OK
";
        assert_eq!(output.to_string(), expected);
        assert!(output.ok());
    }

    #[test]
    fn a_connection_over_tls_names_what_it_was_configured_with() {
        let mut configured = BackendCheck::<TestCode>::new("imap", Ok(()));
        configured.tls = Some(BackendCheckTls {
            provider: Some(String::from("native-tls")),
            crypto: None,
            cert: Some(String::from("/etc/ca.pem")),
            client_cert: Some(String::from("/me.pem")),
        });

        let mut defaults = BackendCheck::new("smtp", Err(anyhow!("Refused")));
        defaults.tls = Some(BackendCheckTls::default());

        let output = AccountCheckOutput {
            accounts: vec![AccountCheck {
                account: String::from("work"),
                backends: vec![configured, defaults, BackendCheck::new("maildir", Ok(()))],
            }],
        };

        let expected = "\
Account:  work
imap:     OK, TLS native-tls, cert /etc/ca.pem, client cert /me.pem
smtp:     FAIL (Refused), TLS
maildir:  OK
";
        assert_eq!(output.to_string(), expected);

        let json = serde_json::to_value(&output).unwrap();
        let backends = &json["accounts"][0]["backends"];
        assert_eq!(
            backends[0]["tls"],
            json!({
                "provider": "native-tls",
                "crypto": null,
                "cert": "/etc/ca.pem",
                "clientCert": "/me.pem",
            })
        );
        assert_eq!(
            backends[1]["tls"],
            json!({ "provider": null, "crypto": null, "cert": null, "clientCert": null })
        );
        assert_eq!(backends[2]["tls"], Value::Null);
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
        let mut check = BackendCheck::<TestCode>::new("imap", Err(anyhow!("evil\u{202e}reply")));
        check.tls = Some(BackendCheckTls {
            cert: Some(String::from("/ca\u{7}.pem")),
            ..BackendCheckTls::default()
        });

        let output = AccountCheckOutput {
            accounts: vec![AccountCheck {
                account: String::from("a\u{1b}b"),
                backends: vec![check],
            }],
        };

        let text = output.to_string();
        assert!(text.contains("a\u{fffd}b"), "{text}");
        assert!(text.contains("FAIL (evil\u{fffd}reply)"), "{text}");
        assert!(text.contains("TLS cert /ca\u{fffd}.pem"), "{text}");
    }

    #[test]
    fn the_check_schema_keeps_the_names_and_types_the_code() {
        let schema = output_schema::<AccountCheckOutput<TestCode>>();
        let account = &schema["$defs"]["AccountCheck"];
        let row = &schema["$defs"]["BackendCheck"];
        let tls = &schema["$defs"]["BackendCheckTls"];

        assert_eq!(schema["title"], "AccountCheckOutput", "{schema}");
        assert_eq!(schema["required"], json!(["accounts"]), "{schema}");
        assert_eq!(
            account["required"],
            json!(["account", "backends"]),
            "{schema}"
        );
        assert_eq!(
            row["required"],
            json!(["backend", "ok", "code", "error", "tls"]),
            "{schema}"
        );
        assert_eq!(
            tls["required"],
            json!(["provider", "crypto", "cert", "clientCert"]),
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
