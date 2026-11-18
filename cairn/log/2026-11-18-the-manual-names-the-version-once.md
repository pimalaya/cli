---
cairn: log
change: the-manual-names-the-version-once
landed: 2026-11-18
---

# The manual names the version once

Finding KIT-7 of the CLI family analysis of 2026-11-15, its `vv` half.

## What landed

- `manual` (src/clap/commands/manual.rs) sets each page's long version without the `v` leading a digit before rendering, so clap_mangen's own `v` makes the VERSION section read `v<version>`. `--version` is unchanged.
- clap's `string` feature in Cargo.toml (no new crate), so the page's command takes that version as a `String`.
- `the_version_section_reads_the_long_version_once_prefixed`, failing before the fix (`vv1.2.3 +imap`).

Non-breaking. Still open from KIT-7 (GAP-P3): no page has an ENVIRONMENT, FILES, EXIT STATUS or EXAMPLES section.

## Capabilities moved

- [terminal](../spec/terminal.md): *Every command has a manual page* modified.
