---
cairn: log
change: the-error-report-defuses-its-chain
landed: 2026-11-18
---

# The error report defuses its chain

Finding KIT-2 of the CLI family analysis of 2026-11-15.

## What landed

- `sanitize::lines` (src/sanitize.rs) replaces the control and bidi characters `sanitize::text` replaces, keeping the line feed and the tab.
- The text `ErrorReport` (src/error.rs) prints its message and each source through it, and adds `Run with --json to see the exact characters of the error` to its suggestions when it replaced a character. The JSON report is unchanged.
- `a_text_report_defuses_the_chain_and_keeps_its_lines` (failed before the fix: the report held the raw ESC and BEL bytes) and `sanitize_lines_keeps_line_feeds_and_tabs`.

Additive: `sanitize::lines` is new.

## Capabilities moved

- [error](../spec/error.md): *The text report is defused* added.
- [terminal](../spec/terminal.md): *Untrusted text is defused* modified.
