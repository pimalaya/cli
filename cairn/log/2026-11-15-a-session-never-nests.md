---
cairn: log
change: a-session-never-nests
landed: 2026-11-15
---

# A session never nests

Task L1 of the CLI alignment round of 2026-11-15 (findings F3 and F34, decision D3, and the `--version` line of task L3).

## What landed

- `repl::Repl` refuses a line whose command is `repl` or `mcp`, by name, alias or inferred prefix (clap's `subcommand_name`), with `Cannot start a session inside a session` coded `usage-invalid`, before the binary's closure runs. The arms himalaya, cardamum and calendula keep for it (src/repl.rs:49, :52, :52) are now dead code to drop.
- The refused session flags, the unclosed quote and the line that is not UTF-8 are coded `usage-invalid`, where they printed `code: null`.
- The version the command tree carries is propagated to every command: a binary declaring `#[command(version, long_version = long_version!())]` on its command enum answers `-V` and `--version` after any command, on stdout in a text session and on stderr in a JSON one. No binary declares it there today (each declares it on its root `Cli` struct), so each keeps the refusal until it does. No public item added.
- `a_nested_session_is_refused_before_the_binary_runs_it`, `a_version_line_prints_the_version_the_tree_carries`, and the codes asserted in the refusal tests.

## Capabilities moved

- [repl](../spec/repl.md): *A session never nests* added; *A session reads one command per line*, *Session flags are refused by name* and *Help keeps JSON lines whole* modified.
