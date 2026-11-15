---
cairn: delta
change: a-session-never-nests
---

## ADDED Requirements

### Requirement: A session never nests
A line whose command starts a session, `repl` or `mcp`, by its name, an alias or a prefix it is inferred from, SHALL fail with `Cannot start a session inside a session`, coded `usage-invalid`, before the binary's own code runs. A line asking for that command's help SHALL still print it.

#### Scenario: An MCP server asked for inside a REPL
- GIVEN a piped session started with `--json`
- WHEN the line `mcp` is read
- THEN stdout holds the failure object coded `usage-invalid` with the error `Cannot start a session inside a session`, and the binary runs nothing

## MODIFIED Requirements

### Requirement: A session reads one command per line
`repl::Repl::run` SHALL read stdin one line at a time and SHALL end at EOF or at a line reading `quit` or `exit`. Each line SHALL be split as a POSIX shell splits it, quotes and escapes included, then parsed with the binary's command grammar without the binary name, plus `--json`, subcommand names inferred from a unique prefix. A blank line, or a line whose first word starts a `#` comment, SHALL be skipped. A line whose quote is not closed SHALL fail with `Cannot split the line: a quote is not closed`, a line that is not UTF-8 with `Cannot read the line: it is not valid UTF-8`, both coded `usage-invalid`, and the loop SHALL go on after every failure.

#### Scenario: A quoted word and an open quote
- GIVEN a piped session
- WHEN the lines `token "show"`, `auth resume 'abc` and `token show` are read
- THEN the first runs `token show`, the second prints the failure object naming the open quote, and the third runs

### Requirement: Session flags are refused by name
A line clap cannot parse that sets a flag fixed for the session SHALL fail with ``Cannot set `<flag>` inside a session, start a new session with it instead``, coded `usage-invalid`, `<flag>` being the fixed flag as listed, in place of clap's error. The flags `-a`, `--account`, `-c`, `--config`, `--log-level`, `--log` and `--log-file` SHALL always be fixed, and a binary SHALL be able to add its own. A flag SHALL be recognised alone, as `--<long>=<value>`, or as a short flag with its value attached, before any `--`.

#### Scenario: Another account on one line
- GIVEN a session
- WHEN the line `envelope list -a work` is read
- THEN it fails with ``Cannot set `-a` inside a session, start a new session with it instead``, coded `usage-invalid`

### Requirement: Help keeps JSON lines whole
A line asking for help or the version SHALL print clap's text on stdout in a text session, and on stderr when the session's `--json` or a `--json` word of the line before any `--` is set, so stdout keeps one JSON value per line. The version SHALL be the one the command tree of the session carries (`#[command(version, long_version = ...)]` on the binary's command enum), `version` for `-V` and `long_version` for `--version`, reachable after any command as on the command line; a tree carrying none has no version flag, so a `--version` line is refused as any unknown flag is.

#### Scenario: Help in a JSON session
- GIVEN a session started with `--json`
- WHEN the line `help` is read
- THEN the help is printed on stderr and stdout gets nothing

#### Scenario: The version in a text session
- GIVEN a text session of the binary `app` whose command enum carries the version `1.2.3` and the long version `v1.2.3 +imap`
- WHEN the lines `-V` and `--version` are read
- THEN stdout holds `app 1.2.3` then `app v1.2.3 +imap`

## REMOVED Requirements
