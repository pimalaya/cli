---
cairn: change
id: the-manual-names-the-version-once
status: landed
created: 2026-11-18
---

# The manual names the version once

## Why

The CLI family analysis of 2026-11-15 (finding KIT-7, rechecked 2026-11-16) found every generated manual page of the seven binaries (556 pages) printing `vv<version>` in its VERSION section. clap_mangen writes that section as `v` followed by the long version (clap_mangen 0.3.3, src/render.rs:364-371), and `long_version!` starts with its own `v` (src/clap/args.rs:129), which `--version` needs.

## What

- `manual` drops the `v` leading a long version followed by a digit from each page's command before rendering it, so the section reads `v1.2.3 ...`; `--version` keeps its text.
- clap is asked for its `string` feature (a `Str` from a `String`, no new crate), to set that version on the page's command.

Rejected: dropping the `v` from `long_version!`, which would change every binary's `--version` text to fix a page.

Non-breaking: no public item changed.
