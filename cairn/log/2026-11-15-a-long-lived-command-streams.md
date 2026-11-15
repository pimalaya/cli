---
cairn: log
change: a-long-lived-command-streams
landed: 2026-11-15
---

# A long-lived command streams, a deep code reaches the report

Owner rulings of 2026-11-15 on two family gaps, items 10 and 13 of the merged list.

## What landed

- `error::with_code<C>(err, code)`, lifted from ortie (src/error.rs, `with_code`): when `code` finds a code the chain's `CodedError`s do not carry, the chain is rebuilt with the same messages over an innermost `CodedError<C>`. A chain already coded `C` or `usage-invalid`, or holding a `PartialOutcome`, is returned unchanged, so a refused line still exits 2 and a partial outcome keeps its status. No signature changed.
- `signal::Shutdown` behind the new `signal` feature (ctrlc, `termination`), which `spinner` now enables. One process-wide handler: without a token, a signal restores the terminal and exits 130, as the spinner did; with one, the first signal sets the request, the second exits 130. The spinner registers its cursor restore with that handler instead of installing its own, which ctrlc refuses twice.
- `StdoutPrinter::streaming()`: text events reach a stdout that is no terminal.
- `a_code_read_deep_in_the_chain_reaches_the_report`.

## Capabilities moved

- [output](../spec/output.md): *A long-lived command streams events* added.
- [error](../spec/error.md): *Failures carry an optional stable code* names `with_code`.
