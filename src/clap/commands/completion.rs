use std::{fmt, fs, path::PathBuf};

use anyhow::Result;
use clap::{value_parser, Command, Parser};
use clap_complete::Shell;
use log::debug;
use serde::{Serialize, Serializer};

use crate::{clap::parsers::path_parser, printer::Printer};

/// Generate completion script for the give shell(s) to the given
/// directory.
///
/// This command allows you to generate completion script for a given
/// shell. The script is printed to the standard output. If you want
/// to write it to a file, just use unix redirection.
#[derive(Debug, Parser)]
pub struct CompletionCommand {
    /// Shell for which completion script should be generated for.
    #[arg(value_parser = value_parser!(Shell))]
    pub shells: Vec<Shell>,

    /// Save completion script to the given directory.
    #[arg(short, long, value_name = "PATH", value_parser = path_parser, default_value = "./")]
    pub dir: PathBuf,
}

impl CompletionCommand {
    pub fn execute(self, printer: &mut impl Printer, mut command: Command) -> Result<()> {
        let dir = self.dir.canonicalize().unwrap_or(self.dir);
        fs::create_dir_all(&dir)?;

        let cmd_name = command.get_name().to_string();
        let mut scripts = Vec::with_capacity(5);

        for shell in self.shells {
            let path = clap_complete::generate_to(shell.clone(), &mut command, &cmd_name, &dir)?;
            let path = path.canonicalize().unwrap_or(path);
            debug!("generated {shell} completion script at {}", path.display());
            scripts.push(Script { shell, path })
        }

        printer.out(Completions { dir, scripts })
    }
}

/// Defines a struct-wrapper to provide a JSON output.
#[derive(Serialize)]
struct Completions {
    dir: PathBuf,
    scripts: Vec<Script>,
}

impl fmt::Display for Completions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n = self.scripts.len();
        let p = self.dir.display();
        writeln!(f, "{n} completion script(s) successfully generated in {p}:")?;

        for Script { shell, path } in &self.scripts {
            let p = path.display();
            writeln!(f, " - {shell} completion script at {p}")?;
        }

        Ok(())
    }
}

/// Defines a struct-wrapper to provide a JSON output.
#[derive(Serialize)]
struct Script {
    #[serde(serialize_with = "serialize_shell")]
    pub shell: Shell,
    pub path: PathBuf,
}

pub fn serialize_shell<S: Serializer>(shell: &Shell, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(&shell.to_string())
}
