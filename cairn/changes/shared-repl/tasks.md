---
cairn: tasks
change: shared-repl
---

- [x] `repl` feature (`terminal`, shlex without default features)
- [x] src/session.rs: `claim_stdin`, `check_stdin`, `in_session`, behind `repl` or `mcp`
- [x] src/repl.rs: `Repl`, the line grammar over the binary's subcommands plus `--json`, the loop over any `BufRead`
- [x] Tests: splitting, blank and comment lines, quit, exit and EOF, unclosed quote, non-UTF-8, session flag refusal, help in a JSON session on stderr, per-line `--json`, no added newline, partial outcome, parse error on a terminal and piped, the stdin guard
- [x] src/lib.rs header, README, CONTRIBUTING, CHANGELOG
- [x] fmt, clippy `-D warnings` per feature, tests, doc
- [x] Delta folded into a new repl spec, log written
