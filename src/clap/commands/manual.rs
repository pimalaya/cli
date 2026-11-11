//! The manual command, writing man pages.

use std::{fmt, fs, path::PathBuf};

use anyhow::{Context, Result, bail};
use clap::{Command, Parser};
use clap_mangen::Man;
use log::debug;
use schemars::JsonSchema;
use serde::Serialize;

use crate::{clap::parsers::path_parser, printer::Printer};

/// Generate manual page for the given command(s).
///
/// This command allows you to generate manual page (following the man
/// page format) for a given command. The page is printed to the
/// standard output, so it can be written to a file using unix
/// redirection. Giving a directory instead writes one page per
/// command in it, creating the directory if it does not exist.
#[derive(Debug, Parser)]
pub struct ManualCommand {
    /// Command(s) for which manual page should be generated for.
    ///
    /// A command is named after its manual page: the program name for
    /// the main page, the program name followed by a dash and the
    /// subcommand name for a subcommand one. Defaults to every
    /// command, which requires a directory.
    pub cmds: Vec<String>,

    /// Save manual pages to the given directory.
    #[arg(short, long, value_name = "PATH", value_parser = path_parser)]
    pub dir: Option<PathBuf>,
}

impl ManualCommand {
    /// Generates the manual pages, either to the standard output or
    /// to the given directory.
    pub fn execute(self, printer: &mut impl Printer, command: Command) -> Result<()> {
        let cmd_name = command.get_name().to_string();

        let mut cmds = vec![(cmd_name.clone(), command.clone())];

        for subcmd in command.get_subcommands() {
            let name = format!("{cmd_name}-{}", subcmd.get_name());
            cmds.push((name, subcmd.clone()));
        }

        for cmd in &self.cmds {
            if !cmds.iter().any(|(name, _)| name == cmd) {
                bail!("Cannot find command {cmd}");
            }
        }

        if !self.cmds.is_empty() {
            cmds.retain(|(name, _)| self.cmds.contains(name));
        }

        let Some(dir) = self.dir else {
            let mut cmds = cmds.into_iter();

            let (Some((cmd, command)), None) = (cmds.next(), cmds.next()) else {
                bail!(
                    "Writing several manual pages to the standard output is ambiguous, use --dir to generate them as files"
                );
            };

            let mut page = Vec::new();
            Man::new(command).render(&mut page)?;
            let page =
                String::from_utf8(page).context("Read generated manual page as UTF-8 error")?;
            debug!("generated {cmd} manual page");

            return printer.out(ManualOutput::Page { cmd, page });
        };

        let dir = dir.canonicalize().unwrap_or(dir);
        fs::create_dir_all(&dir)?;

        let mut pages = Vec::with_capacity(cmds.len());

        for (cmd, command) in cmds {
            let mut page = Vec::new();
            Man::new(command).render(&mut page)?;

            let path = dir.join(format!("{cmd}.1"));
            fs::write(&path, page)?;
            debug!("generated {cmd} manual page at {}", path.display());
            pages.push(PageFile { cmd, path })
        }

        printer.out(ManualOutput::Files { dir, pages })
    }
}

/// What `manual` prints: the one page asked, or the files written.
#[derive(Serialize, JsonSchema)]
#[serde(untagged)]
pub(crate) enum ManualOutput {
    /// The one page asked for.
    Page {
        /// The page name.
        cmd: String,
        /// The manual page, in roff.
        page: String,
    },
    /// The page files written.
    Files {
        /// The directory holding the files.
        dir: PathBuf,
        /// The files written, one per command.
        pages: Vec<PageFile>,
    },
}

impl fmt::Display for ManualOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Page { page, .. } => f.write_str(page),
            Self::Files { dir, pages } => {
                let n = pages.len();
                let p = dir.display();
                writeln!(f, "{n} manual page(s) successfully generated in {p}:")?;

                for PageFile { cmd, path } in pages {
                    let p = path.display();
                    writeln!(f, " - {cmd} manual page at {p}")?;
                }

                Ok(())
            }
        }
    }
}

/// A manual page file written.
#[derive(Serialize, JsonSchema)]
pub(crate) struct PageFile {
    /// The page name.
    cmd: String,
    /// The path of the file.
    path: PathBuf,
}

#[cfg(test)]
mod tests {
    use clap::Command;

    use crate::{clap::commands::manual::ManualCommand, printer::TestPrinter};

    #[test]
    fn single_command_page_goes_to_output() {
        let cmd = ManualCommand {
            cmds: vec!["test-sub".into()],
            dir: None,
        };

        let mut printer = TestPrinter::default();
        let command = Command::new("test").subcommand(Command::new("sub"));
        cmd.execute(&mut printer, command).unwrap();

        assert!(printer.output.contains(".TH sub 1"), "{}", printer.output);
    }

    #[test]
    fn several_commands_without_dir_are_rejected() {
        let cmd = ManualCommand {
            cmds: Vec::new(),
            dir: None,
        };

        let command = Command::new("test").subcommand(Command::new("sub"));
        let err = cmd
            .execute(&mut TestPrinter::default(), command)
            .unwrap_err();

        assert!(err.to_string().contains("use --dir"), "{err}");
    }

    #[test]
    fn unknown_command_is_rejected() {
        let cmd = ManualCommand {
            cmds: vec!["test-unknown".into()],
            dir: None,
        };

        let command = Command::new("test").subcommand(Command::new("sub"));
        let err = cmd
            .execute(&mut TestPrinter::default(), command)
            .unwrap_err();

        assert_eq!(err.to_string(), "Cannot find command test-unknown");
    }
}
