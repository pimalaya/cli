---
cairn: delta
change: the-contract-audit-rulings
---

## ADDED Requirements

### Requirement: A queue read prints the list row
`<bin> pimdir queue show` SHALL print the row `<bin> pimdir queue list` prints for it, every key of it, plus what only a read carries: `state`, `appliedAt`, `refusals` and the like, named by the binary. A row id or handle naming no row SHALL fail coded `row-not-found`, exiting 1.

#### Scenario: An unknown row
- GIVEN a queue holding no row 42
- WHEN `<bin> pimdir queue show 42` runs with `--json`
- THEN stdout holds one failure object coded `row-not-found` and the exit code is 1

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

## MODIFIED Requirements

### Requirement: A write answers a receipt
A command that writes SHALL print what it acted on, using these keys for these meanings: `id` (the item as it is after the write; for a queued write, the id the store gives the item, its link id, never a placeholder such as `q<id>`, `0` or `""`; `null` only when the backend gives none), `ids` (several items, in the order acted on), `count` (how many items matched), `queued` (`true` when a sync engine applies the write later), `row` and `handle` (the queue row a write appended and that row under the store's identity, as `<bin> pimdir queue list` prints them, on a receipt that names its row, such as a pimdir intent's; never in place of `id`), `response` (a protocol reply as sent), `warnings` (warnings attached to an accepted write), `path` (a file written). A receipt SHALL NOT be a generic map of fields. An action with nothing created or changed SHALL still name its target.

### Requirement: Shared nouns keep one shape
Every binary SHALL print an account row as `{name, default, backends, notBuilt}`, an account checked as `{account, backends}`, a backend check row as `{backend, ok, code, error, tls}`, `tls` being `{provider, crypto, cert, clientCert}` (each a string or `null`) or `null` for a local backend or a cleartext connection, a file written as `{path, size}`, an address book or calendar row as `{id, name, description, color, default}`, a mailbox row as `{id, name, role, total, unread}`, a queued action's row id as `row` and its handle as `handle`, an address as `{name, email}`, and the error of one item as `{id, code, error, sources}`. A shared noun MAY carry keys of the binary's own after the shared ones, such as a `kind` tag on an item error; it SHALL NOT rename, retype or drop a shared key, so a reader of the shared keys reads every binary alike.

#### Scenario: A check row with a key of its own
- GIVEN a sync engine whose check row also reports what a source declares
- WHEN it prints the row of a backend that answered over a cleartext connection
- THEN the row carries `backend`, `ok`, `code` `null`, `error` `null` and `tls` `null`, then its own `capabilities`

### Requirement: A queue row keeps one shape
The reference queue row is the one `<bin> pimdir queue list` prints in a binary reading a pimdir store (himalaya, cardamum, calendula); the `pimdir` binary of io-pimdir, a tool over the store itself, MAY keep shapes of its own. `<bin> pimdir queue list` SHALL print `{actions, parked, errors}`: `actions` every pending row, of every kind the store holds; `parked` every parked row; `errors` an item error for each row it cannot decode, the rows it can decode still listed, the command then exiting 3. It SHALL print each row it lists, pending or parked, as `{row, handle, kind, collection, linkId, itemId, producer, queuedAt, attempts, unconfirmed, error}` plus its domain's label key: `subject` for mail, `fullName` for contacts, `summary` for calendars. Every key SHALL print, `null` or not. `row` SHALL be an integer, the queue row id `<bin> pimdir queue show` and `<bin> pimdir queue cancel` take. `handle` SHALL be a string, the same row under the store's identity, `<store-id>.<incarnation>.<row>` (io-pimdir's `PimdirActionHandle`, pimdir STORAGE §4.5, the identity read from `PimdirReader::change_cursor`), which those commands take in place of `row` and check against the store, so a row id never names a row of a copy restored since. `kind` SHALL be a string, the row's action as the store holds it (pimdir STORAGE §15.3), read from io-pimdir (`PimdirAction::kind` for a pending row, `PimdirParkedAction::action` for a parked one): the store's own `add`, `set-flags`, `remove`, `move`, `copy`, `update` and `set-performer`, the intents of `io_pimdir::intent` (`submit`, `calendar-reply`, `calendar-cancel`, `collection-create`, `collection-rename`, `collection-delete`), all kebab-case, or an application's own (`x-`) as stored; it SHALL NOT be re-spelled. `collection` SHALL be a string, the id of the collection the row is queued on, as the binary's collection listing prints it. `linkId` SHALL be a string or `null`: the link id of the item the row creates, the `id` its write's receipt printed, `null` when the row creates no item or the store gives it none. `itemId` SHALL be a string or `null`: the id of the stored item the row addresses (an update, a flag change, a move, a copy, a removal, an intent on an item), as the binary's item listing prints it, `null` when the row addresses no stored item, a creation among them. `producer` SHALL be a string, the process that staged the row. `queuedAt` SHALL be a string, the instant the store stamped the row, RFC 3339 in UTC with `Z`. `attempts` SHALL be an integer from 0, how many times the sync engine tried the row. `unconfirmed` SHALL be a boolean, `true` when the row's outcome is unknown (pimdir STORAGE §15.2), its `error` then beginning `unconfirmed: `. `error` SHALL be a string or `null`: `null` while the row is pending, the store's text once it is parked, a text beginning `unconfirmed: ` marking a row whose outcome is unknown (pimdir STORAGE §15.2). The label SHALL be a string or `null`, read from the body the row pins or the item it addresses, `null` when there is none to read.

#### Scenario: A pending creation and a parked update
- GIVEN a contacts binary whose pimdir queue holds a pending `add` of the card `Jane Doe` and a parked `update` of another card
- WHEN `<bin> pimdir queue list` runs with `--json`
- THEN `actions` holds the first row and `parked` the second, `errors` empty, both rows carrying the eleven shared keys, `handle` reading `<store-id>.<incarnation>.<row>`, and `fullName`, the `add` with its `linkId`, `itemId` `null` and `error` `null`, the `update` with `linkId` `null`, the card it updates as `itemId` and its `error` text

#### Scenario: A row the binary cannot decode
- GIVEN a mail binary whose pimdir queue holds a pending `set-flags` row and a row whose body it cannot decode
- WHEN `<bin> pimdir queue list` runs with `--json`
- THEN `actions` holds the `set-flags` row, `errors` an item error naming the other row, and the exit code is 3

### Requirement: A partial outcome prints its result
A command that ran to its end with an outcome that is not a full success SHALL print its result and exit with a code of 3 or above, whose meaning the command documents; 2 SHALL stay clap's, for a refused command line. A `check` with a failed backend SHALL print every row and exit 3. A result whose `errors` list is not empty SHALL exit 3, or with a status above 3 the command documents for that outcome; it SHALL NOT exit 0. pimalaya-cli SHALL provide a marker error a command returns once its result is printed, which the error report turns into the exit status without printing anything more.

### Requirement: Events precede the result
A command MAY print events before its result, each as soon as the command observes what it reports. An event SHALL be an object whose root carries `event`, a kebab-case name; a result SHALL NOT carry `event` at its root. The last line SHALL be the failure object when the exit code is 1 and the result otherwise; a command printing only events has no result. A failed item SHALL never be a failure object: a command printing one result SHALL list it in its result's `errors`, an item error per failed item, `[]` when none failed, and a command reporting items as events MAY report it as an event field instead. With JSON output, each event SHALL be one flushed line; in text mode, events SHALL be written to stdout only when it is a terminal, a long-lived command streaming them aside (*A long-lived command streams events*).

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
A command that runs until stopped (a watch) or for long (a sync) SHALL print one event line per event while it runs, then exactly one line without `event`: its result, exiting 0, 3 or the binary's own partial status, or the failure object, exiting 1. An error the command recovers from SHALL be an event carrying `code`, `error` and `sources`, as an item error does. On SIGINT or SIGTERM it SHALL stop cleanly and print its result: a watch exits 0, a bounded run its incomplete status. The text form of that result SHALL say the command stopped (`Watch stopped`), so a person reading a pipe knows it ended. pimalaya-cli SHALL provide `signal::Shutdown`, a cloneable token whose `install` makes the first termination signal request the shutdown rather than end the process, a second one ending it with 130, and the spinner SHALL defer to it. Such a command SHALL write its text events to stdout even when it is no terminal, through `StdoutPrinter::streaming`, since a status bar reads them from a pipe. Only such a command SHALL stream: a command ending on its own once its step is done (a check, an init, a bounded step of a long-lived command run alone, such as a sync draining its queue only) SHALL NOT, so its redirected text output holds its result alone.

#### Scenario: A watch piped to a status bar
- GIVEN a watch command built on a streaming printer
- WHEN it runs without `--json`, stdout piped, and receives SIGTERM
- THEN the pipe received one text line per event as it came, then the result, and the exit code is 0

#### Scenario: A check redirected to a file
- GIVEN a sync engine whose check reports progress as text events on a terminal
- WHEN `<bin> account check > report.txt` runs
- THEN report.txt holds the check's result alone

### Requirement: A refused command line exits 2
pimalaya-cli SHALL provide `clap::parse`, which a binary calls in place of clap's `Parser::parse`. Help and version SHALL print and exit as clap does, the version naming the binary after any subcommand (`<bin> account list --version` prints `<bin> <version>`, never `<bin>-account-list <version>`). A command line the grammar refuses (an unknown flag or command, a missing argument, a bad value) SHALL exit 2; when a `--json` word comes before any `--`, it SHALL print on stdout the failure object coded `usage-invalid`, its `error` being clap's message without its `error: ` prefix or styles, and clap's rendering on stderr otherwise. pimalaya-cli SHALL provide `error::usage_error`, building that coded failure from a clap error, for every other place a binary turns a clap error into a failure. A failure whose chain carries that code SHALL exit 2 as well, through `ErrorReport::eval_coded` and `mcp::exit_on_failure`, so a command line a binary refuses after clap parsed it exits as one clap refused.

#### Scenario: A mistyped flag under --json
- GIVEN a binary parsing its arguments with `clap::parse`
- WHEN it runs with `--json --frobnicate`
- THEN stdout holds one failure object with `"code":"usage-invalid"` and an `error` naming `--frobnicate`, and the exit code is 2

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

### Requirement: An account's capabilities print one line each
`capability::AccountCapabilityOutput` SHALL be the output of every binary's `account capability`: `{account, capabilities: [{name, backend, level, detail}]}`, one row per capability of the unified API, `backend` the backend it resolves to or `null` when none is allowed, `level` one of `full`, `partial`, `server` and `none`, `detail` what the level leaves out or `null`. Its text SHALL be a `listing::Listing` headed by the account, one line per capability: the name, the backend, the level and the detail. A capability's `name` SHALL be the unified command it stands for, `<noun>.<verb>` in the binary's own nouns (`message.add`, `card.delete`, `item.delete`, `mailbox.create`, `addressbook.update`), or `<noun>.<verb>.<argument>` for an argument of that command a backend may or may not serve, the argument spelled as its long flag (`event.add.online-meeting`, `event.update.send-updates`), so a script maps a capability to the command and the flag it runs. This vocabulary is not the pimdir store's: a sync engine reporting what a source declares to a store (neverest's `check`) SHALL print the store's names as pimdir STORAGE §15 spells them (`mail.message.add`, `collection.rename`), the names a store records and a queue refusal names, and a binary SHALL NOT translate one into the other in its output.

#### Scenario: A capability with no transport
- GIVEN the account `work` serving `message.move` through `imap` in full and `message.send` through no backend
- WHEN the output is printed
- THEN it reads `Account:  work`, a blank line, `message.move  imap  full  -` and `message.send  -     none  No transport`, and the JSON row of `message.send` has `"backend": null` and `"level": "none"`

### Requirement: The accounts and their checks print alike
`account::AccountListOutput` SHALL be the output of every binary's `account list`: `{accounts: [{name, default, backends, notBuilt}]}`, `backends` the backends the account declares a block for that the build serves and `notBuilt` the ones it leaves out. Its text SHALL be one line per account as `columns::Columns` prints them with no header row: the name sanitized, the backends then each left-out one followed by ` (not built)`, comma-separated, and `default` on the default account. `account::AccountCheckOutput` SHALL be the output of every binary's `account check`: `{accounts: [{account, backends: [{backend, ok, code, error, tls}]}]}`, one `account::AccountCheck` per account checked, in the order checked, a binary checking the one account `-a` names printing a list of one; `code` typed by the binary's code enum; `tls` an `account::BackendCheckTls`, `{provider, crypto, cert, clientCert}`, each a string or `null` where the build's default answers (`provider` `rustls` or `native-tls`, `crypto` `aws` or `ring`, `cert` the trusted certificate's path, `clientCert` the client certificate's path), or `null` for a local backend or a cleartext connection. Its text SHALL print one block per account, one blank line between blocks, so a single account reads as one block: `fields::Fields` holding `Account`, then one field per backend reading `OK` or `FAIL (<error>)`, followed, when `tls` is not `null`, by `, TLS` and its parts set, comma-separated (`<provider>`, `<crypto>`, `cert <path>`, `client cert <path>`), the account, the error and the parts sanitized. `account::BackendCheck::new` SHALL fold a connection attempt into its row: `code` the outermost coded error of the chain, `error` the whole chain on one line, `tls` `null` for the binary to set on a connection over TLS. `AccountCheckOutput::ok` SHALL tell whether every backend of every account answered.

#### Scenario: A reduced build
- GIVEN the default account `work` declaring `imap` and `smtp` blocks, in a build leaving `smtp` out
- WHEN its list is printed
- THEN the text reads `work  imap,smtp (not built)  default`, and the JSON row `{"name":"work","default":true,"backends":["imap"],"notBuilt":["smtp"]}`

#### Scenario: A backend refusing the credentials
- GIVEN the account `work` whose `imap` answers and whose `smtp` fails with `Cannot open the session` caused by `Bad credentials`
- WHEN its check is printed
- THEN the text reads `Account:  work`, `imap:     OK` and `smtp:     FAIL (Cannot open the session: Bad credentials)`

#### Scenario: Two accounts, one over TLS
- GIVEN the account `home` whose `maildir` answers, and the account `work` whose `imap` answers over native-tls trusting `/etc/ca.pem`
- WHEN their check is printed
- THEN the text reads `Account:  home`, `maildir:  OK`, a blank line, `Account:  work` and `imap:     OK, TLS native-tls, cert /etc/ca.pem`, and the JSON row of `maildir` has `"tls": null` and the one of `imap` `"tls": {"provider": "native-tls", "crypto": null, "cert": "/etc/ca.pem", "clientCert": null}`

### Requirement: Every binary carries an account group
Every binary SHALL carry an `account` group holding `account list`, printing the accounts its configuration declares without reaching anything, and `account check`, opening what each account it checks stands on (each backend it declares, for a binary with backends) and printing one row each, under its account: a list of accounts, of one for a binary running against the account `-a` names, of every account it runs over for one covering several (a sync engine); a binary MAY also list every account a backend it opens serves, such as the accounts of a pimdir store, each with its own rows. A binary with a unified API SHALL also carry `account capability`. A binary whose backends serve no unified API carries none, having no capability for it to report. A binary that had a top-level `check` SHALL keep it as a hidden alias of `account check`.

#### Scenario: A binary moving its check
- GIVEN a binary whose check ran as `check`
- WHEN it takes the account group
- THEN `account check` runs the check, and `check` still does without being listed in the help

### Requirement: Help keeps JSON lines whole
A line asking for help or the version SHALL print clap's text on stdout in a text session, and on stderr when the session's `--json` or a `--json` word of the line before any `--` is set, so stdout keeps one JSON value per line. The version SHALL be the one the command tree of the session carries (`#[command(version, long_version = ...)]` on the binary's command enum), `version` for `-V` and `long_version` for `--version`, reachable after any command as on the command line and naming the binary there, not the command; a tree carrying none has no version flag, so a `--version` line is refused as any unknown flag is.

#### Scenario: Help in a JSON session
- GIVEN a session started with `--json`
- WHEN the line `help` is read
- THEN the help is printed on stderr and stdout gets nothing

#### Scenario: The version in a text session
- GIVEN a text session of the binary `app` whose command enum carries the version `1.2.3` and the long version `v1.2.3 +imap`
- WHEN the lines `-V`, `--version` and `token show --version` are read
- THEN stdout holds `app 1.2.3`, `app v1.2.3 +imap`, then `app v1.2.3 +imap` again

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

## REMOVED Requirements
