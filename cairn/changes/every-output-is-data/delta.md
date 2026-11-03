---
cairn: delta
change: every-output-is-data
---

## ADDED Requirements

### Requirement: Every output is a described value
Every value a command prints SHALL be a named type implementing `Serialize` and `JsonSchema`, its `Display` writing the text form. pimalaya-cli SHALL provide no message wrapper. No field SHALL carry prose the binary composed; text a server or a file provides is data and SHALL travel as a string field.

### Requirement: A write answers a receipt
A command that writes SHALL print what it acted on, using these keys for these meanings: `id` (the item as it is after the write, `null` when queued), `ids` (several items, in the order acted on), `count` (how many items matched), `queued` (`true` when a sync engine applies the write later; no queue id), `response` (a protocol reply as sent), `warnings` (warnings attached to an accepted write), `path` (a file written). A receipt SHALL NOT be a generic map of fields. An action with nothing created or changed SHALL still name its target.

### Requirement: One command, one schema
Every command that prints SHALL register its schema under its command path. A command whose result varies SHALL print one type whose alternatives are told apart by their keys. pimalaya-cli SHALL provide a check over the clap command tree reporting every leaf without a key that the binary does not declare silent, and every key naming no leaf. The `completion`, `manual` and `json-schema` outputs SHALL be registered.

#### Scenario: A command printing data without a schema
- GIVEN a leaf whose output type is not in the registry and that is not declared silent
- WHEN the binary runs the registry check in its tests
- THEN the check reports the leaf's command path

### Requirement: Collections are named and present
A collection SHALL be printed even when empty, under a plural key naming its rows, never under a generic key. A `get` SHALL print the row its `list` prints.

### Requirement: Absent is null
A binary's own types SHALL print every key, `null` when the value is absent. An empty string SHALL mean the empty value, never an absent one. A provider payload SHALL keep its own omissions.

### Requirement: Paging resumes
A command that can stop before the end of a listing SHALL print `nextPage`, `null` once the listing is complete, and SHALL accept that value back through `--page-token`.

### Requirement: Values and times have one spelling
Values a binary defines SHALL be kebab-case; values a protocol defines SHALL keep its spelling. An instant a binary resolves SHALL be RFC 3339 in UTC with `Z`; a timestamp read from content SHALL keep its offset; a date SHALL be `YYYY-MM-DD`; a duration SHALL be integer seconds. A binary's own types SHALL NOT print epoch numbers.

### Requirement: Bytes are never raw under JSON
With JSON output, a command SHALL NOT write raw bytes to stdout and SHALL NOT decode bytes lossily. Binary content SHALL be `{data, size, mime}`, `data` in base64. vCard and iCalendar text SHALL be a string when it is valid UTF-8, base64 otherwise.

### Requirement: Protocol commands are raw-faithful
A command of a protocol namespace SHALL print the server's payload as received, under a designed collection key when it lists, without re-serializing it through a type that adds or drops fields.

### Requirement: Shared nouns keep one shape
Every binary SHALL print an account row as `{name, default, backends}`, a backend check row as `{backend, ok, code, error}`, the wizard's output as `{name, default, document}`, a collection row as `{id, name, description, color, default}`, a queued action's row id as `row`, and an address as `{name, email}`.

### Requirement: A partial outcome prints its result
A command that ran to its end with an outcome that is not a full success SHALL print its result and exit with a code of 2 or above, whose meaning the command documents. A `check` with a failed backend SHALL print every row and exit 2.

### Requirement: Events precede the result
A command MAY print events before its result. An event SHALL be an object whose root carries `event`, a kebab-case name; a result SHALL NOT carry `event` at its root. The last line SHALL be the failure object when the exit code is 1 and the result otherwise; a command printing only events has no result. A failed item SHALL be an event field, never a failure object. With JSON output, each event SHALL be one flushed line; in text mode, events SHALL be written to stdout only when it is a terminal.

#### Scenario: A sync piped to a script
- GIVEN a command registering events
- WHEN it runs with `--json` and stdout piped, then succeeds
- THEN stdout holds one line per event, each with an `event` key, then the result line without one, and the exit code is 0

#### Scenario: Text output redirected to a file
- GIVEN the same command without `--json`
- WHEN its stdout is redirected to a file
- THEN the file holds the result alone

### Requirement: Events are described apart
The registry SHALL hold an `events` map beside `schemas`, keyed by command path, each entry one type whose alternatives are the command's tagged events. `json-schema` SHALL write an entry as `<key>.events.json` with the `$id` `https://pimalaya.org/schemas/<bin>/v<version>/<key>.events.json`. The registry check SHALL cover both maps, report a result root holding `event` and an event alternative without it.

## MODIFIED Requirements

### Requirement: Failures carry an optional stable code
The error report SHALL print a `code` field beside `error`, `sources` and `backtrace`: the code of the outermost coded error in the chain, `null` when the chain holds none. Code values SHALL be kebab-case. The code vocabulary SHALL belong to each binary; pimalaya-cli SHALL provide only the mechanism. A binary SHALL have one failure object, and a failure folded into a result SHALL keep its code.

### Requirement: The failure object has a schema
The error report SHALL implement `JsonSchema`, describing the failure object with `code` required, nullable and typed by the binary's code enum, from the same declaration its serialization uses.
