---
cairn: delta
change: a-result-carries-its-errors
---

# Delta

## ADDED Requirements

### Requirement: An item error is the failure object of one item
(error) pimalaya-cli SHALL provide `error::ItemError<C>`, serialized as `{id, code, error, sources}`: `id` the item's id, `null` when unknown; `code` the code of the outermost coded error in the chain, `null` when none; `error` the message; `sources` the chain below it, outermost first. `ItemError::new(id, &err)` SHALL build it from an error, finding the code anywhere in the chain. It SHALL implement `JsonSchema` from the same declaration its serialization uses, every key required, `id` and `code` nullable, `code` typed by the binary's code enum. Its `Display` SHALL be one line, `<id>  <code>  <error>`, an absent id or code written `-`.

#### Scenario: A message the server answered for but the client could not read
- GIVEN an error coded `message-unreadable` under the context `Cannot read message 34`
- WHEN `ItemError::new(Some("34"), &err)` is serialized
- THEN it prints `{"id":"34","code":"message-unreadable","error":"Cannot read message 34","sources":[…]}`, and its `Display` reads `34  message-unreadable  Cannot read message 34`

## MODIFIED Requirements

### Requirement: Shared nouns keep one shape
(output) Every binary SHALL print an account row as `{name, default, backends}`, a backend check row as `{backend, ok, code, error}`, a collection row as `{id, name, description, color, default}`, a queued action's row id as `row`, an address as `{name, email}`, and the error of one item as `{id, code, error, sources}`.

### Requirement: A partial outcome prints its result
(output) A command that ran to its end with an outcome that is not a full success SHALL print its result and exit with a code of 3 or above, whose meaning the command documents; 2 SHALL stay clap's, for a refused command line. A `check` with a failed backend SHALL print every row and exit 3. A result whose `errors` list is not empty SHALL exit 3. pimalaya-cli SHALL provide a marker error a command returns once its result is printed, which the error report turns into the exit status without printing anything more.

### Requirement: Events precede the result
(output) A command MAY print events before its result, each as soon as the command observes what it reports. An event SHALL be an object whose root carries `event`, a kebab-case name; a result SHALL NOT carry `event` at its root. The last line SHALL be the failure object when the exit code is 1 and the result otherwise; a command printing only events has no result. A failed item SHALL never be a failure object: a command printing one result SHALL list it in its result's `errors`, an item error per failed item, `[]` when none failed, and a command reporting items as events MAY report it as an event field instead. With JSON output, each event SHALL be one flushed line; in text mode, events SHALL be written to stdout only when it is a terminal.

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

## REMOVED Requirements

None.
