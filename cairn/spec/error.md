---
cairn: spec
capability: error
status: current
---

# Error

A failed command prints its error report to stdout, as text or JSON, and exits with status 1. A command that ran to its end with an outcome short of full success prints its result instead and exits 3 or above. A command line the grammar refuses exits 2. A command acting on several items lists the failure of each single item as an item error in its result's `errors`.

### Requirement: Failures carry an optional stable code
The error report SHALL print a `code` field beside `error`, `sources` and `backtrace`: the code of the outermost coded error in the chain, `null` when the chain holds none. Code values SHALL be kebab-case. The code vocabulary SHALL belong to each binary; pimalaya-cli SHALL provide the mechanism and the family's shared codes, `CommonCode`, which the failure object's schema admits beside the binary's: `usage-invalid`, which it raises itself, and `request-origin`, which a binary raises. A binary SHALL have one failure object, and a failure folded into a result SHALL keep its code. A binary reading a code off a typed error deep in the chain SHALL hand it to the report through `error::with_code`, which rebuilds the chain over an innermost coded error with the same messages, and leaves alone a chain already coded, by the binary or with a shared code, or holding a partial outcome.

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

### Requirement: A request off its API is coded request-origin
A raw protocol request command (`msgraph request`, `gmail request`, `gcal request`, `gpeople request` and the like) SHALL refuse a URL whose origin (scheme, host and port) is not the origin of the API its bearer token is for, before anything is sent, with a failure coded `request-origin`, exiting 1. pimalaya-cli SHALL provide `error::request_origin(url, api)`, building that failure from the two origins as a URL serializes them, its message ``Cannot send a request to `<url>`: it is off the API origin `<api>`, which alone receives the account's token``. A binary SHALL NOT code that refusal with a code of its own vocabulary.

#### Scenario: A full URL on another host
- GIVEN a binary whose `msgraph request` sends the account's token to `https://graph.microsoft.com`
- WHEN it runs with `--json get https://evil.example/me`
- THEN stdout holds one failure object with `"code":"request-origin"`, the exit code is 1, and no request was sent

### Requirement: The failure object has a schema
The error report SHALL implement `JsonSchema`, describing the failure object with `code` required, nullable and typed by the binary's code enum, from the same declaration its serialization uses.

### Requirement: A refused command line exits 2
pimalaya-cli SHALL provide `clap::parse`, which a binary calls in place of clap's `Parser::parse`. Help and version SHALL print and exit as clap does, the version naming the binary after any subcommand (`<bin> account list --version` prints `<bin> <version>`, never `<bin>-account-list <version>`). A command line the grammar refuses (an unknown flag or command, a missing argument, a bad value) SHALL exit 2; when a `--json` word comes before any `--`, it SHALL print on stdout the failure object coded `usage-invalid`, its `error` being clap's message without its `error: ` prefix or styles, and clap's rendering on stderr otherwise. pimalaya-cli SHALL provide `error::usage_error`, building that coded failure from a clap error, for every other place a binary turns a clap error into a failure. A failure whose chain carries that code SHALL exit 2 as well, through `ErrorReport::eval_coded` and `mcp::exit_on_failure`, so a command line a binary refuses after clap parsed it exits as one clap refused.

#### Scenario: A mistyped flag under --json
- GIVEN a binary parsing its arguments with `clap::parse`
- WHEN it runs with `--json --frobnicate`
- THEN stdout holds one failure object with `"code":"usage-invalid"` and an `error` naming `--frobnicate`, and the exit code is 2

### Requirement: A command is required under JSON
pimalaya-cli SHALL provide `clap::meet_bare_invocation`, which a binary whose command is optional calls when none is given. Under JSON it SHALL fail with `A command is required, see --help`, coded `usage-invalid`, so the binary prints the failure object and exits 2, its configuration left unread. In text it SHALL fail with the error of a missing configuration, which the binary's closure answers and a person meets as the welcome, and SHALL print the binary's help on stdout otherwise, a broken configuration counting as one.

#### Scenario: A bare run under JSON
- GIVEN a binary calling `meet_bare_invocation` when no command is given
- WHEN it runs with `--json` alone
- THEN stdout holds one failure object coded `usage-invalid`, `error` reading `A command is required, see --help`, and the exit code is 2

### Requirement: A standard command needs its argument
`completion`, `manual` and `json-schema` SHALL require their names (shells, pages, schemas) or `--dir`, so clap refuses each run without either, exiting 2 and coded `usage-invalid` under `--json`. Several names without `--dir`, which would print several values, and a page or schema name the binary has none of SHALL fail coded `usage-invalid` and exit 2, as a refused command line.

#### Scenario: A bare manual
- GIVEN a binary carrying the shared `manual` command
- WHEN it runs `--json manual`
- THEN stdout holds one failure object coded `usage-invalid` naming `--dir`, and the exit code is 2

### Requirement: An item error is the failure object of one item
pimalaya-cli SHALL provide `error::ItemError<C>`, serialized as `{id, code, error, sources}`: `id` the item's id, `null` when unknown; `code` the code of the outermost coded error in the chain, `null` when none; `error` the message; `sources` the chain below it, outermost first. `ItemError::new(id, &err)` SHALL build it from an error, finding the code anywhere in the chain. It SHALL implement `JsonSchema` from the same declaration its serialization uses, every key required, `id` and `code` nullable, `code` typed by the binary's code enum. Its `Display` SHALL be one line, `<id>  <code>  <error>`, an absent id or code written `-`. A result SHALL list every failed item in one `errors` list, never across lists of its own, and SHALL exit 3, or with a status above 3 the command documents, when the list is not empty. A row of that list MAY carry, after the four keys, a `kind` tag (kebab-case) telling the failures of one command apart and keys of the binary's own that the kind needs (cairn/spec/output.md, *Shared nouns keep one shape*), a binary flattening `ItemError` into a type of its own for it.

#### Scenario: A message the server answered for but the client could not read
- GIVEN an error coded `message-unreadable` under the context `Cannot read message 34`
- WHEN `ItemError::new(Some("34"), &err)` is serialized
- THEN it prints `{"id":"34","code":"message-unreadable","error":"Cannot read message 34","sources":[…]}`, and its `Display` reads `34  message-unreadable  Cannot read message 34`

#### Scenario: A sync failing two ways
- GIVEN a sync whose server rejected one item and whose source was not reached for a collection
- WHEN its result prints with `--json`
- THEN `errors` holds two item errors, the first tagged `"kind":"rejected"`, the second `"kind":"unreached"` with the source it names under a key of its own, and the exit code is 3 or the sync's documented partial status

### Requirement: The text report is defused
The text form of the error report SHALL run its message and each of its sources through `sanitize::lines`, so no control or bidi character of the chain reaches the terminal while its lines stay apart. When it replaced a character, its suggestions SHALL end with `Run with --json to see the exact characters of the error`. The JSON form SHALL keep the message and the sources exact.

#### Scenario: An id carrying escape sequences
- GIVEN the error `Cannot read message nope<ESC>[31mRED<BEL>` caused by a source on two lines, `Expected =` then `at line 2<ESC>]52;c;aGk=<BEL>`
- WHEN its text report is printed
- THEN it reads `Error: Cannot read message nope�[31mRED�`, and under `Caused by:` the source on its two lines with U+FFFD in place of ESC and BEL, the suggestions naming `--json`
