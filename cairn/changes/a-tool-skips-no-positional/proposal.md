---
cairn: change
id: a-tool-skips-no-positional
status: landed
created: 2026-11-18
---

# A tool skips no positional

## Why

The CLI family analysis of 2026-11-15 (findings KIT-5 and CAL-5) found calendula's MCP tool `calendar_create` unable to take the `id` it listed: `{"id":"newcal","name":"My Cal"}` answered `usage-invalid`, "the argument '<NAME>' cannot be used multiple times". `Grammar::argv` writes the positionals after `--` (src/mcp/grammar.rs:249), and after `--` clap no longer skips a missing positional of a command set with `allow_missing_positional`: it gives the values to the last positional. Ruling D28 (calendula 14bbcc3) removed the one such command, so the limit is latent, but nothing stops a binary from building such a tool again, and the tool would list arguments it cannot take.

## What

- `Grammar::new` panics on a command set with `allow_missing_positional`, naming the tool: the mistake is the binary's, caught the first time its tool list is built (its tests, or the server's start), rather than answered as a usage error to every call.
- No grammar change: a command needing an optional positional before a required one takes the optional value as a flag, as calendula's `calendar create <NAME> [--id <ID>]` does.

Non-breaking for the family: no binary builds such a tool today; `Grammar::new` keeps its signature, documented with a `# Panics` section.
