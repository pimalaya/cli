//! Shared clap building blocks: arguments, value parsers, the ready-made
//! completion and manual generation commands, and [`parse`], the entry
//! point honouring the exit contract on a refused command line.

pub mod args;
pub mod commands;
pub mod parsers;

use std::{ffi::OsString, process};

use clap::Parser;

use crate::{
    clap::args::JsonFlag,
    error::{EXIT_USAGE, ErrorReport, NoErrorCode, usage_error},
    printer::{Printer, StdoutPrinter},
};

/// Parses the process arguments into `T`.
///
/// Help and version print and exit as clap does. A command line the
/// grammar refuses exits [`EXIT_USAGE`]: with a `--json` word before any
/// `--`, it prints the failure object coded `usage-invalid` on stdout,
/// otherwise clap's rendering on stderr.
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

    let err = match T::try_parse_from(&args) {
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

    use super::json_requested;

    fn args(words: &[&str]) -> Vec<OsString> {
        words.iter().map(OsString::from).collect()
    }

    #[test]
    fn json_counts_only_before_the_separator() {
        assert!(json_requested(&args(&["app", "--json", "list"])));
        assert!(json_requested(&args(&["app", "list", "--bad", "--json"])));
        assert!(!json_requested(&args(&["app", "add", "--", "--json"])));
        assert!(!json_requested(&args(&["--json", "list"])));
    }
}
