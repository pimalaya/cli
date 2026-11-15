---
cairn: change
id: a-session-never-nests
status: landed
created: 2026-11-15
---

# A session never nests

## Why

The CLI alignment round of 2026-11-15 (pimalaya/.github cairn/changes/cli-alignment-2026-11-15, task L1, findings F3 and F34, decision D3) found that himalaya, cardamum and calendula each refuse a `repl` or `mcp` line inside a session with their own uncoded error (himalaya/src/repl.rs:49, cardamum/src/repl.rs:52, calendula/src/repl.rs:52), while ortie answers `usage-invalid` from a reduced grammar. The session flags the kit refuses by name (`-a`, `-c`, `--log-level`, `--log-file`), the unclosed quote and the line that is not UTF-8 printed `code: null` everywhere. The owner ruled that every REPL takes the binary's whole grammar and that the nested-session refusal is coded `usage-invalid`, written once here. The spec also promised that a `--version` line prints clap's text, where the code answered `usage-invalid`: the command enum a session parses lines with carries no version, the binaries declaring theirs on their root `Cli` struct (himalaya/src/cli.rs:77-80), so no `-V` or `--version` flag exists in a session.

## What

- `repl::Repl` refuses a line whose command is `repl` or `mcp` (by name, alias or inferred prefix) with `Cannot start a session inside a session`, coded `usage-invalid`, before the binary's closure runs, so the apps' own arms become dead code they can drop.
- The refused session flags, the unclosed quote and the line that is not UTF-8 are coded `usage-invalid`.
- The version the command tree carries is propagated to every command, as the binaries' own `propagate_version` does. A binary gets `-V` and `--version` in a session by declaring `#[command(version, long_version = long_version!())]` on its command enum, which clap also applies to its root command; one declaring none keeps today's refusal. No public item is added for it.

Non-breaking for the apps: no public item changed or removed, no field added.
