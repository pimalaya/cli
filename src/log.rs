use std::fs::OpenOptions;

use anyhow::Result;
use env_logger::{Builder, Target};

use crate::clap::args::LogFlags;

pub struct Logger;

impl Logger {
    /// Initialises [`env_logger`] from `log`. When [`LogFlags::file`]
    /// is set the log target is opened in append mode (creating it
    /// if missing) and used for output; otherwise logs go to stderr
    /// as usual.
    ///
    /// The file is intentionally opened with [`expect`] — if the
    /// caller asked for a log file but we cannot create or open it,
    /// the binary should refuse to run rather than silently fall back
    /// to stderr and pollute interactive prompts.
    pub fn try_init(log: &LogFlags) -> Result<()> {
        let mut builder = Builder::new();

        match log.level {
            Some(level) => {
                // Explicit `--log-level` overrides any `RUST_LOG`.
                builder.filter_level(level.into());
            }
            None => {
                // Defer to `RUST_LOG` (if unset, env_logger filters everything
                // — same as `--log-level off`).
                builder.parse_default_env();
            }
        }

        if let Some(path) = &log.file {
            let file = OpenOptions::new().create(true).append(true).open(path)?;
            builder.target(Target::Pipe(Box::new(file)));
        }

        builder.try_init()?;
        Ok(())
    }
}
