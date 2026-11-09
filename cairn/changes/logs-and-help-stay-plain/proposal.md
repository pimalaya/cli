---
cairn: change
id: logs-and-help-stay-plain
status: landed
created: 2026-11-09
---

# Logs and help stay plain

## Why

The owner decided on 2026-11-08 and 2026-11-09 that the Pimalaya CLIs print plain text and JSON, with no colour and no terminal queries. pimalaya-cli 0.3.1 works against that through its `terminal` feature: env_logger takes `auto-color`, which colours the level of each log line on a terminal, and clap takes `wrap_help`, which asks the terminal for its width before printing help. A consumer cannot turn either off, since Cargo features only add up.

Measured on Ortie's release binary, colour and terminal width cost about 22 KB and 10 crates: anstream, anstyle-parse, anstyle-query, colorchoice, is_terminal_polyfill, utf8parse, terminal_size, rustix, linux-raw-sys, bitflags.

## What

- env_logger takes no features: log lines carry no ANSI colour, on a terminal or not.
- clap takes `std`, `derive` and `help`, dropping `wrap_help`. `help` was enabled through `wrap_help` and stays, so `--help` keeps its generated text. Without `wrap_help` clap neither queries the terminal size nor wraps help text: each paragraph prints as one line, and the terminal folds long lines.
- Every dependency takes `default-features = false` and names the features the code uses: anyhow `std`, schemars `derive` and `std`, serde `derive` and `std`, serde_json `std`, shellexpand `base-0` and `tilde` (`shellexpand::full`), toml `parse` and `serde` (`toml::from_str`). log, git2, clap_complete and clap_mangen have no default features; the flag makes that explicit.
- The `prompt`, `wizard`, `table` and `spinner` dependencies keep exactly today's features, written out: inquire, crossterm, comfy-table, ctrlc, secrecy (`serde`), thiserror (`std`). Their removal is a separate change.

A consumer's own clap or env_logger line can still enable colour or `wrap_help`; this change only stops pimalaya-cli from forcing them.

Out of scope: the consumers' own Cargo.toml lines, and the removal of the prompt, wizard, table and spinner features.
