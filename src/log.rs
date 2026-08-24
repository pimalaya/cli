//! Logger initialization from the shared log flags.

use std::fs::OpenOptions;

use anyhow::Result;
use env_logger::{Builder, Target};
use log::LevelFilter;

use crate::clap::args::LogFlags;

/// Initializes the process logger from the shared log flags.
pub struct Logger;

impl Logger {
    /// Initialises `env_logger` from `log`. When [`LogFlags::file`]
    /// is set the log target is opened in append mode (creating it
    /// if missing) and used for output; otherwise logs go to stderr
    /// as usual.
    ///
    /// Opening the log file is fallible: if the caller asked for one
    /// but we cannot create or open it, the error is returned rather
    /// than silently falling back to stderr and polluting interactive
    /// prompts.
    pub fn try_init(log: &LogFlags) -> Result<()> {
        let mut builder = Builder::new();

        match log.level {
            Some(level) => {
                builder.filter_level(level.into());
            }
            None => {
                builder.filter_level(LevelFilter::Off);
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
