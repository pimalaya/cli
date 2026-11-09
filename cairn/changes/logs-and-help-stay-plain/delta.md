---
cairn: delta
change: logs-and-help-stay-plain
---

## ADDED Requirements

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
