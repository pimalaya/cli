---
cairn: spec
capability: output
status: current
---

# Output

The printer writes a command's events and result to stdout, as text or JSON, or keeps them as JSON for a caller handing them on, and the JSON Schema command describes what the JSON looks like. Every output is data: a described value, never a sentence.

### Requirement: Piped JSON is newline-delimited
When JSON output is enabled and stdout is not a terminal, the printer SHALL write each value as compact JSON followed by a newline, and SHALL flush after it, so a process emitting several values produces one parseable JSON value per line.

### Requirement: Schemas are stamped with the contract version
`JsonSchemaCommand` SHALL set each generated schema's `$id` to `https://pimalaya.org/schemas/<bin>/v<version>/<key>.json`, where `<version>` is the binary's major version, or `0.<minor>` before 1.0. The contract version is the binary's major version; payloads themselves SHALL carry no version field.

### Requirement: Schema roots can be checked for objects
pimalaya-cli SHALL provide a check returning the registry keys whose schema root is not a JSON object (typed `object`, a reference to an object definition, or an `anyOf` or `oneOf` whose alternatives all are), so each binary can assert that every payload is usable as structured tool output.

### Requirement: Every output is a described value
Every value a command prints SHALL be a named type implementing `Serialize` and `JsonSchema`, its `Display` writing the text form. pimalaya-cli SHALL provide no message wrapper. No field SHALL carry prose the binary composed; text a server or a file provides is data and SHALL travel as a string field.

### Requirement: A write answers a receipt
A command that writes SHALL print what it acted on, using these keys for these meanings: `id` (the item as it is after the write, `null` when queued or when the backend returns none), `ids` (several items, in the order acted on), `count` (how many items matched), `queued` (`true` when a sync engine applies the write later; no queue id), `response` (a protocol reply as sent), `warnings` (warnings attached to an accepted write), `path` (a file written). A receipt SHALL NOT be a generic map of fields. An action with nothing created or changed SHALL still name its target.

### Requirement: One command, one schema
Every command that prints SHALL register its schema under its command path. A command whose result varies SHALL print one type whose alternatives are told apart by their keys. pimalaya-cli SHALL provide a check over the clap command tree reporting every runnable command (a leaf, or a command whose subcommand is optional) without a key that the binary does not declare silent, and every key naming no runnable command, the failure object's `<bin>-error` aside. The `completion`, `manual` and `json-schema` outputs SHALL be registered by the registry itself, at the root of the command tree. Schemas SHALL describe a value as printed (the serialize contract), so a key the type always prints is required, `null` or not.

#### Scenario: A command printing data without a schema
- GIVEN a leaf whose output type is not in the registry and that is not declared silent
- WHEN the binary runs the registry check in its tests
- THEN the check reports the leaf's command path

### Requirement: Collections are named and present
A collection SHALL be printed even when empty, under a plural key naming its rows, never under a generic key. A `get` SHALL print the row its `list` prints.

### Requirement: Absent is null
A binary's own types SHALL print every key, `null` when the value is absent. An empty string SHALL mean the empty value, never an absent one. A provider payload SHALL keep its own omissions.

### Requirement: Paging resumes
A command that can stop before the end of a listing SHALL print `nextPage`, `null` once the listing is complete, and SHALL accept that value back through `--page-token`. In text, the cursor SHALL print as a `Next page: <cursor>` line after the listing, one blank line apart when the listing printed anything.

### Requirement: Values and times have one spelling
Values a binary defines SHALL be kebab-case; values a protocol defines SHALL keep its spelling. An instant a binary resolves SHALL be RFC 3339 in UTC with `Z`; a timestamp read from content SHALL keep its offset; a date SHALL be `YYYY-MM-DD`; a duration SHALL be integer seconds. A binary's own types SHALL NOT print epoch numbers.

### Requirement: Bytes are never raw under JSON
With JSON output, a command SHALL NOT write raw bytes to stdout and SHALL NOT decode bytes lossily. Binary content SHALL be `{data, size, mime}`, `data` in base64. vCard and iCalendar text SHALL be a string when it is valid UTF-8, base64 otherwise.

### Requirement: Protocol commands are raw-faithful
A command of a protocol namespace whose payload is JSON SHALL print the server's payload as received, under a designed collection key when it lists, without re-serializing it through a type that adds or drops fields.

### Requirement: Structured when understood
A reply of a text protocol, or a value of a local store, SHALL print as a structured value when the binary parses it, and as its text otherwise, in an alternative told apart from the structured ones by its keys. No field SHALL hold a structured value in one output and a string in another.

#### Scenario: A response the binary cannot parse
- GIVEN a raw IMAP exchange whose reply holds a response imap-codec cannot decode
- WHEN the command prints it under `--json`
- THEN that response prints as an object holding its text, its keys distinct from those of a parsed response, and the other responses print parsed

### Requirement: Protocol replies are events
Inside a protocol namespace, a response the server sends before a command's completion SHALL be an event, and the completion SHALL be the result. A command SHALL emit events at the granularity it observes; a binary SHALL NOT be required to observe each response of a typed exchange.

### Requirement: A protocol refusal is a result
Inside a protocol namespace, a completion the server refuses (an IMAP `NO` or `BAD`, an SMTP 4xx or 5xx reply, a ManageSieve `NO`, an HTTP 4xx or 5xx carrying the provider's error body) SHALL print as the result and exit 3. A command that gets no completion (a network, TLS or decoding failure, a local error) SHALL print the failure object and exit 1. Outside protocol namespaces, a refusal SHALL be a coded failure.

### Requirement: Shared nouns keep one shape
Every binary SHALL print an account row as `{name, default, backends}`, a backend check row as `{backend, ok, code, error}`, a collection row as `{id, name, description, color, default}`, a queued action's row id as `row`, an address as `{name, email}`, and the error of one item as `{id, code, error, sources}`.

### Requirement: A partial outcome prints its result
A command that ran to its end with an outcome that is not a full success SHALL print its result and exit with a code of 3 or above, whose meaning the command documents; 2 SHALL stay clap's, for a refused command line. A `check` with a failed backend SHALL print every row and exit 3. A result whose `errors` list is not empty SHALL exit 3. pimalaya-cli SHALL provide a marker error a command returns once its result is printed, which the error report turns into the exit status without printing anything more.

### Requirement: Events precede the result
A command MAY print events before its result, each as soon as the command observes what it reports. An event SHALL be an object whose root carries `event`, a kebab-case name; a result SHALL NOT carry `event` at its root. The last line SHALL be the failure object when the exit code is 1 and the result otherwise; a command printing only events has no result. A failed item SHALL never be a failure object: a command printing one result SHALL list it in its result's `errors`, an item error per failed item, `[]` when none failed, and a command reporting items as events MAY report it as an event field instead. With JSON output, each event SHALL be one flushed line; in text mode, events SHALL be written to stdout only when it is a terminal.

#### Scenario: A sync piped to a script
- GIVEN a command registering events
- WHEN it runs with `--json` and stdout piped, then succeeds
- THEN stdout holds one line per event, each with an `event` key, then the result line without one, and the exit code is 0

#### Scenario: Text output redirected to a file
- GIVEN the same command without `--json`
- WHEN its stdout is redirected to a file
- THEN the file holds the result alone

#### Scenario: A listing with one item that failed
- GIVEN a listing of three items, one of which the backend could not read
- WHEN it runs with `--json`
- THEN it prints the two items read and an `errors` list holding the third's item error, and the exit code is 3

### Requirement: Events are described apart
The registry SHALL hold an `events` map beside `schemas`, keyed by command path, each entry one type whose alternatives are the command's tagged events. `json-schema` SHALL write an entry as `<key>.events.json` with the `$id` `https://pimalaya.org/schemas/<bin>/v<version>/<key>.events.json`. The registry check SHALL cover both maps, report a result root holding `event` and an event alternative without it.

### Requirement: A capture printer counts its values
`printer::CapturePrinter` SHALL keep each value and each event a command prints as JSON, and SHALL report itself as printing JSON. `into_single` SHALL answer the value when exactly one was printed, and fail with `The command printed <n> values, one expected` otherwise, `<n>` the number printed.

#### Scenario: Two values
- GIVEN a command printing two values
- WHEN the single value is taken
- THEN it fails with `The command printed 2 values, one expected`
