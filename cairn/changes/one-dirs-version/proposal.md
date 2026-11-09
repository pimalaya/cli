---
cairn: change
id: one-dirs-version
status: landed
created: 2026-11-09
---

# One dirs version

## Why

`clap::parsers::path_parser` calls `shellexpand::full`, which exists only with shellexpand's `tilde` feature, and shellexpand 3.1.2 pins that feature's dirs to `>=4, <7`. pimalaya-config and the Pimalaya binaries already use dirs 7, so every binary built with the `terminal` feature carries dirs 6 beside dirs 7, and `cargo deny check` warns about the duplicate in each consumer.

## What

- `path_parser` calls `shellexpand::full_with_context` with the two contexts `full` builds: the home directory from dirs 7, as a string only when it is valid Unicode, and the environment through `std::env::var`, an unset variable being an error. The parser behaves as before.
- shellexpand takes `base-0` alone; the `terminal` feature gains dirs 7, `default-features = false` (it has no features).
- Tests guard what the swap could break: `~`, `$HOME` and `${HOME}` expanding to the home directory, and an unset variable rejected with the same message as before.

Out of scope: pimalaya-config, fixed in its own one-dirs-version change, and the consumers moving to a pimalaya-cli that carries this change.
