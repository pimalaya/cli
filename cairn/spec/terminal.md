---
cairn: spec
capability: terminal
status: current
---

# Terminal

The `terminal` feature gives a binary its clap glue, its logger, its printer, its text layouts and its error report; the printer and the error report have their own capabilities, output and error. What pimalaya-cli prints for a human is plain text: log lines carry no colour, help is printed as written, a record prints as aligned fields and a list as borderless aligned columns, and nothing asks the terminal for its size. Since Cargo features only add up, pimalaya-cli asks its dependencies for no feature its code does not use; a binary wanting colour or wrapped help enables it on its own clap or env_logger line.

### Requirement: Log lines carry no colour
`Logger::try_init` SHALL write each log line as plain text, `[<LEVEL> <target>] <message>`, with no ANSI escape sequence, whether the line goes to a terminal, a pipe or the `--log-file` file. pimalaya-cli SHALL NOT enable env_logger's `color` or `auto-color` feature.

#### Scenario: A warning on a terminal
- GIVEN a binary run with `--log-level warn`, stderr on a terminal
- WHEN it logs a warning
- THEN stderr holds `[WARN  <target>] <message>` and no escape byte

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

### Requirement: Untrusted text is defused
`sanitize::text` SHALL replace each control character (C0, DEL, C1) and each bidi embedding, override and isolate of Unicode UAX #9 with U+FFFD, and SHALL borrow text holding none. It SHALL be available with the `terminal` feature.

#### Scenario: A reversed file name
- GIVEN `invoice\u{202e}fdp.exe`
- WHEN it is sanitized
- THEN it reads `invoice�fdp.exe`

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
