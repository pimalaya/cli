---
cairn: change
id: shared-mcp-server
status: landed
created: 2026-11-11
---

# Shared MCP server

## Why

Ortie (src/mcp.rs) and Himalaya (src/mcp.rs) each answer Model Context Protocol requests over stdio with near-identical JSON-RPC code: -32700 for bad JSON, -32600 for a response object, notifications ignored, -32601 for an unknown method, `initialize` answering revision 2025-06-18 whatever the client asked, a printer capturing the one value a command prints. An audit of both found the same defects in each:

1. Off-by-one: both report `The command printed {values.len()} values` after `values.pop()`, so two values read `printed 1`.
2. A JSON array (a batch), a scalar or any other non-object gets no answer.
3. `jsonrpc` is never checked.
4. A non-UTF-8 line ends the server: `lines()` fails and the loop returns.
5. A failure before the loop (resolving the account) is printed by `main` on stdout, which must carry MCP messages only.
6. Himalaya sets `outputSchema` only when the root is literally `"type": "object"`, which is what MCP takes; the registry's object-root check also accepts `anyOf` of objects, so it cannot decide alone.
7. The client's `protocolVersion` is never read.

Himalaya also generates each tool's input schema from its clap command and turns a call back into an argv parsed by that same command. That half is generic; the tool tables, the read/write/send gates, the attachments and the resources are Himalaya's.

## What

Behind a new `mcp` feature (`terminal`, no new dependency), src/mcp.rs:

- `ServerInfo { name, version, instructions }`, `PROTOCOL_VERSIONS` (2025-06-18 alone, latest first).
- `read(line, info, tools) -> Incoming` answering one line of bytes: `Reply(Value)` (-32700 for non-UTF-8 or bad JSON, -32600 with a null id for a non-object or a batch, -32600 for a wrong `jsonrpc`, a bad `id` or no `method`, -32601, `ping`, `initialize` echoing a supported client version or answering the latest, `tools/list`), `Call(ToolCall)`, or `Ignore` (a notification, a blank line). `tools/call` is checked first: -32602 for bad params, a missing or unlisted tool, non-object arguments; an argument the tool's `inputSchema` does not declare, when it sets `additionalProperties: false`, is refused as an `isError` result naming the declared ones.
- `ToolCall { id, name, arguments }` with `respond(result) -> Value`; `ToolResult { content, structured_content, is_error }` with `success`, `failure`, `error::<C>` and `printed::<C>(printer, result)`: events first as text items, the value as text and as `structuredContent` when an object, a partial outcome keeping its result with `isError: true`, a failure or a wrong value count as `isError: true` with the failure object as text.
- `answer(line, info, tools, run) -> Option<Value>`, one line to its response; `send(output, message)`, one flushed line; `serve(info, tools, run)`, the blocking loop over stdin for a server without background events; `spawn_stdin_reader(events, wrap)`, a thread feeding stdin lines as `Input::Line(bytes)` then `Input::Eof` into the binary's own channel, for a server with background events. Both claim stdin (`session::claim_stdin`).
- `exit_on_failure::<C, T>(result)`: the failure report as text on stderr and exit status 1, so a server failing at startup leaves stdout clean.
- `tool_output_schema(schema)`: the schema when its root is typed `object`, else nothing.

src/mcp/grammar.rs: `Grammar`, Himalaya's generic half: a built clap command's arguments as properties (`exposes`, the argument schema, `minItems`/`maxItems` for an argument taking one value), `retain` for the binary's gates, `fixed` words, `input_schema`, and `argv` (options glued `--long=value`, a true flag `--long`, fixed words, then `--` and the positionals in order). Himalaya's `max-width` exclusion goes: no table is drawn any more.

In src/printer.rs, behind `terminal`: `CapturePrinter { values, events }` keeping what a command prints as JSON, `into_single` failing with the true count.

## Not in scope

The moves of Ortie and Himalaya onto the module, each a change in its own repository. MCP revisions other than 2025-06-18: 2025-03-26 requires batches, which the server refuses, and later ones are unchecked.
