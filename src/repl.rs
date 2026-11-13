//! A read-eval-print loop over stdin, running a binary's own commands
//! against what a session keeps open.
//!
//! A session keeps one process alive for one account, so what the
//! account needs (a client connection, a token read off a keyring) is
//! built once rather than once per command. [`Repl::run`] reads stdin one
//! line at a time, splits it as a shell does, parses it with the binary's
//! command grammar plus `--json`, and hands the command to the binary
//! with a printer of its own.
//!
//! Every answer goes to stdout, failures included, so a piped session
//! answers each line once and an application can keep one open. stderr holds
//! the prompt, clap's rendering of a mistyped line on a terminal, and the
//! help in a JSON session, where stdout keeps one JSON value per line.
//! The session claims stdin ([`claim_stdin`]), so a command reading its
//! input there is refused.
//!
//! [`claim_stdin`]: crate::session::claim_stdin

use std::{
    io::{self, BufRead, IsTerminal, Write},
    str,
};

use anyhow::{Context, Error, Result, anyhow};
use clap::{Command, CommandFactory, FromArgMatches, Parser, Subcommand};

use crate::{
    clap::args::JsonFlag,
    error::{ErrorCode, ErrorReport, partial_outcome_of, usage_error},
    printer::{Printer, StdoutPrinter},
    session::claim_stdin,
};

/// The flags every session fixes when it starts: the shared account,
/// configuration and log flags.
const FIXED: &[&str] = &[
    "-a",
    "--account",
    "-c",
    "--config",
    "--log-level",
    "--log",
    "--log-file",
];

/// The text hint of a failure while debug logs are off: `--log-level` is
/// fixed when the session starts.
const LOG_LEVEL_HINT: &str = "Restart the session with --log-level to enable more verbose logs";

/// A read-eval-print loop over stdin for one binary.
#[derive(Clone, Debug)]
pub struct Repl {
    /// The binary name, which the prompt and the help carry.
    pub bin: &'static str,
    /// Whether the session prints JSON, as its `--json` says.
    pub json: bool,
    /// The flags the session fixes when it starts, spelled as a line
    /// would (`-a`, `--account`): a line setting one is refused naming
    /// it. A binary adds its own, as in `--backend`.
    pub fixed: Vec<&'static str>,
}

impl Repl {
    /// Builds the loop of the binary `bin`, printing JSON when `json` is
    /// set, as the session's printer does, and fixing the shared account,
    /// configuration and log flags.
    pub fn new(bin: &'static str, json: bool) -> Self {
        Self {
            bin,
            json,
            fixed: FIXED.to_vec(),
        }
    }

    /// Runs the loop over stdin until EOF, `quit` or `exit`.
    ///
    /// `T` is the binary's command tree, parsed from each line without
    /// the binary name, and `C` its failure codes. `execute` runs one
    /// parsed command with the printer of its line, JSON when the session
    /// or the line asks for it. The loop claims stdin first, and fails
    /// only when stdin or stdout does.
    pub fn run<T: Subcommand, C: ErrorCode>(
        &self,
        mut execute: impl FnMut(&mut StdoutPrinter, T) -> Result<()>,
    ) -> Result<()> {
        claim_stdin();

        let stdin = io::stdin();

        let mut streams = Streams {
            interactive: stdin.is_terminal(),
            input: stdin.lock(),
            stdout: io::stdout(),
            stderr: io::stderr(),
        };

        let mut printer = |json| StdoutPrinter::new(&JsonFlag { enabled: json });
        self.run_on::<T, C, _>(&mut streams, &mut printer, &mut execute)
    }

    /// Runs the loop over `streams`, each line printing through the
    /// printer `printer` builds for its JSON flag.
    fn run_on<T: Subcommand, C: ErrorCode, P: Printer>(
        &self,
        streams: &mut Streams<impl BufRead, impl Write, impl Write>,
        printer: &mut impl FnMut(bool) -> P,
        execute: &mut impl FnMut(&mut P, T) -> Result<()>,
    ) -> Result<()> {
        // NOTE: the flattened `--json` would lend its doc as the about.
        let mut grammar = Line::<T>::command()
            .name(self.bin)
            .bin_name(self.bin)
            .about(None)
            .long_about(None);

        let mut line = Vec::new();

        loop {
            if streams.interactive {
                write!(streams.stderr, "{}> ", self.bin)?;
                streams.stderr.flush()?;
            }

            line.clear();

            let read = streams.input.read_until(b'\n', &mut line);
            if read.context("Cannot read a line from stdin")? == 0 {
                return Ok(());
            }

            match self.step::<T>(&mut grammar, &line) {
                Step::Quit => return Ok(()),
                Step::Skip => continue,
                Step::Run { cmd, json } => {
                    let mut printer = printer(json);
                    let result = execute(&mut printer, cmd);
                    answer::<C>(&mut printer, result)?;
                }
                Step::Fail { err, json } => answer::<C>(&mut printer(json), Err(err))?,
                // Help and version are no failures, on stderr in a JSON
                // session so stdout stays JSON.
                Step::Clap { err, json: true } if !err.use_stderr() => {
                    write!(streams.stderr, "{}", err.render())?
                }
                Step::Clap { err, json: false } if !err.use_stderr() => {
                    write!(streams.stdout, "{}", err.render())?
                }
                Step::Clap { err, .. } if streams.interactive => {
                    write!(streams.stderr, "{}", err.render())?
                }
                Step::Clap { err, json } => {
                    answer::<C>(&mut printer(json), Err(usage_error(&err)))?
                }
            }

            streams.stdout.flush()?;
        }
    }

    /// Sorts one line read off stdin.
    fn step<T: Subcommand>(&self, grammar: &mut Command, line: &[u8]) -> Step<T> {
        let Ok(line) = str::from_utf8(line) else {
            let err = anyhow!("Cannot read the line: it is not valid UTF-8");
            return Step::Fail {
                err,
                json: self.json,
            };
        };

        let Some(words) = shlex::split(line) else {
            let err = anyhow!("Cannot split the line: a quote is not closed");
            return Step::Fail {
                err,
                json: self.json,
            };
        };

        match words.as_slice() {
            [] => return Step::Skip,
            [word] if word == "quit" || word == "exit" => return Step::Quit,
            _ => (),
        }

        // The words before any `--` are flags: clap stops at help or at an
        // error before telling whether `--json` is set.
        let flags = words.iter().take_while(|word| *word != "--");
        let json = self.json || flags.clone().any(|word| word == "--json");

        let parsed = grammar
            .try_get_matches_from_mut(&words)
            .and_then(|matches| {
                Line::<T>::from_arg_matches(&matches).map_err(|err| err.format(grammar))
            });

        let err = match parsed {
            Ok(line) => {
                let json = self.json || line.json.enabled;
                return Step::Run {
                    cmd: line.cmd,
                    json,
                };
            }
            Err(err) => err,
        };

        if !err.use_stderr() {
            return Step::Clap { err, json };
        }

        let fixed = flags.into_iter().find_map(|word| {
            self.fixed.iter().copied().find(|flag| {
                if flag.starts_with("--") {
                    let value = word.strip_prefix(flag);
                    value.is_some_and(|value| value.is_empty() || value.starts_with('='))
                } else {
                    // A short flag, its value attached or not.
                    !word.starts_with("--") && word.starts_with(flag)
                }
            })
        });

        match fixed {
            Some(flag) => {
                let err = anyhow!(
                    "Cannot set `{flag}` inside a session, start a new session with it instead"
                );
                Step::Fail { err, json }
            }
            None => Step::Clap { err, json },
        }
    }
}

/// The words of one line: the binary's command tree and the line's own
/// `--json`.
#[derive(Debug, Parser)]
#[command(no_binary_name = true, infer_subcommands = true)]
struct Line<T: Subcommand> {
    #[command(subcommand)]
    cmd: T,
    #[command(flatten)]
    json: JsonFlag,
}

/// Where the loop reads its lines and writes what no printer does.
struct Streams<I, O, E> {
    input: I,
    stdout: O,
    stderr: E,
    /// Whether a person types the lines: the prompt renders, and a
    /// mistyped line gets clap's rendering on stderr.
    interactive: bool,
}

/// What one line asks of the loop.
enum Step<T> {
    /// End the session.
    Quit,
    /// Read the next line: a blank or comment line.
    Skip,
    /// Run the command, printing JSON or not.
    Run { cmd: T, json: bool },
    /// Print the failure object, as JSON or not.
    Fail { err: Error, json: bool },
    /// Answer clap's verdict, help or version or a parse error, in a JSON
    /// session or not.
    Clap { err: clap::Error, json: bool },
}

/// Prints how a command ended: nothing more after its result or a
/// partial outcome, the failure object after a failure.
fn answer<C: ErrorCode>(printer: &mut impl Printer, result: Result<()>) -> Result<()> {
    match result {
        Err(err) if partial_outcome_of(&err).is_none() => {
            let mut report = ErrorReport::<C>::from(err);
            report.log_level_hint = LOG_LEVEL_HINT;
            printer.out(report)
        }
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use std::{
        cell::RefCell,
        fmt,
        io::{self, Write},
        rc::Rc,
    };

    use anyhow::{Result, bail};
    use clap::Subcommand;
    use serde::Serialize;
    use serde_json::{Value, json};

    use crate::{
        error::{NoErrorCode, PartialOutcome},
        output::Bytes,
        printer::Printer,
        repl::{Repl, Streams},
    };

    /// A stream shared between the loop and the printers.
    #[derive(Clone, Default)]
    struct Sink(Rc<RefCell<Vec<u8>>>);

    impl Sink {
        fn text(&self) -> String {
            String::from_utf8(self.0.borrow().clone()).unwrap()
        }
    }

    impl Write for Sink {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.0.borrow_mut().extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    /// A printer writing to a [`Sink`] the way the stdout one writes to
    /// a pipe.
    struct SinkPrinter {
        sink: Sink,
        json: bool,
    }

    impl Printer for SinkPrinter {
        fn out<T: fmt::Display + Serialize>(&mut self, data: T) -> Result<()> {
            if self.json {
                serde_json::to_writer(&mut self.sink, &data)?;
                writeln!(self.sink)?;
            } else {
                write!(self.sink, "{data}")?;
            }

            Ok(())
        }

        fn event<T: fmt::Display + Serialize>(&mut self, event: T) -> Result<()> {
            self.out(event)
        }

        fn out_bytes(&mut self, bytes: Bytes) -> Result<()> {
            self.out(bytes)
        }

        fn is_json(&self) -> bool {
            self.json
        }
    }

    #[derive(Debug, Subcommand)]
    enum TestCommand {
        /// Print a word.
        Echo { word: String },
        /// Fail.
        Fail,
        /// Print a word, then end with a partial outcome.
        Partial,
    }

    #[derive(Serialize)]
    struct Word {
        word: String,
    }

    impl fmt::Display for Word {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            writeln!(f, "{}", self.word)
        }
    }

    fn word(printer: &mut SinkPrinter, word: &str) -> Result<()> {
        printer.out(Word {
            word: word.to_owned(),
        })
    }

    /// Runs a session over `input`, answering its stdout and stderr.
    fn session(json: bool, interactive: bool, input: &[u8]) -> (String, String) {
        let mut repl = Repl::new("app", json);
        repl.fixed.extend(["-b", "--backend"]);
        let stdout = Sink::default();
        let stderr = Sink::default();

        let mut streams = Streams {
            input,
            stdout: stdout.clone(),
            stderr: stderr.clone(),
            interactive,
        };

        let mut printer = |json| SinkPrinter {
            sink: stdout.clone(),
            json,
        };

        let mut execute = |printer: &mut SinkPrinter, cmd| match cmd {
            TestCommand::Echo { word: w } => word(printer, &w),
            TestCommand::Fail => bail!("Boom"),
            TestCommand::Partial => {
                word(printer, "half")?;
                Err(PartialOutcome::default().into())
            }
        };

        repl.run_on::<TestCommand, NoErrorCode, _>(&mut streams, &mut printer, &mut execute)
            .unwrap();

        (stdout.text(), stderr.text())
    }

    /// The JSON values of a piped JSON session's stdout.
    fn values(stdout: &str) -> Vec<Value> {
        stdout
            .lines()
            .map(|line| serde_json::from_str(line).unwrap_or_else(|err| panic!("{err}: {line}")))
            .collect()
    }

    fn error(value: &Value) -> &str {
        value["error"].as_str().unwrap_or_default()
    }

    #[test]
    fn lines_are_split_as_a_shell_splits_them() {
        let input = b"echo \"a b\"\necho 'c\nec d\\ e\n";
        let (stdout, stderr) = session(true, false, input);
        let values = values(&stdout);

        assert_eq!(values.len(), 3, "{stdout}");
        assert_eq!(values[0], json!({ "word": "a b" }));
        assert_eq!(
            error(&values[1]),
            "Cannot split the line: a quote is not closed"
        );
        assert_eq!(values[2], json!({ "word": "d e" }));
        assert_eq!(stderr, "");
    }

    #[test]
    fn blank_and_comment_lines_are_skipped() {
        let input = b"\n   \n# echo a\necho b # c\n";
        let (stdout, _) = session(true, false, input);

        assert_eq!(values(&stdout), [json!({ "word": "b" })]);
    }

    #[test]
    fn quit_exit_and_eof_end_the_session() {
        for input in [&b"echo a\nquit\necho b\n"[..], b"echo a\n exit \necho b\n"] {
            let (stdout, _) = session(false, false, input);
            assert_eq!(stdout, "a\n");
        }

        let (stdout, _) = session(false, false, b"echo a\necho b");
        assert_eq!(stdout, "a\nb\n");
    }

    #[test]
    fn a_line_that_is_not_utf8_fails_alone() {
        let (stdout, _) = session(true, false, b"echo \xff\necho a\n");
        let values = values(&stdout);

        assert_eq!(values.len(), 2, "{stdout}");
        assert_eq!(
            error(&values[0]),
            "Cannot read the line: it is not valid UTF-8"
        );
        assert_eq!(values[1], json!({ "word": "a" }));
    }

    #[test]
    fn a_session_flag_is_refused_by_name() {
        let input =
            b"echo a -a work\necho a --config=x.toml\necho -bimap a\necho -- -a\necho a --nope\n";
        let (stdout, _) = session(true, false, input);
        let values = values(&stdout);

        assert_eq!(values.len(), 5, "{stdout}");

        for (value, flag) in values.iter().zip(["-a", "--config", "-b"]) {
            let message = format!(
                "Cannot set `{flag}` inside a session, start a new session with it instead"
            );
            assert_eq!(error(value), message);
        }

        assert_eq!(values[3], json!({ "word": "-a" }));
        assert!(!error(&values[4]).contains("Cannot set"), "{stdout}");
    }

    #[test]
    fn help_goes_to_stderr_in_a_json_session() {
        let (stdout, stderr) = session(true, false, b"help\necho --json --help\n");
        assert_eq!(stdout, "");
        assert!(stderr.starts_with("Usage:"), "{stderr}");
        assert!(stderr.contains("  echo     Print a word\n"), "{stderr}");
        assert!(stderr.contains("\nPrint a word\n"), "{stderr}");

        let (stdout, stderr) = session(false, false, b"help\n");
        assert!(stdout.contains("  echo     Print a word\n"), "{stdout}");
        assert_eq!(stderr, "");
    }

    #[test]
    fn a_line_json_flag_applies_to_that_line_and_nothing_is_added() {
        let (stdout, _) = session(false, false, b"echo a --json\necho b\nfail --json\n");
        let mut lines = stdout.lines();

        assert_eq!(lines.next(), Some(r#"{"word":"a"}"#));
        assert_eq!(lines.next(), Some("b"));
        assert_eq!(
            error(&serde_json::from_str(lines.next().unwrap()).unwrap()),
            "Boom"
        );
        assert_eq!(lines.next(), None);
        assert!(stdout.starts_with("{\"word\":\"a\"}\nb\n{"), "{stdout}");
    }

    #[test]
    fn a_partial_outcome_prints_its_result_alone() {
        let (stdout, _) = session(true, false, b"partial\necho a\n");
        assert_eq!(
            values(&stdout),
            [json!({ "word": "half" }), json!({ "word": "a" })]
        );
    }

    #[test]
    fn a_failure_prints_on_stdout_even_on_a_terminal() {
        let (stdout, stderr) = session(false, true, b"fail\n");

        assert!(stdout.starts_with("Error: Boom\n"), "{stdout}");
        assert_eq!(stderr, "app> app> ");
    }

    #[test]
    fn a_failure_hints_at_restarting_the_session_for_logs() {
        let (stdout, _) = session(false, false, b"fail\n");
        let hint = "\n - Restart the session with --log-level to enable more verbose logs\n";
        assert!(stdout.ends_with(hint), "{stdout}");
    }

    #[test]
    fn a_command_help_names_the_binary_in_its_usage() {
        let (stdout, _) = session(false, false, b"help echo\necho --help\n");
        let usage: Vec<&str> = stdout
            .lines()
            .filter(|line| line.starts_with("Usage:"))
            .collect();

        assert_eq!(usage, ["Usage: app echo [OPTIONS] <WORD>"; 2], "{stdout}");
    }

    #[test]
    fn a_parse_error_goes_to_clap_on_a_terminal_and_to_stdout_when_piped() {
        let (stdout, stderr) = session(true, true, b"frobnicate\n");
        assert_eq!(stdout, "");
        assert!(
            stderr.contains("error: unrecognized subcommand"),
            "{stderr}"
        );

        let (stdout, stderr) = session(true, false, b"frobnicate\n");
        let values = values(&stdout);
        assert_eq!(values.len(), 1, "{stdout}");
        assert!(
            error(&values[0]).contains("unrecognized subcommand"),
            "{stdout}"
        );
        assert_eq!(values[0]["code"], "usage-invalid", "{stdout}");
        assert_eq!(stderr, "");

        let (stdout, _) = session(false, false, b"frobnicate\n");
        assert!(
            stdout.starts_with("Error: unrecognized subcommand"),
            "{stdout}"
        );
    }
}
