---
cairn: spec
capability: repl
status: current
---

# REPL

The `repl` feature gives a binary keeping one account open across commands a read-eval-print loop over stdin: each line is one of the binary's own commands, without the binary name, run against what the session holds and answered on stdout, so a piped session answers each line once and an application can keep one open. The session module, with the `repl` or the `mcp` feature, keeps the commands a session runs off its stdin.

### Requirement: A session reads one command per line
`repl::Repl::run` SHALL read stdin one line at a time and SHALL end at EOF or at a line reading `quit` or `exit`. Each line SHALL be split as a POSIX shell splits it, quotes and escapes included, then parsed with the binary's command grammar without the binary name, plus `--json`, subcommand names inferred from a unique prefix. A blank line, or a line whose first word starts a `#` comment, SHALL be skipped. A line whose quote is not closed SHALL fail with `Cannot split the line: a quote is not closed`, a line that is not UTF-8 with `Cannot read the line: it is not valid UTF-8`, and the loop SHALL go on after every failure.

#### Scenario: A quoted word and an open quote
- GIVEN a piped session
- WHEN the lines `token "show"`, `auth resume 'abc` and `token show` are read
- THEN the first runs `token show`, the second prints the failure object naming the open quote, and the third runs

### Requirement: Each line answers through its own printer
Each line SHALL print its result through a printer of its own, JSON when the session's `--json` or the line's is set, the following lines keeping the session's format. The loop SHALL add nothing after a result, every text output ending with its own newline, and SHALL flush stdout after each line. A failed command SHALL print the failure object of the binary's error codes through that printer on stdout, whether stdin is a terminal or not, its text form hinting `Restart the session with --log-level to enable more verbose logs` while debug logs are off, since a line cannot set `--log-level`. A command ending with a partial outcome SHALL print nothing after its result. A line clap cannot parse SHALL print clap's rendering on stderr when stdin is a terminal, and the failure object on stdout otherwise. When stdin is a terminal, the prompt `<bin>> ` SHALL render on stderr before each line.

#### Scenario: A line's JSON flag in a text session
- GIVEN a text session
- WHEN the lines `token inspect --json` and `token show` are read
- THEN the first prints its result as JSON and the second as text

#### Scenario: A failure in a text session
- GIVEN a text session started without `--log-level`
- WHEN a line fails
- THEN its failure ends with the hint `Restart the session with --log-level to enable more verbose logs`

### Requirement: Session flags are refused by name
A line clap cannot parse that sets a flag fixed for the session SHALL fail with ``Cannot set `<flag>` inside a session, start a new session with it instead``, `<flag>` being the fixed flag as listed, in place of clap's error. The flags `-a`, `--account`, `-c`, `--config`, `--log-level`, `--log` and `--log-file` SHALL always be fixed, and a binary SHALL be able to add its own. A flag SHALL be recognised alone, as `--<long>=<value>`, or as a short flag with its value attached, before any `--`.

#### Scenario: Another account on one line
- GIVEN a session
- WHEN the line `envelope list -a work` is read
- THEN it fails with ``Cannot set `-a` inside a session, start a new session with it instead``

### Requirement: Help keeps JSON lines whole
A line asking for help or the version SHALL print clap's text on stdout in a text session, and on stderr when the session's `--json` or a `--json` word of the line before any `--` is set, so stdout keeps one JSON value per line.

#### Scenario: Help in a JSON session
- GIVEN a session started with `--json`
- WHEN the line `help` is read
- THEN the help is printed on stderr and stdout gets nothing

### Requirement: Help names the binary
The help of a line SHALL render its usage line with the binary name before the command, as the command line does.

#### Scenario: A command's help
- GIVEN a session of the binary `app`
- WHEN the line `help token show` or `token show --help` is read
- THEN the usage line reads `Usage: app token show [OPTIONS]`

### Requirement: Commands never read stdin inside a session
`session::claim_stdin` SHALL mark stdin as read by a session, and the REPL and the MCP server SHALL call it before reading their stdin. From then on `session::in_session` SHALL be true and `session::check_stdin` SHALL fail with `Cannot read input from stdin inside a session, pass it as an argument or a file path`, carrying the code the binary passes.
