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

### Requirement: Gates list what a tool may do
A server started without a gate flag SHALL list only the tools that read. `--allow-write` SHALL list the tools that change the account's data, and `--allow-send` the tools that act beyond the account, such as sending mail, an invitation or an auto-reply. A binary MAY add a gate of its own for a narrower destructive class, as ortie's `--allow-revoke` for revoking a token at its authorization server. An argument that makes a listed tool write or send SHALL be listed only under the gate of what it does. A tool or an argument a gate leaves out SHALL be refused as any tool or argument not listed.

#### Scenario: No gate
- GIVEN a binary whose tools read a message, with a `seen` argument flagging it, and send one
- WHEN its server starts without a gate flag
- THEN `tools/list` lists the reading tool alone, without `seen`

### Requirement: Annotations describe the tool as listed
A tool's `readOnlyHint` SHALL be `true` exactly when the tool, with the arguments it lists, changes no data. Its `openWorldHint` SHALL be `true` exactly when it acts beyond the account's own data, reaching other people (sending mail, an invitation, an auto-reply); it SHALL NOT mean that the tool reaches the network, which every remote backend does. A gate SHALL move neither hint by itself: a tool listing the same arguments SHALL carry the same hints whichever gates are open.

#### Scenario: A remote listing and an invitation
- GIVEN a tool listing the events of a remote calendar and a tool inviting attendees
- WHEN the tools are listed
- THEN the first carries `readOnlyHint` `true` and `openWorldHint` `false`, the second `readOnlyHint` `false` and `openWorldHint` `true`

### Requirement: A call is checked before it runs
`tools/call` SHALL answer -32602 when `params` is not an object, when `name` is missing or names no listed tool, and when `arguments` is neither an object nor absent or `null`. When the tool's `inputSchema` sets `additionalProperties` to `false`, an argument it does not declare SHALL be refused as a tool result with `isError: true` and the failure object coded `usage-invalid`, `Unknown argument <key>, <tool> takes <properties>`, or `takes none`. Any other call SHALL be handed to the binary with its arguments as an object.

### Requirement: A tool result is what the command printed
`mcp::ToolResult::printed` SHALL turn what a command printed into a `printer::CapturePrinter` into a tool result: each event first, one text item each, then the single value as a text item, also as `structuredContent` when it is an object. A command ending with a partial outcome SHALL keep that result and set `isError: true`. A failed command, or one that printed a number of values other than one, SHALL give `isError: true` with the failure object as text and no `structuredContent`.

### Requirement: A text item has one builder
`mcp::text` SHALL build the text content item `{"type": "text", "text": <text>}`, the one a `ToolResult` holds for each event, value and failure object, so a binary adding an item of its own does not spell it.

### Requirement: An output schema has an object root
`mcp::tool_output_schema` SHALL answer a JSON Schema as a tool's `outputSchema` only when its root is typed `object`, and nothing for a root that is a reference, an `anyOf` or a `oneOf`, even of objects.

### Requirement: Stdout carries protocol messages only
`mcp::serve` and `mcp::spawn_stdin_reader` SHALL claim stdin before reading it. `mcp::exit_on_failure` SHALL print a failure as text on stderr and exit with status 1, or with the status of a partial outcome, so a server failing before or during its loop writes nothing on stdout.

### Requirement: A tool's input is its command's grammar
`mcp::grammar::Grammar` SHALL describe a built clap command as one property per argument, named after its long flag or the lowercased value name of a positional, leaving out global, hidden, help and version arguments, arguments taking a path (a path value hint or a `PATH` value name), the ones a binary filters out, and the ones its fixed words set. A flag SHALL be a boolean, a repeatable argument an array, an argument the binary takes one value of and any other argument a scalar; a value SHALL be an `integer` when its value parser yields a Rust integer type and a `string` otherwise, possible values an `enum`. The help SHALL be the `description`, unless the binary replaces it, and a long flag of a property written between backquotes in it SHALL read as the property's name. Required arguments, repeatable positionals needing at least one value and arguments taking one value SHALL be `required`, an optional positional taking one value SHALL NOT, and the schema SHALL set `additionalProperties: false`. `argv` SHALL write the command path, `--<long>=<value>` for each option value, `--<long>` for a true flag, the fixed words, then `--` and the positionals in their order. It SHALL take a lone value where an array is expected as an array of one, and an array of one for an argument taking one value, and SHALL refuse an undeclared key, a value of the wrong type, a count other than one for an argument taking one and, once the values given are checked, a required argument missing or `null`, as ``Missing argument `<name>` `` (``Missing arguments `<a>`, `<b>` `` for several), each a failure coded `usage-invalid`.

#### Scenario: A value starting with a dash
- GIVEN the command `attachment download` with `--mailbox`, the flag `--stdout` fixed, and the positionals `message-id` and `attachment-id`, the latter taking one value
- WHEN the arguments `mailbox` `-odd`, `message-id` `m2` and `attachment-id` `"3"` are turned into an argv
- THEN it reads `attachment download --mailbox=-odd --stdout -- m2 3`, and `stdout` is no property

#### Scenario: A query given as one string
- GIVEN a command whose positional `query` takes many words
- WHEN the argument `query` is `"subject hello order by date desc"`
- THEN the argv carries it as one word after `--`, as `["subject hello order by date desc"]` would

#### Scenario: A page number
- GIVEN an option `--page` parsed by `value_parser!(u32).range(1..)`
- WHEN the input schema is built
- THEN `page` is typed `integer`

### Requirement: A refused call names its arguments
`Grammar::usage_error` SHALL turn clap's refusal of a call's argv into a failure coded `usage-invalid` naming each argument as the tool's input does, never clap's usage text, whether clap renders it `--<long>`, `<NAME>` or, for an optional positional, `[NAME]`: a missing argument as ``Missing argument `<name>` `` (``Missing arguments `<a>`, `<b>` `` for several), a refused value as ``Invalid value `<value>` for `<name>`: <reason>``, the reason being the value parser's message or ``the possible values are `<a>`, `<b>` ``. A refusal of another kind (a conflict between arguments), or naming an argument that is no property, SHALL keep clap's message as `error::usage_error` words it.

#### Scenario: A page that is no number
- GIVEN a tool whose `page` argument is parsed by `value_parser!(u32).range(1..)`
- WHEN a call gives `page` `"x"` and clap refuses its argv
- THEN the tool result is an error whose failure object is coded `usage-invalid` and reads ``Invalid value `x` for `page`: invalid digit found in string``

#### Scenario: A refused value of an optional positional
- GIVEN a tool whose optional positional `transport` takes `socket` or `tcp`, which clap renders `[TRANSPORT]`
- WHEN a call gives `transport` `"udp"` and clap refuses its argv
- THEN the failure object reads ``Invalid value `udp` for `transport`: the possible values are `socket`, `tcp` ``, coded `usage-invalid`

### Requirement: A tool's command skips no positional
`mcp::grammar::Grammar::new` SHALL panic on a command set with clap's `allow_missing_positional`, naming the tool, since its `argv` writes the positionals after `--`, where clap no longer skips a missing positional of such a command. A binary SHALL take an optional value standing before a required positional as a flag instead.

#### Scenario: An optional positional before a required one
- GIVEN the command `list [CALENDAR] <ID>` set with `allow_missing_positional`
- WHEN a binary builds its grammar
- THEN it panics with a message naming `allow_missing_positional`, before any tool is listed
