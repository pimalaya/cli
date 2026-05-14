use secrecy::SecretString;

use crate::prompt::{self, PromptResult};

#[derive(Clone, Debug)]
pub struct WizardJmapConfig {
    pub server: String,
    pub auth: JmapAuth,
}

#[derive(Clone, Debug)]
pub enum JmapAuth {
    Basic { login: String, secret: JmapSecret },
    Bearer { secret: JmapSecret },
}

#[derive(Clone, Debug)]
pub enum JmapSecret {
    Raw(SecretString),
    Command(String),
}

const BASIC: &str = "Basic (username + password)";
const BEARER: &str = "Bearer (OAuth access token)";
const AUTHS: [&str; 2] = [BASIC, BEARER];

const CMD: &str = "Use a shell command to retrieve my secret (recommended)";
const RAW: &str = "Save secret in the configuration file (plaintext, NOT recommended)";
const SECRETS: [&str; 2] = [CMD, RAW];

pub fn run(
    account_name: impl AsRef<str>,
    local_part: impl AsRef<str>,
    domain: impl AsRef<str>,
    defaults: Option<&WizardJmapConfig>,
) -> PromptResult<WizardJmapConfig> {
    let account_name = account_name.as_ref();
    let local_part = local_part.as_ref();
    let domain = domain.as_ref();

    let default_server = defaults
        .map(|c| c.server.clone())
        .unwrap_or_else(|| domain.to_string());

    let server = prompt::text(
        "JMAP server (bare authority or full URL):",
        Some(default_server.as_str()),
    )?;

    let default_strategy = match defaults.map(|c| &c.auth) {
        Some(JmapAuth::Basic { .. }) => Some(BASIC),
        Some(JmapAuth::Bearer { .. }) => Some(BEARER),
        None => None,
    };

    let strategy = prompt::item("JMAP authentication strategy:", AUTHS, default_strategy)?;

    let auth = match strategy {
        BASIC => {
            let default_login = defaults
                .and_then(|c| match &c.auth {
                    JmapAuth::Basic { login, .. } if !login.is_empty() => Some(login.clone()),
                    _ => None,
                })
                .unwrap_or_else(|| format!("{local_part}@{domain}"));

            let login = prompt::text("JMAP login:", Some(default_login.as_str()))?;
            let secret = prompt_secret(account_name, "password")?;

            JmapAuth::Basic { login, secret }
        }
        BEARER => {
            let secret = prompt_secret(account_name, "token")?;
            JmapAuth::Bearer { secret }
        }
        _ => unreachable!(),
    };

    Ok(WizardJmapConfig { server, auth })
}

fn prompt_secret(account_name: &str, label: &str) -> PromptResult<JmapSecret> {
    let strategy = prompt::item("JMAP secret strategy:", SECRETS, None)?;

    match strategy {
        CMD => {
            let default_cmd = default_secret_cmd(account_name);
            let cmd = prompt::text("Shell command:", Some(default_cmd.as_str()))?;
            Ok(JmapSecret::Command(cmd))
        }
        RAW => {
            let secret =
                prompt::password(format!("JMAP {label}:"), format!("Confirm JMAP {label}:"))?;
            Ok(JmapSecret::Raw(secret))
        }
        _ => unreachable!(),
    }
}

fn default_secret_cmd(account_name: &str) -> String {
    if cfg!(target_os = "macos") {
        format!(
            "security find-generic-password \
	     -a '{account_name}' \
	     -s 'himalaya-{account_name}-jmap' \
	     -w"
        )
    } else if cfg!(target_os = "linux") {
        format!("secret-tool lookup account {account_name} service himalaya-jmap")
    } else {
        String::new()
    }
}
