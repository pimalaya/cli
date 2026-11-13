---
cairn: change
id: a-refused-line-exits-two
status: landed
created: 2026-11-13
---

# A refused line exits two

## Why

A command line clap refuses exits 2 with clap's text on stderr, whatever `--json` says, while the family gave 2 to partial outcomes too: a script could not tell a typo from a `check` with a failed backend, and a `--json` caller got no failure object at all. The Himalaya local-backends campaign (2026-11-13) reported it on every backend. No binary of the family is released with the old numbering, so the owner chose on 2026-11-13 to give 2 to clap and move partial outcomes to 3.

## What

- `error::EXIT_USAGE` is 2, `error::EXIT_PARTIAL` 3; `PartialOutcome` refuses a status below 3.
- `error::CommonCode`, codes pimalaya-cli raises beside each binary's vocabulary, holds `usage-invalid`; the failure object's `code` and its schema admit it.
- `error::usage_error` turns a clap error into a failure coded `usage-invalid`, clap's message without its `error: ` prefix or styles.
- `clap::parse` and `clap::parse_from` replace `Parser::parse`: help and version as clap does; a refused line exits 2, printing the failure object on stdout when a `--json` word comes before any `--`, clap's text on stderr otherwise.
- The REPL answers a refused line with that coded failure, which also ends the doubled `Error: error:` of its text form.
- The MCP server and `mcp::grammar::Grammar::argv` code their argument refusals (an undeclared key, a value of the wrong type, a wrong count) `usage-invalid` too.
