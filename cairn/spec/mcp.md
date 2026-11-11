---
cairn: spec
capability: mcp
status: current
---

# MCP server

The `mcp` feature gives a binary a Model Context Protocol server over stdio: one JSON-RPC message per line, stdout carrying protocol messages only. pimalaya-cli answers the protocol itself (errors, `initialize`, `ping`, `tools/list`) and checks each tool call, the binary runs the call and answers a tool result, usually built from what its command printed, and a tool's input may be derived from the clap command it runs. Like the REPL, the server claims its stdin (cairn/spec/repl.md).

### Requirement: One JSON-RPC message per line
`mcp::read` SHALL answer one line of bytes: -32700 with a null `id` when it is not UTF-8 or not JSON; -32600 with a null `id` when it is not a JSON object, a batch array included; -32600 when its `jsonrpc` is not `"2.0"`, its `id` is neither a string nor a number, or it has no string `method`, with its `id` when valid and `null` otherwise. A blank line and a notification (a `method` without `id`) SHALL get no answer. A request SHALL get -32601 for an unknown method, `{}` for `ping`, and the tools the server lists for `tools/list`.

#### Scenario: A batch
- GIVEN a server
- WHEN the line `[{"jsonrpc":"2.0","id":1,"method":"ping"}]` is read
- THEN it answers -32600 with `id` `null`

#### Scenario: Bytes that are not UTF-8
- GIVEN a server
- WHEN a line holding the byte 0xFF is read
- THEN it answers -32700 with `id` `null` and reads the next line

### Requirement: The protocol version is negotiated
`initialize` SHALL answer the client's `protocolVersion` when the server supports it, and the latest version it supports otherwise; 2025-06-18 SHALL be the one supported. The answer SHALL carry `capabilities.tools.listChanged` `false`, `serverInfo` with the binary's `name` and `version`, and `instructions` when the binary gives some.

#### Scenario: A newer client
- GIVEN a server
- WHEN `initialize` asks for `2099-01-01`
- THEN it answers `protocolVersion` `2025-06-18`

### Requirement: A call is checked before it runs
`tools/call` SHALL answer -32602 when `params` is not an object, when `name` is missing or names no listed tool, and when `arguments` is neither an object nor absent or `null`. When the tool's `inputSchema` sets `additionalProperties` to `false`, an argument it does not declare SHALL be refused as a tool result with `isError: true` and the failure object `Unknown argument <key>, <tool> takes <properties>`, or `takes none`. Any other call SHALL be handed to the binary with its arguments as an object.

### Requirement: A tool result is what the command printed
`mcp::ToolResult::printed` SHALL turn what a command printed into a `printer::CapturePrinter` into a tool result: each event first, one text item each, then the single value as a text item, also as `structuredContent` when it is an object. A command ending with a partial outcome SHALL keep that result and set `isError: true`. A failed command, or one that printed a number of values other than one, SHALL give `isError: true` with the failure object as text and no `structuredContent`.

### Requirement: An output schema has an object root
`mcp::tool_output_schema` SHALL answer a JSON Schema as a tool's `outputSchema` only when its root is typed `object`, and nothing for a root that is a reference, an `anyOf` or a `oneOf`, even of objects.

### Requirement: Stdout carries protocol messages only
`mcp::serve` and `mcp::spawn_stdin_reader` SHALL claim stdin before reading it. `mcp::exit_on_failure` SHALL print a failure as text on stderr and exit with status 1, or with the status of a partial outcome, so a server failing before or during its loop writes nothing on stdout.

### Requirement: A tool's input is its command's grammar
`mcp::grammar::Grammar` SHALL describe a built clap command as one property per argument, named after its long flag or the lowercased value name of a positional, leaving out global, hidden, help and version arguments, arguments taking a path (a path value hint or a `PATH` value name), the ones a binary filters out, and the ones its fixed words set. A flag SHALL be a boolean, a repeatable argument an array of strings, possible values an `enum`, the help a `description`; an argument the binary takes one value of SHALL be an array with `minItems` and `maxItems` 1. Required arguments, repeatable positionals needing at least one value and arguments taking one value SHALL be `required`, an optional positional taking one value SHALL NOT, and the schema SHALL set `additionalProperties: false`. `argv` SHALL write the command path, `--<long>=<value>` for each option value, `--<long>` for a true flag, the fixed words, then `--` and the positionals in their order, and SHALL refuse an undeclared key, a value of the wrong type and a count other than one for an argument taking one.

#### Scenario: A value starting with a dash
- GIVEN the command `attachment download` with `--mailbox`, the flag `--stdout` fixed, and the positionals `message-id` and `attachment-id`
- WHEN the arguments `mailbox` `-odd`, `message-id` `m2` and `attachment-id` `["3"]` are turned into an argv
- THEN it reads `attachment download --mailbox=-odd --stdout -- m2 3`, and `stdout` is no property
