---
cairn: log
change: the-contract-audit-rulings
landed: 2026-11-20
---

# The contract audit rulings

Section B of the CLI contract audit of 2026-11-20 and the owner's rulings on A1 to A4 (plans ledger cli-contract-audit-2026-11-20, proposal.md).

## What landed

- `account check`: `AccountCheckOutput` is `{accounts: [{account, backends}]}`, every binary printing a list, of one for the account `-a` names; the text prints one `Fields` block per account, a blank line apart, so one account reads as before. `BackendCheck` gains `tls`, `BackendCheckTls {provider, crypto, cert, clientCert}`, `null` from `BackendCheck::new` and set by the binary on a connection over TLS, its text appended as `, TLS native-tls, cert /etc/ca.pem` (`, TLS` alone when the build's defaults answer every part). himalaya's `TlsTrust` is the model; its `tls: null` on a cleartext connection is the rule. Breaking for the crate's API (fields and shape), the binaries' JSON breaking with it.
- `clap::meet_bare_invocation` replaces the five copies (himalaya, cardamum, calendula, carillon, neverest): under JSON it fails `A command is required, see --help`, coded `usage-invalid`, exit 2, before reading the configuration (carillon's CRL-X3 saw the help printed as text on stdout, exit 0); in text it answers the missing configuration the binary's closure gives, which `main` turns into the welcome, or prints the help.
- `completion`, `manual` and `json-schema` carry a required clap group over their names and `--dir`: a bare run is refused by clap, `<SHELLS|--dir <PATH>>`, exit 2 and `usage-invalid` under `--json` (it exited 1 uncoded, naming `--dir`, in every binary). Several names without `--dir` and an unknown page or schema name now fail coded `usage-invalid`, exit 2, through `failure_status`.
- `Grammar::usage_error` reads a positional rendered `[NAME]` (an optional one) as well as `<NAME>`: comodoro's `get` with `transport: "udp"` reads ``Invalid value `udp` for `transport`: the possible values are `socket`, `tcp` `` (COM-X4). Test `a_refused_value_of_an_optional_positional_names_its_property`, failing before the fix with clap's text.
- `--version` after a subcommand: clap prints a subcommand's display name, which its build sets to the dashed path. `clap::parse` and the REPL now give every subcommand the root's name as display name before parsing; `manual` builds its own tree, so its page names keep the dashed path. Tests `the_version_after_a_subcommand_names_the_binary` and the tightened `a_version_line_prints_the_version_the_tree_carries` (`app-echo v1.2.3 +feature` before), both failing on the old code. No binary change needed beyond calling `clap::parse`.
- Spec only: a shared noun may carry the binary's keys after the shared ones (A1); one `errors` list, exit 3 or the command's documented status above it, its rows optionally tagged `kind` with keys of their own (A2.3, neverest's named lists not admitted); streaming reserved to long-lived commands, a check or a bounded step run alone never streaming (A2.4); the reference queue row is the apps' `<bin> pimdir queue list`, printing `{actions, parked, errors}`, `queue show` the list row plus what a read carries, an unknown row `row-not-found`, exit 1, the `pimdir` binary free to derive (A3, A4); capability names `<noun>.<verb>` or `<noun>.<verb>.<argument>` (cardamum, calendula X1).

## Capabilities moved

- [output](../spec/output.md): *A write answers a receipt*, *Shared nouns keep one shape*, *A queue row keeps one shape*, *A partial outcome prints its result*, *Events precede the result*, *A long-lived command streams events* modified; *A queue read prints the list row* added.
- [error](../spec/error.md): *A refused command line exits 2*, *An item error is the failure object of one item* modified; *A command is required under JSON*, *A standard command needs its argument* added.
- [terminal](../spec/terminal.md): *An account's capabilities print one line each*, *The accounts and their checks print alike* modified.
- [commands](../spec/commands.md): *Every binary carries an account group* modified.
- [repl](../spec/repl.md): *Help keeps JSON lines whole* modified.
- [mcp](../spec/mcp.md): *A refused call names its arguments* modified.
