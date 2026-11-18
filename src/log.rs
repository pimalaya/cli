//! Logger initialization from the shared log flags.

use std::{fs::File, path::Path};

use anyhow::{Context, Result};
use env_logger::{Builder, Target};
use log::LevelFilter;

use crate::{clap::args::LogFlags, file::private_options};

/// Initializes the process logger from the shared log flags.
pub struct Logger;

impl Logger {
    /// Initialises `env_logger` from `log`. When [`LogFlags::file`]
    /// is set the log target is opened in append mode (creating it
    /// if missing, readable by its owner alone on Unix, and never
    /// through a symbolic link) and used for output; otherwise logs go
    /// to stderr as usual.
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
            let file = open_log_file(path)?;
            builder.target(Target::Pipe(Box::new(file)));
        }

        builder.try_init()?;
        Ok(())
    }
}

/// Opens the log file at `path` in append mode, creating it private if
/// missing ([`private_options`]), and names it when it cannot.
fn open_log_file(path: &Path) -> Result<File> {
    private_options()
        .create(true)
        .append(true)
        .open(path)
        .with_context(|| format!("Cannot open log file {}", path.display()))
}

#[cfg(all(test, unix))]
mod tests {
    use std::{
        env, fs,
        os::unix::fs::{PermissionsExt, symlink},
        path::PathBuf,
        process,
    };

    use crate::log::open_log_file;

    fn scratch(name: &str) -> PathBuf {
        let dir = env::temp_dir().join(format!("pimalaya-cli-log-{name}-{}", process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn a_new_log_file_is_private() {
        let dir = scratch("private");
        let path = dir.join("trace.log");

        open_log_file(&path).unwrap();
        let mode = fs::metadata(&path).unwrap().permissions().mode();
        fs::remove_dir_all(&dir).unwrap();

        assert_eq!(mode & 0o777, 0o600);
    }

    #[test]
    fn a_log_file_never_follows_a_symlink() {
        let dir = scratch("symlink");
        let target = dir.join("target");
        let link = dir.join("trace.log");
        symlink(&target, &link).unwrap();

        let err = open_log_file(&link).unwrap_err();
        let created = target.exists();
        fs::remove_dir_all(&dir).unwrap();

        assert!(!created);
        assert!(err.to_string().contains("trace.log"), "{err}");
    }

    #[test]
    fn an_unopenable_log_file_is_named() {
        let dir = scratch("missing");
        let path = dir.join("missing").join("trace.log");

        let err = open_log_file(&path).unwrap_err();
        fs::remove_dir_all(&dir).unwrap();

        assert!(err.to_string().contains(&*path.to_string_lossy()), "{err}");
    }
}
