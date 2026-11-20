//! Shared clap building blocks: arguments, value parsers, the ready-made
//! completion and manual generation commands, [`parse`], the entry point
//! honouring the exit contract on a refused command line, and
//! [`meet_bare_invocation`], answering a binary run without a command.

pub mod args;
pub mod commands;
pub mod parsers;

use std::{ffi::OsString, mem, process};

use anyhow::{Error, Result};
use clap::{Command, Parser};

use crate::{
    clap::args::JsonFlag,
    error::{EXIT_USAGE, ErrorReport, NoErrorCode, usage_error, usage_invalid},
    printer::{Printer, StdoutPrinter},
};

/// Parses the process arguments into `T`.
///
/// Help and version print and exit as clap does, the version naming the
/// binary after any subcommand. A command line the grammar refuses exits
/// [`EXIT_USAGE`]: with a `--json` word before any `--`, it prints the
/// failure object coded `usage-invalid` on stdout, otherwise clap's
/// rendering on stderr.
pub fn parse<T: Parser>() -> T {
    parse_from(std::env::args_os())
}

/// Parses `args`, the binary name first, into `T`, as [`parse`] does.
pub fn parse_from<T, I>(args: I) -> T
where
    T: Parser,
    I: IntoIterator,
    I::Item: Into<OsString> + Clone,
{
    let args: Vec<OsString> = args.into_iter().map(Into::into).collect();

    let err = match try_parse_from::<T>(&args) {
        Ok(parsed) => return parsed,
        Err(err) => err,
    };

    if !err.use_stderr() || !json_requested(&args) {
        err.exit()
    }

    let mut printer = StdoutPrinter::new(&JsonFlag { enabled: true });
    let _ = printer.out(ErrorReport::<NoErrorCode>::from(usage_error(&err)));
    process::exit(EXIT_USAGE)
}

/// Answers a binary run without a command, its configuration still
/// unread.
///
/// Under JSON, it fails coded `usage-invalid`, which the error report
/// prints as the failure object and exits [`EXIT_USAGE`] with. In text, it
/// fails with the error `config_missing` answers, the missing
/// configuration a person meets as the welcome, or prints the help of
/// `command`, the binary's command tree. `config_missing` answers an error
/// only when no configuration exists: a broken one counts as one, its
/// error left to the first command reading it.
pub fn meet_bare_invocation(
    printer: &impl Printer,
    mut command: Command,
    config_missing: impl FnOnce() -> Option<Error>,
) -> Result<()> {
    if printer.is_json() {
        return Err(usage_invalid("A command is required, see --help"));
    }

    if let Some(err) = config_missing() {
        return Err(err);
    }

    command.print_help()?;
    Ok(())
}

/// Parses `args` into `T` over its command tree whose subcommands carry
/// the binary's name, as [`version_named_after_root`] gives them.
fn try_parse_from<T: Parser>(args: &[OsString]) -> Result<T, clap::Error> {
    let mut command = version_named_after_root(T::command());
    let mut matches = command.try_get_matches_from_mut(args)?;
    T::from_arg_matches_mut(&mut matches).map_err(|err| err.format(&mut command))
}

/// Gives every subcommand of `command`, at any depth, the root's name as
/// its display name, so the version printed after a subcommand names the
/// binary rather than the dashed path of the subcommand.
pub(crate) fn version_named_after_root(command: Command) -> Command {
    let name = command
        .get_display_name()
        .unwrap_or(command.get_name())
        .to_owned();

    let mut command = command;
    named(&mut command, &name);
    command
}

/// Names every subcommand under `command` `name`, depth first.
fn named(command: &mut Command, name: &str) {
    for subcommand in command.get_subcommands_mut() {
        *subcommand = mem::take(subcommand).display_name(name);
        named(subcommand, name);
    }
}

/// Whether a `--json` word comes before any `--`, the binary name aside:
/// clap stops at an error before telling whether the flag is set.
fn json_requested(args: &[OsString]) -> bool {
    args.iter()
        .skip(1)
        .take_while(|arg| *arg != "--")
        .any(|arg| arg == "--json")
}

#[cfg(test)]
mod tests {
    use std::ffi::OsString;

    use anyhow::anyhow;
    use clap::{CommandFactory, Parser, Subcommand, error::ErrorKind};

    use crate::{
        clap::{
            args::JsonFlag,
            commands::{
                completion::CompletionCommand, json_schema::JsonSchemaCommand,
                manual::ManualCommand,
            },
            json_requested, meet_bare_invocation, try_parse_from,
        },
        error::{CommonCode, code_of, failure_status},
        printer::{CapturePrinter, TestPrinter},
    };

    fn args(words: &[&str]) -> Vec<OsString> {
        words.iter().map(OsString::from).collect()
    }

    /// A binary whose command is optional, carrying its version.
    #[derive(Debug, Parser)]
    #[command(name = "app", version = "1.2.3", long_version = "v1.2.3 +imap")]
    #[command(propagate_version = true)]
    struct TestCli {
        #[command(subcommand)]
        cmd: Option<TestCommand>,
        #[command(flatten)]
        json: JsonFlag,
    }

    #[derive(Debug, Subcommand)]
    enum TestCommand {
        #[command(subcommand)]
        Account(TestAccountCommand),
        Completion(CompletionCommand),
        Manual(ManualCommand),
        JsonSchema(JsonSchemaCommand),
    }

    #[derive(Debug, Subcommand)]
    enum TestAccountCommand {
        List,
    }

    #[test]
    fn json_counts_only_before_the_separator() {
        assert!(json_requested(&args(&["app", "--json", "list"])));
        assert!(json_requested(&args(&["app", "list", "--bad", "--json"])));
        assert!(!json_requested(&args(&["app", "add", "--", "--json"])));
        assert!(!json_requested(&args(&["--json", "list"])));
    }

    #[test]
    fn the_version_after_a_subcommand_names_the_binary() {
        for (words, version) in [
            (&["app", "-V"][..], "app 1.2.3\n"),
            (&["app", "account", "-V"][..], "app 1.2.3\n"),
            (
                &["app", "account", "list", "--version"][..],
                "app v1.2.3 +imap\n",
            ),
        ] {
            let err = try_parse_from::<TestCli>(&args(words)).unwrap_err();
            assert_eq!(err.render().to_string(), version, "{words:?}");
        }

        let cli = try_parse_from::<TestCli>(&args(&["app", "account", "list"])).unwrap();
        assert!(matches!(
            cli.cmd,
            Some(TestCommand::Account(TestAccountCommand::List))
        ));
    }

    #[test]
    fn a_standard_command_without_its_argument_is_refused_by_clap() {
        for name in ["completion", "manual", "json-schema"] {
            let err = try_parse_from::<TestCli>(&args(&["app", name])).unwrap_err();
            assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument, "{name}");
            assert_eq!(err.exit_code(), 2, "{name}");

            let dir = try_parse_from::<TestCli>(&args(&["app", name, "--dir", "/tmp"]));
            assert!(dir.is_ok(), "{name}");
        }

        let shell = try_parse_from::<TestCli>(&args(&["app", "completion", "bash"]));
        assert!(shell.is_ok());
        let page = try_parse_from::<TestCli>(&args(&["app", "manual", "app"]));
        assert!(page.is_ok());
    }

    #[test]
    fn a_bare_invocation_under_json_is_a_refused_command_line() {
        let printer = CapturePrinter::default();
        let command = TestCli::command();
        let err = meet_bare_invocation(&printer, command, || panic!("config read")).unwrap_err();

        assert_eq!(err.to_string(), "A command is required, see --help");
        assert_eq!(code_of::<CommonCode>(&err), Some(CommonCode::UsageInvalid));
        assert_eq!(failure_status(&err), 2);
    }

    #[test]
    fn a_bare_invocation_in_text_meets_the_missing_configuration() {
        let printer = TestPrinter::default();
        let command = TestCli::command();
        let missing = || Some(anyhow!("No configuration found"));
        let err = meet_bare_invocation(&printer, command, missing).unwrap_err();

        assert_eq!(err.to_string(), "No configuration found");
        assert_eq!(code_of::<CommonCode>(&err), None);
    }
}
