---
cairn: spec
capability: error
status: current
---

# Error

A failed command prints its error report to stdout, as text or JSON, and exits with status 1. A command that ran to its end with an outcome short of full success prints its result instead and exits 3 or above. A command line the grammar refuses exits 2. A command acting on several items lists the failure of each single item as an item error in its result's `errors`.

### Requirement: Failures carry an optional stable code
The error report SHALL print a `code` field beside `error`, `sources` and `backtrace`: the code of the outermost coded error in the chain, `null` when the chain holds none. Code values SHALL be kebab-case. The code vocabulary SHALL belong to each binary; pimalaya-cli SHALL provide only the mechanism, and the codes it raises itself, `usage-invalid` alone, which the failure object's schema admits beside the binary's. A binary SHALL have one failure object, and a failure folded into a result SHALL keep its code. A binary reading a code off a typed error deep in the chain SHALL hand it to the report through `error::with_code`, which rebuilds the chain over an innermost coded error with the same messages, and leaves alone a chain already coded, refused as `usage-invalid` or holding a partial outcome.

### Requirement: A code names the domain's noun
A code about one item or one collection SHALL start with the noun its binary's domain gives it: `message-` and `mailbox-` for mail, `card-` and `addressbook-` for contacts, `item-` and `calendar-` for calendars, as in `<noun>-not-found`. `item-duplicate`, an item whose UID its collection already holds, SHALL be the one such code the domains share. A create naming a collection that already exists SHALL fail coded with the collection noun, `mailbox-duplicate`, `addressbook-duplicate` or `calendar-duplicate`, on every backend: a local store (Maildir, mbox, vdir, pimdir, a creation already queued there included) as much as a server answering that the collection exists. It SHALL NOT succeed, fail uncoded or fail coded `item-duplicate`. A collection is named by what its backend keys it by, so a backend letting two collections share a display name holds no duplicate to refuse. Inside a protocol namespace, a server's refusal stays a result (cairn/spec/output.md, *A protocol refusal is a result*).

#### Scenario: An unknown id in two domains
- GIVEN an id no collection holds
- WHEN a mail binary reads it as a message and a contacts binary as a card
- THEN the first fails coded `message-not-found` and the second `card-not-found`

#### Scenario: A collection created twice
- GIVEN a mailbox `Archive` in a pimdir store and an address book `work` in a vdir, both existing
- WHEN a mail binary creates the mailbox `Archive` and a contacts binary the address book `work`
- THEN the first fails coded `mailbox-duplicate` and the second `addressbook-duplicate`, each exiting 1, and neither store changes

### Requirement: The failure object has a schema
The error report SHALL implement `JsonSchema`, describing the failure object with `code` required, nullable and typed by the binary's code enum, from the same declaration its serialization uses.

### Requirement: A refused command line exits 2
pimalaya-cli SHALL provide `clap::parse`, which a binary calls in place of clap's `Parser::parse`. Help and version SHALL print and exit as clap does. A command line the grammar refuses (an unknown flag or command, a missing argument, a bad value) SHALL exit 2; when a `--json` word comes before any `--`, it SHALL print on stdout the failure object coded `usage-invalid`, its `error` being clap's message without its `error: ` prefix or styles, and clap's rendering on stderr otherwise. pimalaya-cli SHALL provide `error::usage_error`, building that coded failure from a clap error, for every other place a binary turns a clap error into a failure. A failure whose chain carries that code SHALL exit 2 as well, through `ErrorReport::eval_coded` and `mcp::exit_on_failure`, so a command line a binary refuses after clap parsed it exits as one clap refused.

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

### Requirement: The text report is defused
The text form of the error report SHALL run its message and each of its sources through `sanitize::lines`, so no control or bidi character of the chain reaches the terminal while its lines stay apart. When it replaced a character, its suggestions SHALL end with `Run with --json to see the exact characters of the error`. The JSON form SHALL keep the message and the sources exact.

#### Scenario: An id carrying escape sequences
- GIVEN the error `Cannot read message nope<ESC>[31mRED<BEL>` caused by a source on two lines, `Expected =` then `at line 2<ESC>]52;c;aGk=<BEL>`
- WHEN its text report is printed
- THEN it reads `Error: Cannot read message nope�[31mRED�`, and under `Caused by:` the source on its two lines with U+FFFD in place of ESC and BEL, the suggestions naming `--json`
