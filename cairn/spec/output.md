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
A command that writes SHALL print what it acted on, using these keys for these meanings: `id` (the item as it is after the write; for a queued write, the id the store gives the item, its link id, never a placeholder such as `q<id>`, `0` or `""`; `null` only when the backend gives none), `ids` (several items, in the order acted on), `count` (how many items matched), `queued` (`true` when a sync engine applies the write later; no queue id), `response` (a protocol reply as sent), `warnings` (warnings attached to an accepted write), `path` (a file written). A receipt SHALL NOT be a generic map of fields. An action with nothing created or changed SHALL still name its target.

### Requirement: A file written is named with its size
A command writing to a file what it would otherwise print SHALL print `output::SavedOutput`, `{path, size}`, `path` the file written and `size` the number of bytes written. Its text SHALL read `Saved <size> bytes to <path>`.

#### Scenario: A part saved to a file
- GIVEN a command writing 42 bytes to `/tmp/a.pdf`
- WHEN it prints its receipt
- THEN the text reads `Saved 42 bytes to /tmp/a.pdf` and the JSON `{"path":"/tmp/a.pdf","size":42}`

### Requirement: One command, one schema
Every command that prints SHALL register its schema under its command path. A command whose result varies SHALL print one type whose alternatives are told apart by their keys. pimalaya-cli SHALL provide a check over the clap command tree reporting every runnable command (a leaf, or a command whose subcommand is optional) without a key that the binary does not declare silent, and every key naming no runnable command, the failure object's `<bin>-error` and the item error's `<bin>-item-error` aside. The `completion`, `manual` and `json-schema` outputs SHALL be registered by the registry itself, at the root of the command tree. Schemas SHALL describe a value as printed (the serialize contract), so a key the type always prints is required, `null` or not.

#### Scenario: A command printing data without a schema
- GIVEN a leaf whose output type is not in the registry and that is not declared silent
- WHEN the binary runs the registry check in its tests
- THEN the check reports the leaf's command path

### Requirement: Collections are named and present
A collection SHALL be printed even when empty, under a plural key naming its rows, never under a generic key. A read SHALL print the row its list prints, plus what only a read carries (the item's content), with the item's `id` and `etag`, `etag` `null` where the backend versions no item. A read of one file holding several items, such as an iCalendar file holding a recurring event and its overrides, MAY print their rows as a collection (`events: [...]`) beside the file's `id` and `etag`. A listing MAY carry `total`, the number of items the whole listing holds: the key is optional across the binaries, and paging SHALL NOT depend on it.

### Requirement: Absent is null
A binary's own types SHALL print every key, `null` when the value is absent. An empty string SHALL mean the empty value, never an absent one. A provider payload SHALL keep its own omissions.

### Requirement: Paging resumes
A command that can stop before the end of a listing SHALL print `nextPage`, `null` once the listing is complete, and SHALL accept that value back through `--page-token`. A ranked search such as `pimdir search` SHALL page the same way, never by a `more` flag. Every listing of a protocol namespace that can stop before the end SHALL page the same way: `nextPage` SHALL be the provider's cursor as a string, as the provider gives it (Google's `nextPageToken`, Microsoft Graph's `@odata.nextLink`), `null` once the provider gives none, and `--page-token` SHALL take it back as is; a protocol giving no cursor (JMAP pages by position) SHALL print, as a string, the value the binary resumes from. The provider's cursor key SHALL be lifted into `nextPage`, not printed beside it; the rest of the payload SHALL stay raw, a sync token such as `nextSyncToken` or `@odata.deltaLink` included. In text, the cursor SHALL print as a `Next page: <cursor>` line after the listing, one blank line apart when the listing printed anything.

### Requirement: Values and times have one spelling
Values a binary defines SHALL be kebab-case; values a protocol defines SHALL keep its spelling. An instant a binary resolves SHALL be RFC 3339 in UTC with `Z`; a timestamp read from content SHALL keep its offset; a date SHALL be `YYYY-MM-DD`; a duration SHALL be integer seconds. A binary's own types SHALL NOT print epoch numbers.

### Requirement: Bytes are never raw under JSON
With JSON output, a command SHALL NOT write raw bytes to stdout and SHALL NOT decode bytes lossily. Binary content SHALL be `{data, size, mime}`, `data` in base64. vCard and iCalendar content SHALL be a string beside `encoding`: the text when its bytes are valid UTF-8, `encoding` then `utf-8`, and their base64 (RFC 4648 standard alphabet, padded) otherwise, `encoding` then `base64`.

### Requirement: Protocol commands are raw-faithful
A command of a protocol namespace whose payload is JSON SHALL print the server's payload as received, under a designed collection key when it lists, without re-serializing it through a type that adds or drops fields. The provider's paging cursor alone SHALL move, lifted into `nextPage` (*Paging resumes*).

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
Every binary SHALL print an account row as `{name, default, backends, notBuilt}`, a backend check row as `{backend, ok, code, error}`, a file written as `{path, size}`, an address book or calendar row as `{id, name, description, color, default}`, a mailbox row as `{id, name, role, total, unread}`, a queued action's row id as `row`, an address as `{name, email}`, and the error of one item as `{id, code, error, sources}`.

### Requirement: A queue row keeps one shape
`pimdir queue list` SHALL print each row it lists, pending or parked, as `{row, kind, collection, linkId, itemId, producer, queuedAt, attempts, unconfirmed, error}` plus its domain's label key: `subject` for mail, `fullName` for contacts, `summary` for calendars. Every key SHALL print, `null` or not. `row` SHALL be an integer, the queue row id `pimdir queue show` and `pimdir queue cancel` take. `kind` SHALL be a string, the row's action as the store holds it (pimdir STORAGE §15.3), read from io-pimdir (`PimdirAction::kind` for a pending row, `PimdirParkedAction::action` for a parked one): the store's own `add`, `set-flags`, `remove`, `move`, `copy`, `update` and `set-performer`, the intents of `io_pimdir::intent` (`submit`, `calendar-reply`, `calendar-cancel`, `collection-create`, `collection-rename`, `collection-delete`), all kebab-case, or an application's own (`x-`) as stored; it SHALL NOT be re-spelled. `collection` SHALL be a string, the id of the collection the row is queued on, as the binary's collection listing prints it. `linkId` SHALL be a string or `null`: the link id of the item the row creates, the `id` its write's receipt printed, `null` when the row creates no item or the store gives it none. `itemId` SHALL be a string or `null`: the id of the stored item the row addresses (an update, a flag change, a move, a copy, a removal, an intent on an item), as the binary's item listing prints it, `null` when the row addresses no stored item, a creation among them. `producer` SHALL be a string, the process that staged the row. `queuedAt` SHALL be a string, the instant the store stamped the row, RFC 3339 in UTC with `Z`. `attempts` SHALL be an integer from 0, how many times the sync engine tried the row. `unconfirmed` SHALL be a boolean, `true` when the row's outcome is unknown (pimdir STORAGE §15.2), its `error` then beginning `unconfirmed: `. `error` SHALL be a string or `null`: `null` while the row is pending, the store's text once it is parked, a text beginning `unconfirmed: ` marking a row whose outcome is unknown (pimdir STORAGE §15.2). The label SHALL be a string or `null`, read from the body the row pins or the item it addresses, `null` when there is none to read.

#### Scenario: A pending creation and a parked update
- GIVEN a contacts binary whose pimdir queue holds a pending `add` of the card `Jane Doe` and a parked `update` of another card
- WHEN `pimdir queue list` runs with `--json`
- THEN both rows carry the ten shared keys and `fullName`, the `add` with its `linkId`, `itemId` `null` and `error` `null`, the `update` with `linkId` `null`, the card it updates as `itemId` and its `error` text

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

### Requirement: A long-lived command streams events
A command that runs until stopped (a watch) or for long (a sync) SHALL print one event line per event while it runs, then exactly one line without `event`: its result, exiting 0, 3 or the binary's own partial status, or the failure object, exiting 1. An error the command recovers from SHALL be an event carrying `code`, `error` and `sources`, as an item error does. On SIGINT or SIGTERM it SHALL stop cleanly and print its result: a watch exits 0, a bounded run its incomplete status. The text form of that result SHALL say the command stopped (`Watch stopped`), so a person reading a pipe knows it ended. pimalaya-cli SHALL provide `signal::Shutdown`, a cloneable token whose `install` makes the first termination signal request the shutdown rather than end the process, a second one ending it with 130, and the spinner SHALL defer to it. Such a command SHALL write its text events to stdout even when it is no terminal, through `StdoutPrinter::streaming`, since a status bar reads them from a pipe.

#### Scenario: A watch piped to a status bar
- GIVEN a watch command built on a streaming printer
- WHEN it runs without `--json`, stdout piped, and receives SIGTERM
- THEN the pipe received one text line per event as it came, then the result, and the exit code is 0

### Requirement: Events are described apart
The registry SHALL hold an `events` map beside `schemas`, keyed by command path, each entry one type whose alternatives are the command's tagged events. `json-schema` SHALL write an entry as `<key>.events.json` with the `$id` `https://pimalaya.org/schemas/<bin>/v<version>/<key>.events.json`. The registry check SHALL cover both maps, report a result root holding `event` and an event alternative without it. A reference carrying keywords of its own, the newtype variant of an internally tagged enum, SHALL be read as one alternative by those keywords.

### Requirement: A capture printer counts its values
`printer::CapturePrinter` SHALL keep each value and each event a command prints as JSON, and SHALL report itself as printing JSON. `into_single` SHALL answer the value when exactly one was printed, and fail with `The command printed <n> values, one expected` otherwise, `<n>` the number printed.

#### Scenario: Two values
- GIVEN a command printing two values
- WHEN the single value is taken
- THEN it fails with `The command printed 2 values, one expected`
