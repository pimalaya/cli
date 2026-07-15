//! Interactive IMAP account setup wizard.

use core::fmt;

use secrecy::SecretString;

use crate::prompt::{self, PromptResult};

/// IMAP account settings collected by the wizard.
#[derive(Clone, Debug)]
pub struct WizardImapConfig {
    /// The IMAP server hostname.
    pub host: String,
    /// The IMAP server port.
    pub port: u16,
    /// The connection encryption scheme.
    pub encryption: Encryption,
    /// The login (username) sent during authentication.
    pub login: String,
    /// The authentication method and its secret.
    pub auth: ImapAuth,
}

/// Connection encryption scheme offered by the wizard.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Encryption {
    /// Implicit TLS negotiated on connection (the default).
    #[default]
    Tls,
    /// Opportunistic upgrade to TLS through STARTTLS.
    StartTls,
    /// No encryption (insecure).
    None,
}

impl fmt::Display for Encryption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Tls => f.write_str("Always (TLS)"),
            Self::StartTls => f.write_str("Opportunistic (STARTTLS)"),
            Self::None => f.write_str("None (insecure)"),
        }
    }
}

/// IMAP authentication method.
#[derive(Clone, Debug)]
pub enum ImapAuth {
    /// Password authentication carrying the password secret.
    Password(ImapSecret),
}

/// Source of an IMAP password.
#[derive(Clone, Debug)]
pub enum ImapSecret {
    /// The password stored in plaintext in the configuration.
    Raw(SecretString),
    /// A shell command whose output is the password.
    Command(String),
}

const ENCRYPTIONS: [Encryption; 3] = [Encryption::Tls, Encryption::StartTls, Encryption::None];

const CMD: &str = "Use a shell command to retrieve my password (recommended)";
const RAW: &str = "Save password in the configuration file (plaintext, NOT recommended)";

const SECRETS: [&str; 2] = [CMD, RAW];

/// Runs the interactive IMAP account wizard, returning the collected
/// settings.
pub fn run(
    account_name: impl AsRef<str>,
    local_part: impl AsRef<str>,
    domain: impl AsRef<str>,
    defaults: Option<&WizardImapConfig>,
) -> PromptResult<WizardImapConfig> {
    let account_name = account_name.as_ref();
    let local_part = local_part.as_ref();
    let domain = domain.as_ref();

    let default_host = defaults
        .map(|c| c.host.clone())
        .unwrap_or_else(|| format!("imap.{domain}"));

    let host = prompt::text("IMAP hostname:", Some(&default_host))?;

    let default_encryption = defaults.map(|c| c.encryption).unwrap_or_default();

    let encryption = prompt::item("IMAP encryption:", ENCRYPTIONS, Some(default_encryption))?;

    let default_port = if encryption == default_encryption {
        defaults
            .map(|c| c.port)
            .unwrap_or_else(|| default_port(encryption))
    } else {
        default_port(encryption)
    };

    let port = prompt::u16("IMAP port:", Some(default_port))?;

    let default_login = defaults
        .map(|c| c.login.clone())
        .unwrap_or_else(|| format!("{local_part}@{domain}"));

    let login = prompt::text("IMAP login:", Some(&default_login))?;

    let auth = {
        let strategy = prompt::item("IMAP authentication strategy:", SECRETS, None)?;
        let secret = match strategy {
            CMD => {
                let default_cmd = default_secret_cmd(account_name, "imap");
                ImapSecret::Command(prompt::text("Shell command:", Some(&default_cmd))?)
            }
            RAW => ImapSecret::Raw(prompt::password(
                "IMAP password:",
                "Confirm IMAP password:",
            )?),
            _ => unreachable!(),
        };

        ImapAuth::Password(secret)
    };

    Ok(WizardImapConfig {
        host,
        port,
        encryption,
        login,
        auth,
    })
}

fn default_secret_cmd(account_name: &str, protocol: &str) -> String {
    if cfg!(target_os = "macos") {
        format!(
            "security find-generic-password \
	     -a '{account_name}' \
	     -s 'himalaya-{account_name}-{protocol}' \
	     -w"
        )
    } else if cfg!(target_os = "linux") {
        format!("secret-tool lookup account {account_name} service himalaya-{protocol}")
    } else {
        String::new()
    }
}

fn default_port(encryption: Encryption) -> u16 {
    match encryption {
        Encryption::Tls => 993,
        Encryption::StartTls | Encryption::None => 143,
    }
}
