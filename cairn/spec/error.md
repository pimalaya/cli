---
cairn: spec
capability: error
status: current
---

# Error

A failed command prints its error report to stdout, as text or JSON, and exits with status 1. A command that ran to its end with an outcome short of full success prints its result instead and exits 3 or above. A command line the grammar refuses exits 2. A command acting on several items lists the failure of each single item as an item error in its result's `errors`.

### Requirement: Failures carry an optional stable code
The error report SHALL print a `code` field beside `error`, `sources` and `backtrace`: the code of the outermost coded error in the chain, `null` when the chain holds none. Code values SHALL be kebab-case. The code vocabulary SHALL belong to each binary; pimalaya-cli SHALL provide only the mechanism, and the codes it raises itself, `usage-invalid` alone, which the failure object's schema admits beside the binary's. A binary SHALL have one failure object, and a failure folded into a result SHALL keep its code.

### Requirement: The failure object has a schema
The error report SHALL implement `JsonSchema`, describing the failure object with `code` required, nullable and typed by the binary's code enum, from the same declaration its serialization uses.

### Requirement: A refused command line exits 2
pimalaya-cli SHALL provide `clap::parse`, which a binary calls in place of clap's `Parser::parse`. Help and version SHALL print and exit as clap does. A command line the grammar refuses (an unknown flag or command, a missing argument, a bad value) SHALL exit 2; when a `--json` word comes before any `--`, it SHALL print on stdout the failure object coded `usage-invalid`, its `error` being clap's message without its `error: ` prefix or styles, and clap's rendering on stderr otherwise. pimalaya-cli SHALL provide `error::usage_error`, building that coded failure from a clap error, for every other place a binary turns a clap error into a failure.

#### Scenario: A mistyped flag under --json
- GIVEN a binary parsing its arguments with `clap::parse`
- WHEN it runs with `--json --frobnicate`
- THEN stdout holds one failure object with `"code":"usage-invalid"` and an `error` naming `--frobnicate`, and the exit code is 2

### Requirement: An item error is the failure object of one item
pimalaya-cli SHALL provide `error::ItemError<C>`, serialized as `{id, code, error, sources}`: `id` the item's id, `null` when unknown; `code` the code of the outermost coded error in the chain, `null` when none; `error` the message; `sources` the chain below it, outermost first. `ItemError::new(id, &err)` SHALL build it from an error, finding the code anywhere in the chain. It SHALL implement `JsonSchema` from the same declaration its serialization uses, every key required, `id` and `code` nullable, `code` typed by the binary's code enum. Its `Display` SHALL be one line, `<id>  <code>  <error>`, an absent id or code written `-`.

#### Scenario: A message the server answered for but the client could not read
- GIVEN an error coded `message-unreadable` under the context `Cannot read message 34`
- WHEN `ItemError::new(Some("34"), &err)` is serialized
- THEN it prints `{"id":"34","code":"message-unreadable","error":"Cannot read message 34","sources":[…]}`, and its `Display` reads `34  message-unreadable  Cannot read message 34`
