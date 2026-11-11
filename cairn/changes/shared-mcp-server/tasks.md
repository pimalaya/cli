---
cairn: tasks
change: shared-mcp-server
---

- [x] `mcp` feature (`terminal`, no new dependency)
- [x] src/printer.rs: `CapturePrinter` and `into_single`
- [x] src/mcp.rs: `ServerInfo`, `read`, `Incoming`, `ToolCall`, `ToolResult`, `answer`, `send`, `serve`, `spawn_stdin_reader`, `exit_on_failure`, `tool_output_schema`
- [x] src/mcp/grammar.rs: `Grammar` from Himalaya's tool schema and argv code
- [x] Tests: parse error, non-UTF-8, invalid request, batch and scalar, wrong `jsonrpc`, bad id, unknown method, notification, ping, initialize with a supported and an unsupported version, tools/list, tools/call checks, unknown argument, tool results (success, events, partial, failure, wrong count), capture printer with 0, 1 and 2 values, output schema root, grammar schemas and argv parsed back, the blocking loop and the reader over a cursor
- [x] src/lib.rs header, README, CONTRIBUTING, CHANGELOG
- [x] fmt, clippy `-D warnings` per feature, tests, doc
- [x] Delta folded into a new mcp spec and the output spec, log written
