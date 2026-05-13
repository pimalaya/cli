use core::fmt;

use secrecy::SecretString;

use crate::prompt::{self, PromptResult};

#[derive(Clone, Debug)]
pub struct WizardSmtpConfig {
    pub host: String,
    pub port: u16,
    pub encryption: Encryption,
    pub login: String,
    pub auth: SmtpAuth,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Encryption {
    #[default]
    Tls,
    StartTls,
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

#[derive(Clone, Debug)]
pub enum SmtpAuth {
    Password(SmtpSecret),
}

#[derive(Clone, Debug)]
pub enum SmtpSecret {
    Raw(SecretString),
    Command(String),
}

const ENCRYPTIONS: [Encryption; 3] = [Encryption::Tls, Encryption::StartTls, Encryption::None];

const CMD: &str = "Use a shell command to retrieve my password (recommended)";
const RAW: &str = "Save password in the configuration file (plaintext, NOT recommended)";

const SECRETS: [&str; 2] = [CMD, RAW];

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
        let strategy = prompt::item("SMTP authentication strategy:", SECRETS, None)?;
        let secret = match strategy {
            CMD => {
                let default_cmd = default_secret_cmd(account_name, "smtp");
                SmtpSecret::Command(prompt::text("Shell command:", Some(&default_cmd))?)
            }
            RAW => SmtpSecret::Raw(prompt::password("SMTP password:")?),
            _ => unreachable!(),
        };

        SmtpAuth::Password(secret)
    };

    Ok(WizardSmtpConfig {
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
        Encryption::Tls => 465,
        Encryption::StartTls => 587,
        Encryption::None => 25,
    }
}
