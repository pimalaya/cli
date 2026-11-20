---
cairn: spec
capability: terminal
status: current
---

# Terminal

The `terminal` feature gives a binary its clap glue, its logger, its printer, its text layouts and its error report; the printer and the error report have their own capabilities, output and error. What pimalaya-cli prints for a human is plain text: log lines carry no colour, the spinner colours its glyphs on a terminal only, help is printed as written, a record prints as aligned fields and a list as borderless aligned columns, and nothing asks the terminal for its size. Since Cargo features only add up, pimalaya-cli asks its dependencies for no feature its code does not use; a binary wanting colour or wrapped help enables it on its own clap or env_logger line.

### Requirement: Log lines carry no colour
`Logger::try_init` SHALL write each log line as plain text, `[<LEVEL> <target>] <message>`, with no ANSI escape sequence, whether the line goes to a terminal, a pipe or the `--log-file` file. pimalaya-cli SHALL NOT enable env_logger's `color` or `auto-color` feature.

#### Scenario: A warning on a terminal
- GIVEN a binary run with `--log-level warn`, stderr on a terminal
- WHEN it logs a warning
- THEN stderr holds `[WARN  <target>] <message>` and no escape byte

### Requirement: The spinner colours a terminal only
`spinner::Spinner` SHALL draw its frames only when stderr is a terminal, and SHALL colour its frames and its `✓` and `✗` glyphs only when stderr is a terminal and `NO_COLOR` is unset or empty, as no-color.org asks. On a stream that is not a terminal, `success` and `failure` SHALL write their line as plain text, `<glyph> <message>`, with no escape byte, whatever `NO_COLOR` holds. On a terminal under a non-empty `NO_COLOR`, the spinner SHALL write no SGR sequence, the cursor control drawing its frames aside.

#### Scenario: Stderr redirected to a file
- GIVEN a binary run with `2>file`, `NO_COLOR` unset or set to `1`
- WHEN one spinner succeeds with `Checked` and another fails with `Not sent`
- THEN the file holds `✓ Checked` then `✗ Not sent`, each line ending with a newline, and no escape byte

#### Scenario: NO_COLOR on a terminal
- GIVEN stderr on a terminal and `NO_COLOR=1`
- WHEN a spinner runs then succeeds
- THEN stderr holds its frames and its `✓` line with no SGR sequence

### Requirement: Help is printed as written
pimalaya-cli SHALL enable clap's `help` feature and SHALL NOT enable `wrap_help`, so a binary's help never queries the terminal size and is not wrapped to a width. Each paragraph of a doc comment SHALL print on one line, after its indentation, whatever the terminal width; the terminal folds a long line itself. Under `-h`, clap still lays the option list out against 100 columns: an option's help moves under its name when it would start past the 40th column and end past the 100th. Under `--help`, it always sits under the name.

#### Scenario: Long help on a narrow terminal
- GIVEN an option whose doc comment holds a paragraph longer than 60 characters
- WHEN `--help` prints on a 60-column terminal
- THEN that paragraph prints as one line, longer than 60 columns

### Requirement: Dependencies name their features
Every dependency SHALL be declared with `default-features = false` and SHALL name the features pimalaya-cli asks of it, so no consumer gets a dependency feature, colour or a terminal query among them, that the code does not use. The dependencies of the `prompt` and `spinner` features SHALL name the features their defaults enable.

### Requirement: Path arguments expand like a shell
`clap::parsers::path_parser` SHALL expand a leading `~` to the home directory dirs reports, and `$NAME` and `${NAME}` to the environment variable `NAME`, then SHALL canonicalize the path, keeping the expanded path when it does not exist. A home directory that is not valid Unicode SHALL leave the `~` as written. A path naming an unset variable SHALL be rejected with `error looking key '<NAME>' up: environment variable not found`.

#### Scenario: The home directory three ways
- GIVEN `HOME` set to `/home/user`, which holds no `x`
- WHEN `~/x`, `$HOME/x` and `${HOME}/x` are parsed
- THEN each gives `/home/user/x`

#### Scenario: An unset variable
- GIVEN no `UNSET` variable in the environment
- WHEN `$UNSET/file` is parsed
- THEN it is rejected with `error looking key 'UNSET' up: environment variable not found`

### Requirement: One dirs version
shellexpand SHALL be built with `base-0` alone, without its `tilde` feature, the home directory coming from pimalaya-cli's own dirs dependency, so a binary carries one dirs version.

### Requirement: One configuration path per flag
`clap::args::ConfigFlags` SHALL take one path per `-c` or `--config`, given as many times as there are files, the first one the base and the next ones merged on top. A value SHALL NOT be split, whatever it holds, and no environment variable SHALL supply it. Each path SHALL go through `path_parser`.

#### Scenario: A Windows path and a colon
- GIVEN `-c 'C:\a.toml' -c 'b:c;d.toml'`
- WHEN the arguments are parsed
- THEN `paths` holds exactly `C:\a.toml` then `b:c;d.toml`

### Requirement: A text record prints as aligned fields
`fields::Fields` SHALL print a record as one `Label: value` line per field, in the order pushed, the values starting two spaces past the colon of the longest label, as clap aligns its help. A value spanning several lines SHALL continue under its first line, an empty value SHALL leave its label alone with no trailing space, and every line SHALL end with a newline, so an empty record prints nothing. Values SHALL print as given.

#### Scenario: A token record
- GIVEN the fields `Type` = `Bearer`, `Refresh token` = `yes` and `Scopes` = `mail` then `contacts` on two lines
- WHEN the record is printed
- THEN it reads `Type:           Bearer`, `Refresh token:  yes`, `Scopes:         mail` and 16 spaces before `contacts`, each line ending with a newline

### Requirement: A list prints as aligned columns
`columns::Columns` SHALL print a list as its header row then one row per item, in the order pushed, with no border and no colour. Cells SHALL be separated by two spaces and padded to the widest cell of their column, measured in terminal display columns, so a wide character counts two; the last column SHALL NOT be padded, so no line ends with a space the layout added. An empty cell SHALL print as `-`, and a line feed or carriage return inside a cell SHALL print as a space. A row shorter than the widest one SHALL be completed with empty cells. Header labels SHALL print as given, and a list with no label SHALL print no header row. Nothing SHALL be truncated and the terminal SHALL NOT be asked for its size. Every line SHALL end with a newline, so a list without rows prints its header alone.

#### Scenario: A wide last column
- GIVEN the header `ID`, `NAME`, `DESCRIPTION` and the rows `1`, `Inbox`, `Mail received` and `42`, `Sent`, empty
- WHEN the list is printed
- THEN it reads `ID  NAME   DESCRIPTION`, `1   Inbox  Mail received` and `42  Sent   -`, each line ending with a newline and none with a space

#### Scenario: Wide characters
- GIVEN a cell `日本` in a column whose other cell is `ab`
- WHEN the list is printed
- THEN `ab` is followed by four spaces before the next column, `日本` by two

### Requirement: A wide list prints as records
`records::Records` SHALL print a list as one block per item, in the order pushed, one blank line between blocks. A block SHALL start with the item's title alone on a line, followed by its state words in parentheses, comma-separated, when it has any; then its fields as `fields::Fields` prints them; then, when the item has a body, a blank line and each line of the body indented by four spaces, an empty body line staying empty. Every line SHALL end with a newline, so an empty list prints nothing. Titles, state words, fields and bodies SHALL print as given. A binary SHALL print a list as `columns::Columns` when each row reads on one line, its longest value last, and as `Records` when a row carries more than one value of unbounded length.

#### Scenario: Two envelopes
- GIVEN the item `42` in the state `unread`, `flagged`, with the fields `From` = `Alice <a@example.org>` and `Date`, and the body `Hi`, then the item `41` in no state with the body `Re: Hi`
- WHEN the list is printed
- THEN it reads `42 (unread, flagged)`, its two fields, a blank line, `    Hi`, a blank line, then `41`, its fields, a blank line and `    Re: Hi`, each line ending with a newline

### Requirement: A list prints under the fields placing it
`listing::Listing` SHALL print a list as its header fields as `fields::Fields` prints them, one blank line, then its items as `columns::Columns` prints them with no header row. The header's values SHALL start two spaces past the colon of the longest label; every page of a listing carries the same labels, so its values align from page to page. `Listing::new` SHALL head it with `Account:`, the account name sanitized and the backend in parentheses when given; `Listing::head` SHALL add a field, its value sanitized. `Listing::page` SHALL add `Page:` to the header and, while a page follows, set the flag reaching it: `--page <n>` for a page by number or the whole listing, `--page-token <token>` for a page reached by a token, the token sanitized. The page SHALL read `<page>, <noun> <first>` or `<page>, <nouns> <first> to <last>` when numbered, `all, ...` when unpaged, `<count> <noun(s)>` when reached by a token, `empty` when it holds nothing, and end with `, the last` on a numbered or token page with items that no page follows. The flag SHALL print as a `Next page: <flag> <value>` line after the listing's items and before its item errors, one blank line apart when anything printed before it. The line reads as `output::Paginated`'s `Next page: <cursor>`, the flag standing before the value because a listing frame pages by `--page` or by `--page-token` and the reader needs to know which, where a `Paginated` cursor always goes back through `--page-token`. Nothing else SHALL print under the items. With no item, the `empty` line SHALL print in their place when set, and the header SHALL stand alone otherwise, the next page line still following it. A listing with no header field SHALL print its items alone.

#### Scenario: The second page of envelopes
- GIVEN a listing of the account `work` on `imap`, the field `Mailbox` = `INBOX`, the numbered page 2 of size 2 holding two envelopes, page 3 following
- WHEN it is printed
- THEN it reads `Account:  work (imap)`, `Mailbox:  INBOX`, `Page:     2, envelopes 3 to 4`, a blank line, one line per envelope, a blank line, then `Next page: --page 3`

### Requirement: Item errors print last
`error::ItemErrors` SHALL print the item errors of a command as one line per error, in the order pushed: `Error:`, the item's id, the code and the message, aligned among themselves as `columns::Columns` aligns cells, an absent id or code written `-`, the id and message sanitized. A command SHALL print them after its output and one blank line (`ItemErrors::write_after`), and SHALL print nothing, blank line included, when there are none. `listing::Listing` SHALL print its errors that way after its items and its next page line, or alone when it prints nothing else: the next page line belongs to the listing it continues, and the errors stay the last lines of every output, a listing's included.

#### Scenario: Two errors under a listing
- GIVEN a listing holding one envelope, an error coded `message-unreadable` with no id and an uncoded error of the item `42`
- WHEN it is printed
- THEN the envelope line is followed by a blank line, `Error:  -   message-unreadable  Cannot decode it` and `Error:  42  -                   Gone`

#### Scenario: An error under a page that continues
- GIVEN a listing holding one envelope on the numbered page 1 of size 1, page 2 following, and an uncoded error of the item `42`
- WHEN it is printed
- THEN the envelope line is followed by a blank line, `Next page: --page 2`, a blank line and `Error:  42  -  Gone`

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

### Requirement: Untrusted text is defused
`sanitize::text` SHALL replace each control character (C0, DEL, C1) and each bidi embedding, override and isolate of Unicode UAX #9 with U+FFFD, and SHALL borrow text holding none. `sanitize::lines` SHALL do the same, keeping the line feed and the tab, for a text printed on several lines. `sanitize::path_component` SHALL turn a name into one path component by the rule of pimdir STORAGE §18, as io-pimdir's `security::path_component` does: every `/` and `\`, NUL, control character and bidirectional formatting character (U+061C, U+200E, U+200F, U+202A to U+202E, U+2066 to U+2069) replaced by `_`, and an empty name written `_`, `.` and `..` written `_` and `__`. All three SHALL be available with the `terminal` feature.

#### Scenario: A reversed file name
- GIVEN `invoice\u{202e}fdp.exe`
- WHEN it is sanitized
- THEN it reads `invoice�fdp.exe`

#### Scenario: An attachment name climbing out of its directory
- GIVEN the attachment names `../../etc/passwd`, `..` and `invoice\u{202e}fdp.exe`
- WHEN each goes through `sanitize::path_component`
- THEN they read `.._.._etc_passwd`, `__` and `invoice_fdp.exe`

### Requirement: A duration prints friendly
`duration::friendly` SHALL print a count of whole seconds in jiff's friendly compact form (`{span:#}`), the span balanced up to days of 24 hours: `2d 4h 30m`, `0s` when empty. A count too large for a jiff span SHALL print as jiff's `SpanPrinter` prints the unsigned duration, its largest unit the hour. It SHALL be available with the `duration` feature, whose only dependency is jiff.

#### Scenario: A token lifetime and a time left
- GIVEN the durations 90, 3600, 5430 and 187 261 seconds
- WHEN each goes through `duration::friendly`
- THEN they read `1m 30s`, `1h`, `1h 30m 30s` and `2d 4h 1m 1s`

#### Scenario: A duration past a span
- GIVEN the duration `u64::MAX` seconds
- WHEN it goes through `duration::friendly`
- THEN it reads `5124095576030431h 15s`

### Requirement: Items are reached where they are defined
pimalaya-cli SHALL NOT re-export an item, its own or a dependency's: a binary SHALL import each item at the path of the module defining it.

#### Scenario: The completion command
- GIVEN a binary adding the shared completion command to its parser
- WHEN it imports the command
- THEN the path is `pimalaya_cli::clap::commands::completion::CompletionCommand`, and `pimalaya_cli::clap::commands::CompletionCommand` does not resolve

### Requirement: A printed command runs as pasted
`shell::quote` SHALL wrap a string in single quotes, each embedded `'` written `'\''`, so a POSIX shell reads it back unchanged; on Windows it SHALL wrap it in double quotes, which cmd and PowerShell both read. `shell::invocation` SHALL write the binary name, then ` -c ` and the quoted path for each configuration path in order, then ` -a ` and the quoted account name when one is given and not empty.

#### Scenario: A quote in a path and an account
- GIVEN the binary `app`, the paths `/a.toml` and `/b c.toml`, and the account `work's`
- WHEN the invocation is built on Unix
- THEN it reads `app -c '/a.toml' -c '/b c.toml' -a 'work'\''s'`

### Requirement: A missing configuration meets a welcome
`welcome::Welcome` SHALL print the binary's intro line; a blank line; `No configuration found at <path>. Create it from the sample configuration:`; a blank line; the commands creating the file, each indented two spaces; a blank line; the binary's next-step sentence; a blank line; and the first command, indented two spaces, starting with the binary's invocation carrying its `-c` paths and no account. With a shipped sample, the commands SHALL be `mkdir -p <dir>` (`mkdir` on Windows) when the file's parent directory is named and missing, then `install -m 644 <sample> <file>` (`copy` on Windows); without one, `curl -sSL --create-dirs -o <file> <url>` (`curl.exe` on Windows). Paths SHALL be quoted with `shell::quote`. Every line SHALL end with a newline. `welcome::is_wanted` SHALL be true when the printer does not print JSON and stderr is a terminal.

#### Scenario: No sample shipped
- GIVEN the missing file `/a b/it's.toml`, given with `-c`, and no shipped sample
- WHEN the welcome prints on Unix
- THEN it holds the line `  curl -sSL --create-dirs -o '/a b/it'\''s.toml' <url>` and ends with `  <bin> -c '/a b/it'\''s.toml' <first command>`

#### Scenario: A shipped sample and a missing directory
- GIVEN the same file, its directory missing, and a shipped sample
- WHEN the welcome prints on Unix
- THEN the commands are `  mkdir -p '/a b'` then `  install -m 644 '<sample>' '/a b/it'\''s.toml'`, and no `curl`

### Requirement: The shipped sample is found next to the binary
`welcome::shipped_sample` SHALL take the running binary's path, canonicalized except on Windows, and SHALL answer the first existing file among share/doc/<bin>/config.sample.toml under the parent of the binary's directory, where a package installs it, and under the binary's directory, where an extracted release archive puts it, or nothing.

#### Scenario: An installed and an extracted sample
- GIVEN a prefix holding bin/ and share/doc/<bin>/config.sample.toml
- WHEN the sample is looked up from bin/ or from the prefix itself
- THEN both find that file, and neither does before it exists

### Requirement: Every command has a manual page
`manual` SHALL build the command tree with clap's `help` subcommands disabled, then give every visible command at any depth a page named after its dashed path, as in `<bin>-envelope-list`, its `.TH` title that name and its source the root's name and version, its synopsis the whole command. A page's VERSION section SHALL name the version once: a long version starting with `v` and a digit, as `long_version!` writes it, SHALL lose that `v` on the page, the page generator writing its own. A name given SHALL be one of those, and every page listed under a page's SEE ALSO SHALL be one the same run can write. The `manual` and `json-schema` help SHALL name no program's command as an example.

#### Scenario: A leaf page
- GIVEN the tree `test` with the subcommand `sub` holding `list` and `search`
- WHEN `manual test-sub-list` runs
- THEN the page is titled `.TH test-sub-list 1` and its synopsis reads `test sub list`

#### Scenario: Every page
- GIVEN the same tree with a hidden subcommand `secret`
- WHEN `manual --dir <dir>` runs
- THEN it writes `test.1`, `test-sub.1`, `test-sub-list.1` and `test-sub-search.1`, and `test-sub.1` lists `test-sub-list(1)` and `test-sub-search(1)`, no `help(1)`

#### Scenario: A long version written by long_version!
- GIVEN the tree `test` whose long version is `v1.2.3 +imap`
- WHEN `manual test` runs
- THEN the VERSION section reads `v1.2.3 +imap`, not `vv1.2.3 +imap`

### Requirement: A private file is its owner's
`file::private_options` SHALL answer `OpenOptions` which, on Unix, create a file with the permissions 0600 whatever the umask and fail on a symbolic link at the path rather than follow it; a file that exists SHALL keep its permissions. Elsewhere they SHALL be plain options. The caller SHALL add its mode of access.

### Requirement: The log file is private
`Logger::try_init` SHALL open the `--log-file` file in append mode through `file::private_options`, so a log file it creates on Unix is readable by its owner alone and a symbolic link at its path is refused. A log file it cannot open SHALL fail the command with `Cannot open log file <path>`, the OS error as its source.

#### Scenario: A link planted at the log path
- GIVEN a symbolic link `trace.log` pointing to a file that does not exist
- WHEN a binary runs with `--log-file trace.log`
- THEN it fails with an error naming `trace.log`, and the link's target is not created

#### Scenario: A new log file
- GIVEN a umask of 022 and no file at `trace.log`
- WHEN a binary runs with `--log-file trace.log`
- THEN `trace.log` is created with the permissions 0600
