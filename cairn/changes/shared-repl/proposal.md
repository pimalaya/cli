---
cairn: change
id: shared-repl
status: landed
created: 2026-11-11
---

# Shared REPL

## Why

Ortie (src/repl.rs) and Himalaya (src/repl.rs) each hand-roll the same read-eval-print loop: read a line off stdin, split it as a shell does, parse it with the CLI grammar minus the binary name, run it over a session holding the account, print the answer, quit on `quit`, `exit` or EOF. The copies have drifted:

- Errors: Himalaya prints every command error through the printer on stdout, a terminal session included; Ortie prints `{err:?}` on stderr when stdin is a terminal. The family rule is that a failure is printed on stdout.
- Newline: Ortie adds nothing after a result, every text output ending with its own newline; Himalaya adds one more after every line.
- Per-line `--json`: Ortie accepts it and prints that line through a JSON printer of its own; Himalaya's line grammar rejects it.
- Partial outcomes (`error::PartialOutcome`) are handled by Himalaya only; Ortie prints a failure object after the result.
- A session flag on a line (`-a`, `-c`, Himalaya's `--backend`) gets clap's generic unexpected argument error in both.
- `help` in a `--json` session prints clap's text on stdout, breaking one JSON value per line.
- A non-UTF-8 line ends both loops with an I/O error.

Himalaya also guards stdin (`claim_stdin`, `check_stdin`, `in_session`): inside a session stdin carries the commands, so a command reading its input there is refused. The guard is generic and every session needs it.

## What

Behind a new `repl` feature (`terminal` plus shlex, no default features), src/repl.rs:

- `repl::Repl { bin, json, fixed }`, built by `new(bin, json)` with whether the session prints JSON (the session printer's `is_json`) and the shared session flags (`-a`, `--account`, `-c`, `--config`, `--log-level`, `--log`, `--log-file`), to which a binary adds its own through the public `fixed` field.
- `Repl::run::<T, C>(execute)`, the loop over stdin: `T` is the binary's `clap::Subcommand` tree, wrapped with `--json` in a line grammar without binary name and with inferred subcommands, `C` its error code vocabulary, `execute` the closure running one parsed command against a `StdoutPrinter` for that line.
- Lines read as bytes, split by shlex; a blank or comment-only line skipped; `quit` and `exit` end the session; an unclosed quote or a non-UTF-8 line fails alone.
- Each line prints through a printer of its own, JSON when the session or the line asks; nothing is added after a result; stdout is flushed after each line.
- A failure prints the failure object (`ErrorReport<C>`) through that printer on stdout, interactive or not; a partial outcome prints nothing more; a clap parse error prints clap's rendering on stderr on a terminal and the failure object on stdout when piped.
- A line setting a session flag fails naming it.
- `help` and `--version` print clap's text on stdout in a text session and on stderr in a JSON one.
- The prompt `<bin>> ` renders on stderr when stdin is a terminal.

Behind `repl` or `mcp`, src/session.rs: `claim_stdin`, `check_stdin::<C>(code)` and `in_session`, Himalaya's guard with the code given by the binary. `Repl::run` claims stdin.

The loop runs over any `BufRead` and writers inside the crate, so the tests drive it without a process.

## Not in scope

The line editor (cairn/changes/repl-line-editor): it will take its place in this loop's line reading. The moves of Ortie and Himalaya onto the module, each a change in its own repository.
