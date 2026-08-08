//! OS-aware credential-provider picker shared by the account wizards.
//!
//! A secret is read from a well-known credential CLI, from a custom
//! shell command, or stored raw in the configuration. Two flavours share
//! the same machinery:
//!
//! - a **password** is read from an OS keyring ([`KeyringProvider`]);
//! - an **OAuth 2.0 access token** is read from a token broker
//!   ([`TokenBroker`]) that refreshes it on every read.
//!
//! A known provider or broker yields an argv command (no shell), so the
//! config serializes it as a TOML array; only a custom command falls
//! back to a shell string. The picker never *writes* the secret: it just
//! records the read command, leaving the value for the user to store
//! under the chosen entry beforehand.

use core::fmt;

use secrecy::SecretString;

use crate::prompt::{self, PromptResult};

/// A well-known credential-provider CLI a password can be read from.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KeyringProvider {
    /// `secret-tool`, over the Secret Service (GNOME Keyring).
    SecretTool,
    /// `kwallet-query`, over the KDE Wallet.
    KwalletQuery,
    /// `security`, over the macOS Keychain.
    Security,
    /// `pass`, the standard unix password manager.
    Pass,
}

impl KeyringProvider {
    /// The providers relevant on the running OS, most native first.
    /// Empty on platforms without a known stdin-friendly provider
    /// (Windows), where the picker goes straight to a custom command.
    pub fn available() -> Vec<Self> {
        let mut providers = Vec::new();

        if cfg!(target_os = "linux") {
            providers.push(Self::SecretTool);
            providers.push(Self::KwalletQuery);
        }

        if cfg!(target_os = "macos") {
            providers.push(Self::Security);
        }

        if cfg!(unix) {
            providers.push(Self::Pass);
        }

        providers
    }

    /// Display name of the provider, for the pick-list labels.
    pub fn name(self) -> &'static str {
        match self {
            Self::SecretTool => "secret-tool (GNOME Keyring / Secret Service)",
            Self::KwalletQuery => "kwallet-query (KDE Wallet)",
            Self::Security => "security (macOS Keychain)",
            Self::Pass => "pass (password store)",
        }
    }

    /// The argv (program + arguments, no shell) printing the secret at
    /// `key` on stdout — the value of the `*.command` config field.
    ///
    /// `key` is used **verbatim** as the entry identifier: `service` is
    /// an optional namespace a *self-owning* broker (which stores and
    /// reads its own value, e.g. an OAuth token manager) adds — a path
    /// prefix for `pass`/`kwallet`, a distinct attribute for
    /// `secret-tool`/`security`. Pass `None` to read a pre-existing entry
    /// exactly as named.
    pub fn read_command(self, service: Option<&str>, key: &str) -> Vec<String> {
        match self {
            Self::SecretTool => match service {
                Some(service) => {
                    argv(["secret-tool", "lookup", "service", service, "account", key])
                }
                None => argv(["secret-tool", "lookup", "account", key]),
            },
            Self::KwalletQuery => {
                let entry = path(service, key);
                argv(["kwallet-query", "-r", &entry, "kdewallet"])
            }
            Self::Security => match service {
                Some(service) => argv([
                    "security",
                    "find-generic-password",
                    "-s",
                    service,
                    "-a",
                    key,
                    "-w",
                ]),
                None => argv(["security", "find-generic-password", "-a", key, "-w"]),
            },
            Self::Pass => {
                let entry = path(service, key);
                argv(["pass", "show", &entry])
            }
        }
    }

    /// The shell command line *persisting* a secret it receives on stdin
    /// — the write half of a store/read pair for a broker that owns the
    /// value (e.g. an OAuth token manager), as opposed to a pre-existing
    /// entry the user stores themselves. A shell line rather than an
    /// argv: the writes rely on shell features (`$(cat)` on macOS).
    pub fn write_command(self, service: Option<&str>, key: &str) -> String {
        match self {
            Self::SecretTool => match service {
                Some(service) => format!(
                    "secret-tool store --label {service}/{key} service {service} account {key}"
                ),
                None => format!("secret-tool store --label {key} account {key}"),
            },
            Self::KwalletQuery => format!("kwallet-query -w {} kdewallet", path(service, key)),
            // `security` takes the secret as an argument, not on stdin;
            // `$(cat)` bridges it, and `-U` overwrites an existing entry.
            Self::Security => match service {
                Some(service) => {
                    format!("security add-generic-password -U -s {service} -a {key} -w \"$(cat)\"")
                }
                None => format!("security add-generic-password -U -a {key} -w \"$(cat)\""),
            },
            Self::Pass => format!("pass insert -m -f {}", path(service, key)),
        }
    }
}

/// A well-known OAuth 2.0 token broker: an external CLI that owns the
/// account's refresh token and prints a *fresh* access token on stdout.
///
/// Himalaya (and the other Pimalaya tools) ship no OAuth flow of their
/// own; they call one of these on every connection. Reference
/// implementation ([`Ortie`](Self::Ortie)) first.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TokenBroker {
    /// `ortie`, the Pimalaya OAuth 2.0 token broker.
    Ortie,
    /// `pizauth`, an OAuth 2.0 token daemon.
    Pizauth,
    /// `oama`, the OAuth Anywhere Mail Agent.
    Oama,
}

impl TokenBroker {
    /// The known brokers, reference implementation first. Not
    /// OS-specific: all are cross-platform CLIs.
    pub fn available() -> Vec<Self> {
        vec![Self::Ortie, Self::Pizauth, Self::Oama]
    }

    /// Display name of the broker, for the pick-list labels.
    pub fn name(self) -> &'static str {
        match self {
            Self::Ortie => "ortie (Pimalaya OAuth 2.0 token broker)",
            Self::Pizauth => "pizauth (OAuth 2.0 token daemon)",
            Self::Oama => "oama (OAuth Anywhere Mail Agent)",
        }
    }

    /// The argv (program + arguments, no shell) printing a fresh access
    /// token for `account` on stdout — the value of the `*.command`
    /// config field.
    ///
    /// `account` is the broker's own account handle: a name for `ortie`
    /// and `pizauth`, the email address for `oama`. It defaults to the
    /// Himalaya account name; the user adjusts it to match the broker's
    /// configuration.
    pub fn read_command(self, account: &str) -> Vec<String> {
        match self {
            Self::Ortie => argv(["ortie", "token", "show", "-a", account]),
            Self::Pizauth => argv(["pizauth", "show", account]),
            Self::Oama => argv(["oama", "access", account]),
        }
    }
}

/// Collects `parts` into an owned argv vector.
fn argv<const N: usize>(parts: [&str; N]) -> Vec<String> {
    parts.iter().map(|part| part.to_string()).collect()
}

/// Renders a path-based entry: `key` alone, or `service/key` when a
/// namespace is given.
fn path(service: Option<&str>, key: &str) -> String {
    match service {
        Some(service) => format!("{service}/{key}"),
        None => key.to_owned(),
    }
}

/// A secret collected by the picker.
///
/// A known provider or broker yields an argv [`Command`](Self::Command)
/// (serialized as a TOML array); a user-typed command is a
/// [`Shell`](Self::Shell) line (serialized as a string), the fallback
/// form run through the platform shell.
pub enum SecretChoice {
    /// An argv command (program + arguments, no shell) whose stdout is
    /// the secret. The preferred form.
    Command(Vec<String>),
    /// A raw shell command line whose stdout is the secret — the
    /// fallback, run through the platform shell.
    Shell(String),
    /// The secret stored raw (plaintext) in the configuration.
    Raw(SecretString),
}

/// One entry in the secret pick list: a keyring provider, an OAuth
/// broker, a custom command, or a raw value.
enum Choice {
    Keyring(KeyringProvider),
    Broker(TokenBroker),
    Custom,
    Raw,
}

impl PartialEq for Choice {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Keyring(a), Self::Keyring(b)) => a == b,
            (Self::Broker(a), Self::Broker(b)) => a == b,
            (Self::Custom, Self::Custom) | (Self::Raw, Self::Raw) => true,
            _ => false,
        }
    }
}

impl Eq for Choice {}

impl fmt::Display for Choice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Keyring(provider) => f.write_str(provider.name()),
            Self::Broker(broker) => f.write_str(broker.name()),
            Self::Custom => f.write_str("Custom shell command"),
            Self::Raw => f.write_str("Store raw in the configuration (plaintext, NOT recommended)"),
        }
    }
}

/// Prompts for a password: a pick list of the OS keyring providers, then
/// a custom command, then a raw value.
///
/// `key_default` seeds the entry prompt, and the chosen entry is used
/// verbatim: a pre-existing secret is read exactly as named.
pub fn prompt_secret(label: &str, key_default: &str) -> PromptResult<SecretChoice> {
    let mut choices: Vec<Choice> = KeyringProvider::available()
        .into_iter()
        .map(Choice::Keyring)
        .collect();
    choices.push(Choice::Custom);
    choices.push(Choice::Raw);

    prompt_choice(label, key_default, choices)
}

/// Prompts for an API token: a pick list combining the OS keyrings (for
/// a token the user generated on the provider and stored themselves) and,
/// when `oauth` is true, the OAuth 2.0 token brokers (which refresh and
/// print a fresh token on every read), then a custom command and a raw
/// value. Same aim as [`prompt_secret`] — a command that returns the
/// token — merging both acquisition paths behind one strategy prompt. The
/// brokers are hidden unless the service advertises OAuth.
pub fn prompt_token(label: &str, key_default: &str, oauth: bool) -> PromptResult<SecretChoice> {
    let mut choices: Vec<Choice> = KeyringProvider::available()
        .into_iter()
        .map(Choice::Keyring)
        .collect();
    if oauth {
        choices.extend(TokenBroker::available().into_iter().map(Choice::Broker));
    }
    choices.push(Choice::Custom);
    choices.push(Choice::Raw);

    prompt_choice(label, key_default, choices)
}

/// Renders the pick list and resolves the selection into a
/// [`SecretChoice`].
///
/// `label` names the secret ("IMAP password", "API token") and
/// `key_default` seeds the entry/account prompt. A keyring entry is used
/// **verbatim** (no namespace), so a pre-existing secret is read exactly
/// as named; the value must already be stored under it, and a missing one
/// surfaces when the caller tests the account right after.
fn prompt_choice(
    label: &str,
    key_default: &str,
    choices: Vec<Choice>,
) -> PromptResult<SecretChoice> {
    match prompt::item(format!("{label} strategy:"), choices, None)? {
        Choice::Keyring(provider) => {
            let key = prompt::text(
                format!("{label} keyring entry:"),
                Some(key_default.to_owned()),
            )?;

            Ok(SecretChoice::Command(provider.read_command(None, &key)))
        }
        Choice::Broker(broker) => {
            let account = prompt::text(format!("{label} account:"), Some(key_default.to_owned()))?;

            Ok(SecretChoice::Command(broker.read_command(&account)))
        }
        Choice::Custom => {
            let command = prompt::text(format!("{label} shell command:"), None::<String>)?;
            Ok(SecretChoice::Shell(command))
        }
        Choice::Raw => {
            let secret = prompt::password(format!("{label}:"), format!("Confirm {label}:"))?;
            Ok(SecretChoice::Raw(secret))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keyring_read_command_uses_the_entry_verbatim_without_a_namespace() {
        let entry = "pimalaya/posteo";
        assert_eq!(
            KeyringProvider::Pass.read_command(None, entry),
            ["pass", "show", "pimalaya/posteo"],
        );
        assert_eq!(
            KeyringProvider::SecretTool.read_command(None, entry),
            ["secret-tool", "lookup", "account", "pimalaya/posteo"],
        );
        assert_eq!(
            KeyringProvider::Security.read_command(None, entry),
            [
                "security",
                "find-generic-password",
                "-a",
                "pimalaya/posteo",
                "-w"
            ],
        );
        assert_eq!(
            KeyringProvider::KwalletQuery.read_command(None, entry),
            ["kwallet-query", "-r", "pimalaya/posteo", "kdewallet"],
        );
    }

    #[test]
    fn keyring_read_command_namespaces_the_entry_when_a_service_is_given() {
        let (service, account) = (Some("ortie"), "acme");
        assert_eq!(
            KeyringProvider::Pass.read_command(service, account),
            ["pass", "show", "ortie/acme"],
        );
        assert_eq!(
            KeyringProvider::SecretTool.read_command(service, account),
            [
                "secret-tool",
                "lookup",
                "service",
                "ortie",
                "account",
                "acme"
            ],
        );
        assert_eq!(
            KeyringProvider::Security.read_command(service, account),
            [
                "security",
                "find-generic-password",
                "-s",
                "ortie",
                "-a",
                "acme",
                "-w"
            ],
        );
    }

    #[test]
    fn broker_read_command_targets_the_account_per_broker() {
        assert_eq!(
            TokenBroker::Ortie.read_command("acme"),
            ["ortie", "token", "show", "-a", "acme"],
        );
        assert_eq!(
            TokenBroker::Pizauth.read_command("acme"),
            ["pizauth", "show", "acme"]
        );
        assert_eq!(
            TokenBroker::Oama.read_command("me@acme.test"),
            ["oama", "access", "me@acme.test"],
        );
    }

    #[test]
    fn available_lists_are_non_empty() {
        assert!(!TokenBroker::available().is_empty());
        if cfg!(unix) {
            assert!(!KeyringProvider::available().is_empty());
        }
    }
}
