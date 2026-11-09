---
cairn: log
change: logs-and-help-stay-plain
landed: 2026-11-09
---

# Logs and help stay plain

The owner decided on 2026-11-08 and 2026-11-09 that the Pimalaya CLIs print plain text and JSON, with no colour and no terminal queries. pimalaya-cli 0.3.1 forced both on every consumer of its `terminal` feature: env_logger's `auto-color` and clap's `wrap_help`.

Cargo.toml only, no code change. env_logger takes no features. clap takes `std`, `derive` and `help`: `wrap_help` implied `help` (clap_builder 4.6.7), so dropping it alone would have dropped the generated help text. Every dependency now carries `default-features = false`: anyhow `std`, clap_complete and clap_mangen none (they have no defaults), schemars `derive` and `std` (`PathBuf` in a derived schema), serde `derive` and `std`, serde_json `std` (`to_writer`), shellexpand `base-0` and `tilde` (`shellexpand::full`), toml `parse` and `serde` (`from_str` only, so `display` and `std` went), log none. The prompt, wizard, table and spinner dependencies keep their features, written out: inquire `crossterm`, `fuzzy`, `macros`, `one-liners`; crossterm `bracketed-paste`, `derive-more`, `events`, `windows`; comfy-table `tty`; ctrlc `termination`; secrecy `serde`; thiserror `std`.

Help is not wrapped at all now, rather than wrapped at a fixed width: without `wrap_help`, clap_builder's `StyledStr::wrap` and `textwrap::wrap` do nothing, and its fixed 100 columns only decide, under `-h`, whether an option's help moves under its name. Each doc comment paragraph prints on one line and the terminal folds it. Fixed-width wrapping would need `wrap_help` back, with terminal_size and rustix.

Verified: `cargo tree -e normal --no-default-features --features terminal` went from 48 crates to 38, losing anstream, anstyle-parse, anstyle-query, bitflags, colorchoice, is_terminal_polyfill, linux-raw-sys, rustix, terminal_size and utf8parse; `build` alone went from 20 to 19 (toml_writer). The resolved features of the `prompt`, `wizard`, `table` and `spinner` builds are unchanged, the `default` marker aside. Cargo.lock dropped those crates plus anstyle-wincon, once_cell_polyfill and a stale indexmap, hashbrown and equivalent that no build reached. fmt, clippy with all features, with `terminal` alone, `build` alone and each of `prompt`, `wizard`, `table`, `spinner`, `imap`, the 43 unit tests and the doc test pass; cargo deny passes with the same warnings as before. A scratch binary built against 0.3.1 and against this tree, run on a 60-column pseudo-terminal: before, a warning carried 13 escape bytes and `--help` wrapped at 60 columns; after, `[WARN  <target>] <message>` with no escape byte, and `--help` printed a 418-character paragraph as one line.

Spec: terminal (new capability, 3 requirements added).

Left: the consumers' own lines still enable what pimalaya-cli dropped: clap's defaults, colour among them, and `wrap_help` in himalaya, cardamum, calendula, neverest, carillon, sirup, journal and unfog, `wrap_help` in comodoro, env_logger's defaults in cardamum. Each drops them in its own change; Ortie already asks for neither.
