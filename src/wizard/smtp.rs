//! Interactive SMTP account setup wizard.

use core::fmt;

use secrecy::SecretString;

use crate::{
    prompt::{self, PromptResult},
    wizard::keyring::{self, SecretChoice},
};

/// SMTP account settings collected by the wizard.
#[derive(Clone, Debug)]
pub struct WizardSmtpConfig {
    /// The SMTP server hostname.
    pub host: String,
    /// The SMTP server port.
    pub port: u16,
    /// The connection encryption scheme.
    pub encryption: Encryption,
    /// The login (username) sent during authentication.
    pub login: String,
    /// The authentication method and its secret.
    pub auth: SmtpAuth,
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

/// SMTP authentication method.
#[derive(Clone, Debug)]
pub enum SmtpAuth {
    /// Password authentication carrying the password secret.
    Password(SmtpSecret),
}

/// Source of an SMTP password.
#[derive(Clone, Debug)]
pub enum SmtpSecret {
    /// The password stored in plaintext in the configuration.
    Raw(SecretString),
    /// A shell command whose output is the password.
    Command(String),
}

const ENCRYPTIONS: [Encryption; 3] = [Encryption::Tls, Encryption::StartTls, Encryption::None];

/// Runs the interactive SMTP account wizard, returning the collected
/// settings.
pub fn run(
    account_name: impl AsRef<str>,
    local_part: impl AsRef<str>,
    domain: impl AsRef<str>,
    defaults: Option<&WizardSmtpConfig>,
) -> PromptResult<WizardSmtpConfig> {
    let account_name = account_name.as_ref();
    let local_part = local_part.as_ref();
    let domain = domain.as_ref();

    let default_host = defaults
        .map(|c| c.host.clone())
        .unwrap_or_else(|| format!("smtp.{domain}"));

    let host = prompt::text("SMTP hostname:", Some(&default_host))?;

    let default_encryption = defaults.map(|c| c.encryption).unwrap_or_default();

    let encryption = prompt::item("SMTP encryption:", ENCRYPTIONS, Some(default_encryption))?;

    let default_port = if encryption == default_encryption {
        defaults
            .map(|c| c.port)
            .unwrap_or_else(|| default_port(encryption))
    } else {
        default_port(encryption)
    };

    let port = prompt::u16("SMTP port:", Some(default_port))?;

    let default_login = defaults
        .map(|c| c.login.clone())
        .unwrap_or_else(|| format!("{local_part}@{domain}"));

    let login = prompt::text("SMTP login:", Some(&default_login))?;

    let auth = {
        let key = format!("{account_name}-smtp");
        let secret = keyring::prompt_secret("SMTP password", &key, &[])?;
        SmtpAuth::Password(match secret {
            SecretChoice::Command(command) => SmtpSecret::Command(command),
            SecretChoice::Raw(secret) => SmtpSecret::Raw(secret),
        })
    };

    Ok(WizardSmtpConfig {
        host,
        port,
        encryption,
        login,
        auth,
    })
}

fn default_port(encryption: Encryption) -> u16 {
    match encryption {
        Encryption::Tls => 465,
        Encryption::StartTls => 587,
        Encryption::None => 25,
    }
}
