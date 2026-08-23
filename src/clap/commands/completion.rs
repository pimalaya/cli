use std::{fmt, fs, path::PathBuf};

use anyhow::{Context, Result, bail};
use clap::{Command, Parser, value_parser};
use clap_complete::Shell;
use log::debug;
use serde::{Serialize, Serializer};

use crate::{clap::parsers::path_parser, printer::Printer};

/// Generate completion script for the given shell(s).
///
/// This command allows you to generate completion script for a given
/// shell. The script is printed to the standard output, so it can be
/// written to a file using unix redirection. Giving a directory
/// instead writes one script per shell in it, creating the directory
/// if it does not exist.
#[derive(Debug, Parser)]
pub struct CompletionCommand {
    /// Shell(s) for which completion script should be generated for.
    ///
    /// Only one shell can be given when the script goes to the
    /// standard output, generating several at once requires a
    /// directory.
    #[arg(value_parser = value_parser!(Shell), required = true)]
    pub shells: Vec<Shell>,

    /// Save completion scripts to the given directory.
    #[arg(short, long, value_name = "PATH", value_parser = path_parser)]
    pub dir: Option<PathBuf>,
}

impl CompletionCommand {
    /// Generates the completion scripts, either to the standard
    /// output or to the given directory.
    pub fn execute(self, printer: &mut impl Printer, mut command: Command) -> Result<()> {
        let cmd_name = command.get_name().to_string();

        let Some(dir) = self.dir else {
            let [shell] = self.shells[..] else {
                bail!(
                    "Writing several completion scripts to the standard output is ambiguous, use --dir to generate them as files"
                );
            };

            let mut script = Vec::new();
            clap_complete::generate(shell, &mut command, cmd_name, &mut script);
            let script = String::from_utf8(script)
                .context("Read generated completion script as UTF-8 error")?;
            debug!("generated {shell} completion script");

            return printer.out(Completion { shell, script });
        };

        let dir = dir.canonicalize().unwrap_or(dir);
        fs::create_dir_all(&dir)?;

        let mut scripts = Vec::with_capacity(5);

        for shell in self.shells {
            let path = clap_complete::generate_to(shell, &mut command, &cmd_name, &dir)?;
            let path = path.canonicalize().unwrap_or(path);
            debug!("generated {shell} completion script at {}", path.display());
            scripts.push(Script { shell, path })
        }

        printer.out(Completions { dir, scripts })
    }
}

/// Defines a struct-wrapper to provide a JSON output.
#[derive(Serialize)]
struct Completion {
    #[serde(serialize_with = "serialize_shell")]
    pub shell: Shell,
    pub script: String,
}

impl fmt::Display for Completion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.script)
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

#[cfg(test)]
mod tests {
    use std::fmt;

    use anyhow::Result;
    use clap::Command;
    use clap_complete::Shell;
    use serde::Serialize;

    use super::CompletionCommand;
    use crate::printer::Printer;

    #[derive(Default)]
    struct TestPrinter(String);

    impl Printer for TestPrinter {
        fn out<T: fmt::Display + Serialize>(&mut self, data: T) -> Result<()> {
            self.0 = data.to_string();
            Ok(())
        }
    }

    #[test]
    fn single_shell_script_goes_to_output() {
        let cmd = CompletionCommand {
            shells: vec![Shell::Bash],
            dir: None,
        };

        let mut printer = TestPrinter::default();
        cmd.execute(&mut printer, Command::new("test")).unwrap();

        assert!(printer.0.starts_with("_test() {"), "{}", printer.0);
    }

    #[test]
    fn several_shells_without_dir_are_rejected() {
        let cmd = CompletionCommand {
            shells: vec![Shell::Bash, Shell::Zsh],
            dir: None,
        };

        let err = cmd
            .execute(&mut TestPrinter::default(), Command::new("test"))
            .unwrap_err();

        assert!(err.to_string().contains("use --dir"), "{err}");
    }
}
