//! Interactive CardDAV account setup wizard.

use secrecy::SecretString;

use crate::prompt::{self, PromptResult};

/// Context and prompt defaults for the CardDAV wizard.
///
/// Doubles as the wizard's return value: `account_name` / `project_name`
/// seed the default secret command and are carried back untouched, and
/// `email` (when set) seeds the username default. The wizard collects
/// authentication only; the server is resolved by discovery, not
/// prompted.
#[derive(Clone, Debug, Default)]
pub struct WizardCarddavConfig {
    /// Account name, used to seed the default secret command.
    pub account_name: String,
    /// Project (binary) name, used to seed the default secret command.
    pub project_name: String,
    /// Account email, used as the username default when set.
    pub email: Option<String>,
    /// The authentication method and its secret.
    pub auth: CarddavAuth,
    /// When set, drop the Basic option from the authentication-strategy
    /// prompt (e.g. Google, which only accepts OAuth 2.0 tokens).
    pub bearer_only: bool,
}

/// CardDAV authentication mechanism collected by the wizard.
#[derive(Clone, Debug)]
pub enum CarddavAuth {
    /// HTTP Basic authentication with a username and a password secret.
    Basic {
        /// The username sent during authentication.
        username: String,
        /// The password secret.
        secret: CarddavSecret,
    },
    /// HTTP Bearer authentication with a token secret.
    Bearer {
        /// The token secret.
        secret: CarddavSecret,
    },
}

impl Default for CarddavAuth {
    fn default() -> Self {
        Self::Basic {
            username: String::new(),
            secret: CarddavSecret::default(),
        }
    }
}

/// Secret source collected by the wizard: an inline value or a shell
/// command line.
#[derive(Clone, Debug)]
pub enum CarddavSecret {
    /// The secret stored in plaintext in the configuration.
    Raw(SecretString),
    /// A shell command whose output is the secret.
    Command(String),
}

impl Default for CarddavSecret {
    fn default() -> Self {
        Self::Command(String::new())
    }
}

const CMD: &str = "Use a shell command to retrieve my secret (recommended)";
const RAW: &str = "Save secret in the configuration file (plaintext, NOT recommended)";
const SECRETS: [&str; 2] = [CMD, RAW];

const BASIC: &str = "Basic (username + password)";
const BEARER: &str = "Bearer (token)";
const AUTHS: [&str; 2] = [BASIC, BEARER];

/// Runs the interactive CardDAV account wizard, returning the collected
/// settings.
pub fn run(defaults: &WizardCarddavConfig) -> PromptResult<WizardCarddavConfig> {
    // NOTE: Bearer-only providers (e.g. Google) still get the strategy prompt,
    // just without the Basic option.
    let strategies: &[&str] = if defaults.bearer_only {
        &[BEARER]
    } else {
        &AUTHS
    };

    let default_strategy = match &defaults.auth {
        CarddavAuth::Basic { .. } => BASIC,
        CarddavAuth::Bearer { .. } => BEARER,
    };

    let strategy = prompt::item(
        "CardDAV authentication strategy:",
        strategies.iter().copied(),
        Some(default_strategy),
    )?;

    let auth = match strategy {
        BASIC => {
            let default_username = match &defaults.auth {
                CarddavAuth::Basic { username, .. } if !username.is_empty() => {
                    Some(username.clone())
                }
                _ => defaults.email.clone(),
            };

            let username = prompt::text("CardDAV username:", default_username.as_deref())?;
            let secret = prompt_secret(
                &defaults.account_name,
                &defaults.project_name,
                "password",
                auth_secret(&defaults.auth),
            )?;

            CarddavAuth::Basic { username, secret }
        }
        BEARER => {
            let secret = prompt_secret(
                &defaults.account_name,
                &defaults.project_name,
                "token",
                auth_secret(&defaults.auth),
            )?;
            CarddavAuth::Bearer { secret }
        }
        _ => unreachable!(),
    };

    Ok(WizardCarddavConfig {
        account_name: defaults.account_name.clone(),
        project_name: defaults.project_name.clone(),
        email: defaults.email.clone(),
        auth,
        bearer_only: defaults.bearer_only,
    })
}

/// The secret carried by either auth variant, used to seed the secret
/// strategy and command defaults when editing.
fn auth_secret(auth: &CarddavAuth) -> &CarddavSecret {
    match auth {
        CarddavAuth::Basic { secret, .. } => secret,
        CarddavAuth::Bearer { secret } => secret,
    }
}

fn prompt_secret(
    account_name: &str,
    project_name: &str,
    label: &str,
    default: &CarddavSecret,
) -> PromptResult<CarddavSecret> {
    let default_strategy = match default {
        CarddavSecret::Command(_) => CMD,
        CarddavSecret::Raw(_) => RAW,
    };

    let strategy = prompt::item("CardDAV secret strategy:", SECRETS, Some(default_strategy))?;

    match strategy {
        CMD => {
            let default_cmd = match default {
                CarddavSecret::Command(cmd) if !cmd.is_empty() => cmd.clone(),
                _ => default_secret_cmd(account_name, project_name, "carddav"),
            };
            let cmd = prompt::text("Shell command:", Some(&default_cmd))?;
            Ok(CarddavSecret::Command(cmd))
        }
        RAW => {
            let secret = prompt::password(
                format!("CardDAV {label}:"),
                format!("Confirm CardDAV {label}:"),
            )?;
            Ok(CarddavSecret::Raw(secret))
        }
        _ => unreachable!(),
    }
}

fn default_secret_cmd(account_name: &str, project_name: &str, protocol: &str) -> String {
    if cfg!(target_os = "macos") {
        format!(
            "security find-generic-password \
             -a '{account_name}' \
             -s '{project_name}-{account_name}-{protocol}' \
             -w"
        )
    } else if cfg!(target_os = "linux") {
        format!("secret-tool lookup account {account_name} service {project_name}-{protocol}")
    } else {
        String::new()
    }
}
