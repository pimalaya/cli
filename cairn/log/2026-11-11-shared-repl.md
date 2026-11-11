---
cairn: log
change: shared-repl
landed: 2026-11-11
---

# Shared REPL

The read-eval-print loop of Ortie and Himalaya moved into pimalaya-cli behind a new `repl` feature (`terminal` plus shlex 2, no default features: `split` needs only alloc).

- src/repl.rs: `Repl { bin, json, fixed }` with public fields, `new(bin, json)` taking the session's JSON as a bool (an app reaches its REPL holding a printer, so `printer.is_json()`, rather than the `JsonFlag`) and filling `fixed` with `-a`, `--account`, `-c`, `--config`, `--log-level`, `--log` and `--log-file`. A binary pushes its own (Himalaya's `-b`, `--backend`) onto `fixed`. `run::<T, C>(execute)` claims stdin and loops over it; `T: clap::Subcommand` is wrapped in a private generic `Line<T>` (`no_binary_name`, `infer_subcommands`, flattened `JsonFlag`) whose command is renamed to `bin` and stripped of the about the flattened `JsonFlag` would lend it. `execute` gets a fresh `StdoutPrinter` per line, JSON when the session or the line asks.
- The verdicts on the divergences: failures go through the printer on stdout, a terminal included (Himalaya's); nothing is added after a result (Ortie's); `--json` is accepted per line (Ortie's); partial outcomes print nothing more (Himalaya's); a clap parse error goes to clap's rendering on stderr on a terminal and to the failure object on stdout when piped (both). New: a non-UTF-8 line fails alone (lines are read as bytes), a session flag on a line is refused by name once clap fails (so a grammar accepting the word is never second-guessed), and help or version in a JSON session (the session's flag, or a `--json` word before `--`) goes to stderr. The prompt is `<bin>> ` for both, Ortie's leading blank line gone.
- src/session.rs, behind `repl` or `mcp`: Himalaya's `claim_stdin`, `check_stdin` and `in_session`, `check_stdin` taking the binary's code (`check_stdin(ErrorCode::StdinTaken)`) since the vocabulary is the binary's.

The loop core `run_on` takes any `BufRead`, two writers and a printer factory, so the tests run sessions in memory with a printer sharing the stdout sink: shell splitting and an open quote, blank and comment lines, `quit`, `exit` and EOF, a non-UTF-8 line, the session flags (`-a work`, `--config=x.toml`, `-bimap`, and `-- -a` passing as a positional), help on stderr in a JSON session and on stdout in a text one, per-line `--json` with the exact stdout bytes, a partial outcome, a failure on a terminal, a parse error on a terminal and piped. The suite also passes with clap's `usage`, `error-context` and `suggestions` features on, as the apps build it; this crate's own clap has neither, so the tests assert nothing of usage lines or error details.

Spec: repl (new).
