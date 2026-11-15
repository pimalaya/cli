---
cairn: change
id: a-refused-call-names-its-argument
status: landed
created: 2026-11-15
---

# A refused call names its argument

## Why

The CLI alignment round of 2026-11-15 (pimalaya/.github cairn/changes/cli-alignment-2026-11-15, task L5, from a runtime MCP report) found that a tool call missing a required argument answers clap's usage text, `the following required arguments were not provided: <CARD-ID>` followed by a `Usage:` line, naming the argument as the command line spells it rather than as the tool's input schema does (`card-id`). A model reads the schema, not the command line. Ortie, which hand-rolls its tools, already answers ``Missing argument `uri` `` (ortie/src/mcp.rs:779).

## What

- `mcp::grammar::Grammar::argv` refuses a required argument missing or `null`, once the values given are checked: ``Missing argument `<name>` ``, or ``Missing arguments `<a>`, `<b>` ``, coded `usage-invalid`. The binaries get it without a change, the check running before clap.
- `mcp::grammar::Grammar::usage_error`, for the binaries to call where they call `error::usage_error` on clap's refusal of a call's argv: a missing argument as above, a refused value as ``Invalid value `<value>` for `<name>`: <reason>`` (the value parser's message or the possible values), and clap's message for any other refusal. It reads clap's error context, so the `mcp` feature enables clap's `error-context`, which every binary serving MCP enables already.
