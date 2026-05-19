use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use log::LevelFilter;

use super::parsers::path_parser;

/// The account name flag parser.
#[derive(Debug, Default, Parser)]
pub struct AccountFlag {
    /// Override the default account.
    ///
    /// An account name corresponds to an entry in the table at the
    /// root level of your TOML configuration file.
    #[arg(long = "account", short = 'a', global = true)]
    #[arg(name = "account_name", value_name = "NAME")]
    pub name: Option<String>,
}

/// The account name flag parser.
#[derive(Debug, Default, Parser)]
pub struct AccountArg {
    /// Override the default account.
    ///
    /// An account name corresponds to an entry in the table at the
    /// root level of your TOML configuration file.
    #[arg(name = "account_name", value_name = "NAME")]
    pub name: String,
}

/// The JSON output flag parser.
#[derive(Debug, Default, Parser)]
pub struct JsonFlag {
    /// Enable JSON output.
    ///
    /// When set, command output (data or errors) is displayed as JSON
    /// string.
    #[arg(long = "json", name = "json", global = true)]
    pub enabled: bool,
}

/// The log level flag parser.
#[derive(Debug, Default, Parser)]
pub struct LogFlags {
    /// Filter log output by level.
    ///
    /// When omitted, the `RUST_LOG` environment variable is consulted
    /// (it supports per-target filters — see the `env_logger`
    /// documentation). When present, this flag overrides `RUST_LOG`
    /// entirely.
    #[arg(name = "log-level", long = "log-level", visible_alias = "log")]
    #[arg(value_enum, value_name = "LEVEL", global = true)]
    pub level: Option<LogLevel>,

    /// Append log output to the given file instead of stderr.
    ///
    /// Useful when interactive prompts (which always write to stderr)
    /// would otherwise be intermixed with log lines. The file is
    /// opened in append mode and created if it does not exist.
    #[arg(long = "log-file", global = true, name = "log-file")]
    #[arg(value_name = "PATH", value_parser = path_parser)]
    pub file: Option<PathBuf>,
}

/// Log level matching [`log::LevelFilter`].
#[derive(Debug, Default, Clone, Copy, ValueEnum)]
pub enum LogLevel {
    #[default]
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl From<LogLevel> for LevelFilter {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Off => Self::Off,
            LogLevel::Error => Self::Error,
            LogLevel::Warn => Self::Warn,
            LogLevel::Info => Self::Info,
            LogLevel::Debug => Self::Debug,
            LogLevel::Trace => Self::Trace,
        }
    }
}

#[macro_export]
macro_rules! long_version {
    () => {
        concat!(
            "v",
            env!("CARGO_PKG_VERSION"),
            " ",
            env!("CARGO_FEATURES"),
            "\nbuild: ",
            env!("CARGO_CFG_TARGET_OS"),
            " ",
            env!("CARGO_CFG_TARGET_ENV"),
            " ",
            env!("CARGO_CFG_TARGET_ARCH"),
            "\ngit: ",
            env!("GIT_DESCRIBE"),
            ", rev ",
            env!("GIT_REV"),
        )
    };
}
