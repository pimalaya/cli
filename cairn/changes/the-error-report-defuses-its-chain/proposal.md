---
cairn: change
id: the-error-report-defuses-its-chain
status: landed
created: 2026-11-18
---

# The error report defuses its chain

## Why

The CLI family analysis of 2026-11-15 (finding KIT-2, rechecked 2026-11-16) found `ErrorReport` printing the error and each source raw in text mode (src/error.rs:289, :301), the one text path left unsanitized while item errors are (src/error.rs:374-382). An id, an attachment file name or a server's refusal text carried in an error writes its escape sequences to the terminal in every binary: `himalaya message read $'nope\033[31mRED\007'` prints the ESC and BEL bytes, and an attachment `filename*` decoding to an OSC sequence does the same through "Create attachment at ...". `sanitize::text` does not fit: it replaces the newline and the tab too, which would flatten the multi-line errors the family prints (TOML errors, JMAP failures).

## What

- `sanitize::lines`: replaces what `sanitize::text` replaces, the line feed and the tab kept.
- The text report runs the message and each source through it. When it replaced a character, its suggestions add `Run with --json to see the exact characters of the error`; the JSON report keeps the chain exact.

Additive: `sanitize::lines` is new, no public item changed.
