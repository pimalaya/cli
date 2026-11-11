//! Shared clap argument and flag parsers.

use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use log::LevelFilter;

use crate::clap::parsers::path_parser;

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

/// The config path flags parser, one path per flag.
#[derive(Debug, Default, Parser)]
pub struct ConfigFlags {
    /// Override the default configuration file path.
    ///
    /// Paths are shell-expanded, then canonicalized when applicable.
    /// Repeat the flag to merge several files: the first one is the base
    /// and the next ones are merged on top, which is how a public config
    /// and a private one combine.
    #[arg(long = "config", short = 'c', global = true)]
    #[arg(name = "config_paths", value_name = "PATH", value_parser = path_parser)]
    pub paths: Vec<PathBuf>,
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

/// The page token argument parser.
#[derive(Debug, Default, Parser)]
pub struct PageTokenArg {
    /// Resume a listing at the given page.
    ///
    /// The token is the `nextPage` value a previous listing printed,
    /// given back as it is.
    #[arg(long = "page-token", name = "page_token", value_name = "TOKEN")]
    pub token: Option<String>,
}

/// The log level flag parser.
#[derive(Debug, Default, Parser)]
pub struct LogFlags {
    /// Filter log output by level.
    ///
    /// When omitted, the `RUST_LOG` environment variable is consulted
    /// (it supports per-target filters, see the `env_logger`
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
    /// No logging.
    #[default]
    Off,
    /// Errors only.
    Error,
    /// Warnings and errors.
    Warn,
    /// Informational messages and above.
    Info,
    /// Debug messages and above.
    Debug,
    /// Trace messages and above (most verbose).
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

/// Builds the long version string (version, enabled features, build
/// target and git revision) for a binary's `--version` output.
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

/// Builds the --help footer, with link to bug tracker and sponsor.
#[macro_export]
macro_rules! footer {
    () => {
        concat!(
	    "This program is part of Pimalaya, free software funded entirely by grants and donations.\n",
            "If you find it useful, consider supporting its development:\n",
	    "\n",
	    "Report bugs to:\n",
	    "  ", env!("CARGO_PKG_REPOSITORY"), "/issues/\n",
	    "\n",
	    "Sponsoring:\n",
	    "  https://pimalaya.org/sponsor/\n",
        )
    };
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use clap::Parser;

    use crate::clap::args::ConfigFlags;

    #[test]
    fn a_config_path_is_never_split() {
        let args = ["app", "-c", r"C:\a.toml", "-c", "b:c;d.toml"];
        let flags = ConfigFlags::try_parse_from(args).unwrap();
        let paths = [PathBuf::from(r"C:\a.toml"), PathBuf::from("b:c;d.toml")];
        assert_eq!(flags.paths, paths);
    }
}
