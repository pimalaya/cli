---
cairn: delta
change: the-account-outputs-are-shared
---

## ADDED Requirements

### Requirement: The accounts and their checks print alike
(terminal) `account::AccountListOutput` SHALL be the output of every binary's `account list`: `{accounts: [{name, default, backends, notBuilt}]}`, `backends` the backends the account declares a block for that the build serves and `notBuilt` the ones it leaves out. Its text SHALL be one line per account as `columns::Columns` prints them with no header row: the name sanitized, the backends then each left-out one followed by ` (not built)`, comma-separated, and `default` on the default account. `account::AccountCheckOutput` SHALL be the output of every binary's `account check`: `{account, backends: [{backend, ok, code, error}]}`, `code` typed by the binary's code enum. Its text SHALL be `fields::Fields` holding `Account`, then one field per backend reading `OK` or `FAIL (<error>)`, the account and the error sanitized. `account::BackendCheck::new` SHALL fold a connection attempt into its row: `code` the outermost coded error of the chain, `error` the whole chain on one line.

#### Scenario: A reduced build
- GIVEN the default account `work` declaring `imap` and `smtp` blocks, in a build leaving `smtp` out
- WHEN its list is printed
- THEN the text reads `work  imap,smtp (not built)  default`, and the JSON row `{"name":"work","default":true,"backends":["imap"],"notBuilt":["smtp"]}`

#### Scenario: A backend refusing the credentials
- GIVEN the account `work` whose `imap` answers and whose `smtp` fails with `Cannot open the session` caused by `Bad credentials`
- WHEN its check is printed
- THEN the text reads `Account:  work`, `imap:     OK` and `smtp:     FAIL (Cannot open the session: Bad credentials)`

### Requirement: A file written is named with its size
(output) A command writing to a file what it would otherwise print SHALL print `output::SavedOutput`, `{path, size}`, `path` the file written and `size` the number of bytes written. Its text SHALL read `Saved <size> bytes to <path>`.

#### Scenario: A part saved to a file
- GIVEN a command writing 42 bytes to `/tmp/a.pdf`
- WHEN it prints its receipt
- THEN the text reads `Saved 42 bytes to /tmp/a.pdf` and the JSON `{"path":"/tmp/a.pdf","size":42}`

## MODIFIED Requirements

### Requirement: Shared nouns keep one shape
(output) Every binary SHALL print an account row as `{name, default, backends, notBuilt}`, a backend check row as `{backend, ok, code, error}`, a file written as `{path, size}`, a collection row as `{id, name, description, color, default}`, a queued action's row id as `row`, an address as `{name, email}`, and the error of one item as `{id, code, error, sources}`.

## REMOVED Requirements
