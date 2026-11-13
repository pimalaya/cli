//! The manual command, writing man pages.

use std::{fmt, fs, mem, path::PathBuf};

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
    /// the main page, the program name followed by each subcommand name,
    /// dash-separated, for a subcommand one at any depth. Defaults to
    /// every command, which requires a directory.
    pub cmds: Vec<String>,

    /// Save manual pages to the given directory.
    #[arg(short, long, value_name = "PATH", value_parser = path_parser)]
    pub dir: Option<PathBuf>,
}

impl ManualCommand {
    /// Generates the manual pages, either to the standard output or
    /// to the given directory.
    ///
    /// Every visible command of the tree gets a page, named and titled
    /// after its dashed path, as in `<bin>-envelope-list`. The `help`
    /// subcommands clap adds are left out, so every page a SEE ALSO lists
    /// exists. `command` is expected unbuilt, otherwise they are in.
    pub fn execute(self, printer: &mut impl Printer, mut command: Command) -> Result<()> {
        without_help_subcommand(&mut command);
        command.build();

        let source = match command.get_version() {
            Some(version) => format!("{} {version}", command.get_name()),
            None => command.get_name().to_owned(),
        };

        let mut cmds = Vec::new();
        collect_pages(&command, &mut cmds);

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
            Man::new(command).source(&source).render(&mut page)?;
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
            Man::new(command).source(&source).render(&mut page)?;

            let path = dir.join(format!("{cmd}.1"));
            fs::write(&path, page)?;
            debug!("generated {cmd} manual page at {}", path.display());
            pages.push(PageFile { cmd, path })
        }

        printer.out(ManualOutput::Files { dir, pages })
    }
}

/// Disables the `help` subcommand clap adds at build time, in `command`
/// and its whole tree.
fn without_help_subcommand(command: &mut Command) {
    *command = mem::take(command).disable_help_subcommand(true);

    for subcommand in command.get_subcommands_mut() {
        without_help_subcommand(subcommand);
    }
}

/// Pushes the page of `command` then those of its visible subcommands,
/// depth first, each named after its dashed path.
///
/// `command` comes from a built tree, whose display names spell that path.
fn collect_pages(command: &Command, pages: &mut Vec<(String, Command)>) {
    let name = command.get_display_name().unwrap_or(command.get_name());
    pages.push((name.to_owned(), command.clone()));

    for subcommand in command.get_subcommands().filter(|cmd| !cmd.is_hide_set()) {
        collect_pages(subcommand, pages);
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
    use std::{env, fs, process};

    use clap::Command;

    use crate::{clap::commands::manual::ManualCommand, printer::TestPrinter};

    /// A tree two levels deep, with a hidden subcommand.
    fn tree() -> Command {
        Command::new("test")
            .version("1.2.3")
            .subcommand(
                Command::new("sub")
                    .subcommand(Command::new("list"))
                    .subcommand(Command::new("search")),
            )
            .subcommand(Command::new("secret").hide(true))
    }

    fn page(name: &str) -> String {
        let cmd = ManualCommand {
            cmds: vec![name.into()],
            dir: None,
        };

        let mut printer = TestPrinter::default();
        cmd.execute(&mut printer, tree()).unwrap();
        printer.output
    }

    #[test]
    fn single_command_page_goes_to_output() {
        let page = page("test-sub");
        assert!(page.contains(".TH test-sub 1  \"test 1.2.3\""), "{page}");
    }

    #[test]
    fn a_leaf_has_a_page_titled_after_its_whole_path() {
        let page = page("test-sub-list");

        assert!(page.contains(".TH test-sub-list 1"), "{page}");
        assert!(page.contains("\\fBtest sub list\\fR"), "{page}");
    }

    #[test]
    fn see_also_names_pages_that_exist() {
        let page = page("test-sub");

        assert!(page.contains("test\\-sub\\-list(1)"), "{page}");
        assert!(page.contains("test\\-sub\\-search(1)"), "{page}");
        assert!(!page.contains("help(1)"), "{page}");
    }

    #[test]
    fn every_visible_command_gets_a_file() {
        let dir = env::temp_dir().join(format!("pimalaya-cli-manual-{}", process::id()));

        let cmd = ManualCommand {
            cmds: Vec::new(),
            dir: Some(dir.clone()),
        };

        let mut printer = TestPrinter::default();
        cmd.execute(&mut printer, tree()).unwrap();

        let mut files: Vec<_> = fs::read_dir(&dir)
            .unwrap()
            .map(|entry| entry.unwrap().file_name().into_string().unwrap())
            .collect();
        files.sort();
        fs::remove_dir_all(&dir).unwrap();

        assert_eq!(
            files,
            [
                "test-sub-list.1",
                "test-sub-search.1",
                "test-sub.1",
                "test.1"
            ]
        );
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
