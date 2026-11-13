---
cairn: log
change: a-refused-line-exits-two
landed: 2026-11-13
---

# A refused line exits two

src/error.rs: `EXIT_USAGE` (2) beside `EXIT_PARTIAL`, now 3, which `PartialOutcome::new` uses as its floor. `CommonCode` (`usage-invalid`) is pimalaya-cli's own vocabulary; the report resolves its code as the binary's first, then a common one, through a private untagged `ReportCode<C>`, so the JSON stays a bare string and the schema an `anyOf` of both enums. `usage_error` renders the clap error unstyled (`StyledStr` display), strips `error: ` and trailing blanks, and wraps it in `CodedError<CommonCode>`.

src/clap/mod.rs: `parse` and `parse_from` call `try_parse_from`; an error clap writes to stdout (help, version) or any error without a `--json` word before `--` goes to `clap::Error::exit` (stderr, exit 2 for a refusal), else the failure object prints through a JSON `StdoutPrinter` and the process exits `EXIT_USAGE`.

src/repl.rs: a piped refused line goes through `usage_error`, so its JSON carries `usage-invalid` and its text reads `Error: unrecognized subcommand ...` instead of `Error: error: ...`.

src/mcp.rs and src/mcp/grammar.rs: the undeclared-argument refusal and the `argv` refusals (boolean, scalar, array, exactly one) go through the crate-private `error::usage_invalid`, a `CodedError<CommonCode>` with its own message, so a tool call's failure object carries `usage-invalid`.

Tests: three refused lines (unknown flag, missing argument, bad value) coded without prefix or ANSI, the schema admitting `usage-invalid`, the REPL in JSON and text, `--json` counted only before `--`.

Spec: error ("Failures carry an optional stable code" names the common code, "A refused command line exits 2" added, intro renumbered), output ("A protocol refusal is a result" and "A partial outcome prints its result" exit 3), repl ("Each line answers through its own printer"), mcp ("A call is checked before it runs", "A tool's input is its command's grammar").

Binaries adopt `clap::parse` and renumber their partial outcomes on their own: cardamum, calendula and neverest on 2026-11-13, himalaya after its pimdir follow.
