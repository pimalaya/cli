---
cairn: change
id: repl-and-mcp-polish
status: landed
created: 2026-11-11
---

# REPL and MCP polish

## Why

Ortie and Himalaya, the two binaries running on the shared REPL and MCP server, met three rough edges:

1. A failure inside a session hints `Run with --log-level to enable more verbose logs`, but a line setting `--log-level` is refused: the session fixes it when it starts.
2. The help of a command inside a session renders `Usage: envelope list [OPTIONS]`, without the binary name the root help shows (`Usage: <bin> [OPTIONS] <COMMAND>`): the line grammar parses without a binary name, so clap has no `bin_name` to put before the subcommand.
3. Himalaya builds `{"type": "text", "text": ...}` by hand to embed a message part, while the server keeps its text item builder private.

## What

- A failure's text form inside a session hints `Restart the session with --log-level to enable more verbose logs`. One-shot output and every JSON failure object stay as they are.
- The line grammar takes the binary as its `bin_name`, so a command's help reads `Usage: <bin> <command> ...`.
- `mcp::text` builds the text content item, public.
- Not added: a generic `exit_on_failure_unless` for the apps' MCP startup (a missing configuration back to `main` for the welcome, any other failure through `exit_on_failure`). Each app keeps a short match: the helper would only move the guard into a predicate and hide the exit behind an inverted name, a thin wrapper for two short call sites.
