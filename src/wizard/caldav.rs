use core::fmt;

use secrecy::SecretString;

use crate::prompt::{self, PromptResult};

#[derive(Clone, Debug)]
pub struct WizardCaldavConfig {
    pub host: String,
    pub port: u16,
    pub encryption: Encryption,
    /// Optional path override on the discovered server (used when the
    /// admin published the calendar home-set at a non-default URL).
    pub home_url: Option<String>,
    pub auth: CaldavAuth,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Encryption {
    #[default]
    Tls,
    None,
}

impl fmt::Display for Encryption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Tls => f.write_str("Always (TLS)"),
            Self::None => f.write_str("None (insecure)"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum CaldavAuth {
    Basic {
        username: String,
        secret: CaldavSecret,
    },
    Bearer {
        secret: CaldavSecret,
    },
}

#[derive(Clone, Debug)]
pub enum CaldavSecret {
    Raw(SecretString),
    Command(String),
}

const ENCRYPTIONS: [Encryption; 2] = [Encryption::Tls, Encryption::None];

const CMD: &str = "Use a shell command to retrieve my secret (recommended)";
const RAW: &str = "Save secret in the configuration file (plaintext, NOT recommended)";
const SECRETS: [&str; 2] = [CMD, RAW];

const BASIC: &str = "HTTP Basic (username + password)";
const BEARER: &str = "HTTP Bearer (token)";
const AUTHS: [&str; 2] = [BASIC, BEARER];

pub fn run(
    account_name: impl AsRef<str>,
    local_part: impl AsRef<str>,
    domain: impl AsRef<str>,
    defaults: Option<&WizardCaldavConfig>,
) -> PromptResult<WizardCaldavConfig> {
    let account_name = account_name.as_ref();
    let local_part = local_part.as_ref();
    let domain = domain.as_ref();

    let default_host = defaults
        .map(|c| c.host.clone())
        .unwrap_or_else(|| format!("caldav.{domain}"));

    let host = prompt::text("CalDAV hostname:", Some(&default_host))?;

    let default_encryption = defaults.map(|c| c.encryption).unwrap_or_default();

    let encryption = prompt::item("CalDAV encryption:", ENCRYPTIONS, Some(default_encryption))?;

    let default_port = if encryption == default_encryption {
        defaults
            .map(|c| c.port)
            .unwrap_or_else(|| default_port(encryption))
    } else {
        default_port(encryption)
    };

    let port = prompt::u16("CalDAV port:", Some(default_port))?;

    let default_home_url = defaults
        .and_then(|c| c.home_url.clone())
        .unwrap_or_default();

    let home_url = prompt::text(
        "CalDAV home URL (leave blank to auto-discover):",
        Some(&default_home_url),
    )?;

    let home_url = if home_url.trim().is_empty() {
        None
    } else {
        Some(home_url)
    };

    let default_strategy = match defaults.map(|c| &c.auth) {
        Some(CaldavAuth::Basic { .. }) => Some(BASIC),
        Some(CaldavAuth::Bearer { .. }) => Some(BEARER),
        None => None,
    };

    let strategy = prompt::item("CalDAV authentication strategy:", AUTHS, default_strategy)?;

    let auth = match strategy {
        BASIC => {
            let default_username = defaults
                .and_then(|c| match &c.auth {
                    CaldavAuth::Basic { username, .. } if !username.is_empty() => {
                        Some(username.clone())
                    }
                    _ => None,
                })
                .unwrap_or_else(|| format!("{local_part}@{domain}"));

            let username = prompt::text("CalDAV username:", Some(&default_username))?;
            let secret = prompt_secret(account_name, "password")?;

            CaldavAuth::Basic { username, secret }
        }
        BEARER => {
            let secret = prompt_secret(account_name, "token")?;
            CaldavAuth::Bearer { secret }
        }
        _ => unreachable!(),
    };

    Ok(WizardCaldavConfig {
        host,
        port,
        encryption,
        home_url,
        auth,
    })
}

fn prompt_secret(account_name: &str, label: &str) -> PromptResult<CaldavSecret> {
    let strategy = prompt::item("CalDAV secret strategy:", SECRETS, None)?;

    match strategy {
        CMD => {
            let default_cmd = default_secret_cmd(account_name, "caldav");
            let cmd = prompt::text("Shell command:", Some(&default_cmd))?;
            Ok(CaldavSecret::Command(cmd))
        }
        RAW => {
            let secret = prompt::password(
                format!("CalDAV {label}:"),
                format!("Confirm CalDAV {label}:"),
            )?;
            Ok(CaldavSecret::Raw(secret))
        }
        _ => unreachable!(),
    }
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
        Encryption::Tls => 443,
        Encryption::None => 80,
    }
}
