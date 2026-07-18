//! OS-aware credential-provider picker shared by the account wizards.
//!
//! A secret (password or API token) is read from a well-known keyring
//! CLI known for the running OS, from a custom shell command, or stored
//! raw in the configuration. The picker never *writes* the secret: it
//! only records the read command, leaving the value for the user to
//! store under the chosen entry beforehand.

#[cfg(any(feature = "imap", feature = "smtp", feature = "jmap"))]
use core::fmt;

#[cfg(any(feature = "imap", feature = "smtp", feature = "jmap"))]
use secrecy::SecretString;

#[cfg(any(feature = "imap", feature = "smtp", feature = "jmap"))]
use crate::prompt::{self, PromptResult};

/// A well-known credential-provider CLI a secret can be read from.
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

    /// The shell command line printing the secret at `key` on stdout —
    /// the value of the `*.command` config field.
    ///
    /// `key` is used **verbatim** as the entry identifier: `service` is
    /// an optional namespace a *self-owning* broker (which stores and
    /// reads its own value, e.g. an OAuth token manager) adds — a path
    /// prefix for `pass`/`kwallet`, a distinct attribute for
    /// `secret-tool`/`security`. Pass `None` to read a pre-existing entry
    /// exactly as named.
    pub fn read_command(self, service: Option<&str>, key: &str) -> String {
        match self {
            Self::SecretTool => match service {
                Some(service) => format!("secret-tool lookup service {service} account {key}"),
                None => format!("secret-tool lookup account {key}"),
            },
            Self::KwalletQuery => format!("kwallet-query -r {} kdewallet", path(service, key)),
            Self::Security => match service {
                Some(service) => format!("security find-generic-password -s {service} -a {key} -w"),
                None => format!("security find-generic-password -a {key} -w"),
            },
            Self::Pass => format!("pass show {}", path(service, key)),
        }
    }

    /// The command *persisting* a secret it receives on stdin — the write
    /// half of a store/read pair for a broker that owns the value (e.g.
    /// an OAuth token manager), as opposed to a pre-existing entry the
    /// user stores themselves.
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

/// Renders a path-based entry: `key` alone, or `service/key` when a
/// namespace is given.
fn path(service: Option<&str>, key: &str) -> String {
    match service {
        Some(service) => format!("{service}/{key}"),
        None => key.to_owned(),
    }
}

/// A secret collected by the picker: a shell command whose stdout is the
/// secret, or the raw value stored in the configuration.
#[cfg(any(feature = "imap", feature = "smtp", feature = "jmap"))]
pub enum SecretChoice {
    /// A shell command whose stdout is the secret.
    Command(String),
    /// The secret stored raw (plaintext) in the configuration.
    Raw(SecretString),
}

/// One entry in the secret pick list: a product-specific extra option, a
/// known keyring provider, a custom command, or a raw value.
#[cfg(any(feature = "imap", feature = "smtp", feature = "jmap"))]
enum Choice {
    Extra { label: String, command: String },
    Keyring(KeyringProvider),
    Custom,
    Raw,
}

#[cfg(any(feature = "imap", feature = "smtp", feature = "jmap"))]
impl PartialEq for Choice {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Extra { label: a, .. }, Self::Extra { label: b, .. }) => a == b,
            (Self::Keyring(a), Self::Keyring(b)) => a == b,
            (Self::Custom, Self::Custom) | (Self::Raw, Self::Raw) => true,
            _ => false,
        }
    }
}

#[cfg(any(feature = "imap", feature = "smtp", feature = "jmap"))]
impl Eq for Choice {}

#[cfg(any(feature = "imap", feature = "smtp", feature = "jmap"))]
impl fmt::Display for Choice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Extra { label, .. } => f.write_str(label),
            Self::Keyring(provider) => f.write_str(provider.name()),
            Self::Custom => f.write_str("Custom shell command"),
            Self::Raw => f.write_str("Store raw in the configuration (plaintext, NOT recommended)"),
        }
    }
}

/// Prompts for a secret: a pick list of product-specific `extra` options
/// (e.g. an OAuth broker command), then the OS keyring providers, then a
/// custom command, then a raw value.
///
/// `label` names the secret ("IMAP password") and `key_default` seeds
/// the entry prompt (typically `<account>-<protocol>`). The keyring entry
/// the user gives is used **verbatim** (no namespace), so a pre-existing
/// secret is read exactly as named. The value must already be stored
/// under that entry; a missing one surfaces when the caller tests the
/// account right after.
#[cfg(any(feature = "imap", feature = "smtp", feature = "jmap"))]
pub fn prompt_secret(
    label: &str,
    key_default: &str,
    extra: &[(&str, String)],
) -> PromptResult<SecretChoice> {
    let mut choices: Vec<Choice> = extra
        .iter()
        .map(|(label, command)| Choice::Extra {
            label: (*label).to_owned(),
            command: command.clone(),
        })
        .collect();

    choices.extend(
        KeyringProvider::available()
            .into_iter()
            .map(Choice::Keyring),
    );
    choices.push(Choice::Custom);
    choices.push(Choice::Raw);

    match prompt::item(format!("{label} strategy:"), choices, None)? {
        Choice::Extra { command, .. } => Ok(SecretChoice::Command(command)),
        Choice::Keyring(provider) => {
            let key = prompt::text(
                format!("{label} keyring entry:"),
                Some(key_default.to_owned()),
            )?;

            Ok(SecretChoice::Command(provider.read_command(None, &key)))
        }
        Choice::Custom => {
            let command = prompt::text(format!("{label} shell command:"), None::<String>)?;
            Ok(SecretChoice::Command(command))
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
    fn read_command_uses_the_entry_verbatim_without_a_namespace() {
        let entry = "pimalaya/posteo";
        assert_eq!(
            KeyringProvider::Pass.read_command(None, entry),
            "pass show pimalaya/posteo",
        );
        assert_eq!(
            KeyringProvider::SecretTool.read_command(None, entry),
            "secret-tool lookup account pimalaya/posteo",
        );
        assert_eq!(
            KeyringProvider::Security.read_command(None, entry),
            "security find-generic-password -a pimalaya/posteo -w",
        );
        assert_eq!(
            KeyringProvider::KwalletQuery.read_command(None, entry),
            "kwallet-query -r pimalaya/posteo kdewallet",
        );
    }

    #[test]
    fn read_command_namespaces_the_entry_when_a_service_is_given() {
        let (service, account) = (Some("ortie"), "acme");
        assert_eq!(
            KeyringProvider::Pass.read_command(service, account),
            "pass show ortie/acme",
        );
        assert_eq!(
            KeyringProvider::SecretTool.read_command(service, account),
            "secret-tool lookup service ortie account acme",
        );
        assert_eq!(
            KeyringProvider::Security.read_command(service, account),
            "security find-generic-password -s ortie -a acme -w",
        );
    }

    #[test]
    fn available_is_non_empty_on_unix() {
        if cfg!(unix) {
            assert!(!KeyringProvider::available().is_empty());
        }
    }
}
