---
cairn: delta
change: repl-and-mcp-polish
---

## ADDED Requirements

### Requirement: Help names the binary
The help of a line SHALL render its usage line with the binary name before the command, as the command line does.

#### Scenario: A command's help
- GIVEN a session of the binary `app`
- WHEN the line `help token show` or `token show --help` is read
- THEN the usage line reads `Usage: app token show [OPTIONS]`

### Requirement: A text item has one builder
`mcp::text` SHALL build the text content item `{"type": "text", "text": <text>}`, the one a `ToolResult` holds for each event, value and failure object, so a binary adding an item of its own does not spell it.

## MODIFIED Requirements

### Requirement: Each line answers through its own printer
Each line SHALL print its result through a printer of its own, JSON when the session's `--json` or the line's is set, the following lines keeping the session's format. The loop SHALL add nothing after a result, every text output ending with its own newline, and SHALL flush stdout after each line. A failed command SHALL print the failure object of the binary's error codes through that printer on stdout, whether stdin is a terminal or not, its text form hinting `Restart the session with --log-level to enable more verbose logs` while debug logs are off, since a line cannot set `--log-level`. A command ending with a partial outcome SHALL print nothing after its result. A line clap cannot parse SHALL print clap's rendering on stderr when stdin is a terminal, and the failure object on stdout otherwise. When stdin is a terminal, the prompt `<bin>> ` SHALL render on stderr before each line.

#### Scenario: A line's JSON flag in a text session
- GIVEN a text session
- WHEN the lines `token inspect --json` and `token show` are read
- THEN the first prints its result as JSON and the second as text

#### Scenario: A failure in a text session
- GIVEN a text session started without `--log-level`
- WHEN a line fails
- THEN its failure ends with the hint `Restart the session with --log-level to enable more verbose logs`

## REMOVED Requirements
