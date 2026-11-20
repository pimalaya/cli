---
cairn: change
id: the-contract-audit-rulings
status: landed
created: 2026-11-20
---

# The contract audit rulings

## Why

The CLI contract audit of 2026-11-20 checked the seven binaries against this kit's spec and found points the kit settles once for all: five copies of the bare invocation, drifting already (`<bin> --json` printed the help as text); `manual`, `completion` and `json-schema` run without their argument exiting 1 uncoded; the MCP grammar answering clap's text for a refused value of an optional positional (comodoro's `[TRANSPORT]`, COM-X4); `<cmd> --version` naming clap's subcommand (`cardamum-account-list v0.3.0`). The owner's rulings of 2026-11-20 add contract lines: extra keys on shared nouns (A1), a `tls` object on the check row with himalaya's keys (A1), one `errors` list whose rows may carry a `kind` tag (A2.3), progress lines on a pipe only for long-lived commands (A2.4), `account check` printing a list of accounts in every binary (A2.5), the apps' `<bin> pimdir queue list` as the reference queue row, `pimdir` free to derive (A3, A4).

## What

- `account::AccountCheckOutput` becomes `{accounts: [AccountCheck]}`, `AccountCheck` being `{account, backends}`; one block of text per account. `AccountCheckOutput::ok` tells whether every backend answered.
- `account::BackendCheck` gains `tls: Option<BackendCheckTls>`, `{provider, crypto, cert, clientCert}`, `null` for a local backend or a cleartext connection; its text appends `, TLS` and the parts set.
- `clap::meet_bare_invocation(printer, command, config_missing)`: under JSON a failure coded `usage-invalid`, exit 2; in text the missing configuration (the welcome) or the help.
- `completion`, `manual` and `json-schema` require their names or `--dir` (a clap group), and their later refusals (several names without `--dir`, an unknown name) are coded `usage-invalid`, exit 2.
- `mcp::grammar::Grammar::usage_error` reads `[NAME]` as well as `<NAME>`.
- `clap::parse` and the REPL name every subcommand after the binary, so the version after a subcommand prints `<bin> <version>`.
- Spec only: extra keys on shared nouns; the `errors` list exit status and its `kind` tag; the queue listing `{actions, parked, errors}` and `queue show`; capability names `<noun>.<verb>.<argument>`; streaming reserved to long-lived commands.
