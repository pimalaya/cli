use io_discovery::autoconfig::serde::{AutoConfig, SecurityType, ServerType};

use crate::{
    imap::{Encryption, ImapAuth, ImapConfig, ImapSecret},
    prompt::{self, PromptResult},
};

const ENCRYPTIONS: [Encryption; 3] = [Encryption::Tls, Encryption::StartTls, Encryption::None];

const CMD: &str = "Use a shell command to retrieve my password (recommended)";
const RAW: &str = "Save password in the configuration file (plaintext, NOT recommended)";

const SECRETS: [&str; 2] = [CMD, RAW];

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

pub fn start(
    account_name: &str,
    email_local_part: &str,
    email_domain: &str,
    autoconfig: Option<&AutoConfig>,
) -> PromptResult<ImapConfig> {
    let autoconfig_server = autoconfig.and_then(|c| {
        c.email_provider
            .incoming_servers()
            .find(|server| matches!(server.r#type, ServerType::Imap))
    });

    let autoconfig_host = autoconfig_server
        .and_then(|s| s.hostname())
        .map(ToOwned::to_owned);

    let default_host = autoconfig_host.unwrap_or_else(|| format!("imap.{email_domain}"));

    let host = prompt::text("IMAP hostname:", Some(&default_host))?;

    let autoconfig_encryption = autoconfig_server
        .and_then(|s| {
            s.security_type().map(|sec| match sec {
                SecurityType::Plain => Encryption::None,
                SecurityType::Starttls => Encryption::StartTls,
                SecurityType::Tls => Encryption::Tls,
            })
        })
        .unwrap_or_default();

    let autoconfig_port =
        autoconfig_server
            .and_then(|s| s.port())
            .unwrap_or_else(|| match autoconfig_encryption {
                Encryption::Tls => 993,
                Encryption::StartTls | Encryption::None => 143,
            });

    let encryption = prompt::item("IMAP encryption:", ENCRYPTIONS, Some(autoconfig_encryption))?;

    let default_port = if encryption == autoconfig_encryption {
        autoconfig_port
    } else {
        match encryption {
            Encryption::Tls => 993,
            Encryption::StartTls | Encryption::None => 143,
        }
    };

    let port = prompt::u16("IMAP port:", Some(default_port))?;

    let autoconfig_login = autoconfig_server
        .and_then(|s| s.username())
        .map(|u| match u {
            "%EMAILLOCALPART%" => email_local_part.to_owned(),
            "%EMAILADDRESS%" => format!("{email_local_part}@{email_domain}"),
            other => other.to_owned(),
        });

    let default_login =
        autoconfig_login.unwrap_or_else(|| format!("{email_local_part}@{email_domain}"));

    let login = prompt::text("IMAP login:", Some(&default_login))?;

    let auth = {
        let strategy = prompt::item("IMAP authentication strategy:", SECRETS, None)?;
        let secret = match strategy {
            CMD => {
                let default_cmd = default_secret_cmd(account_name, "imap");
                ImapSecret::Command(prompt::text("Shell command:", Some(&default_cmd))?)
            }
            RAW => ImapSecret::Raw(prompt::password("IMAP password:")?),
            _ => unreachable!(),
        };

        ImapAuth::Password(secret)
    };

    Ok(ImapConfig {
        host,
        port,
        encryption,
        login,
        auth,
    })
}
