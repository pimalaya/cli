use core::fmt;

use secrecy::SecretString;

#[derive(Clone, Debug)]
pub struct ImapConfig {
    pub host: String,
    pub port: u16,
    pub encryption: Encryption,
    pub login: String,
    pub auth: ImapAuth,
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
pub enum ImapAuth {
    Password(ImapSecret),
}

#[derive(Clone, Debug)]
pub enum ImapSecret {
    Raw(SecretString),
    Command(String),
}
