---
cairn: spec
capability: terminal
status: current
---

# Terminal

The `terminal` feature gives a binary its clap glue, its logger, its printer and its error report; the printer and the error report have their own capabilities, output and error. What pimalaya-cli prints for a human is plain text: log lines carry no colour, help is printed as written, and nothing asks the terminal for its size. Since Cargo features only add up, pimalaya-cli asks its dependencies for no feature its code does not use; a binary wanting colour or wrapped help enables it on its own clap or env_logger line.

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
Every dependency SHALL be declared with `default-features = false` and SHALL name the features pimalaya-cli asks of it, so no consumer gets a dependency feature, colour or a terminal query among them, that the code does not use. The dependencies of the `prompt`, `wizard`, `table` and `spinner` features SHALL name the features their defaults enable.

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
