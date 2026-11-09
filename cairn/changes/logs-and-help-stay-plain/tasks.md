---
cairn: tasks
change: logs-and-help-stay-plain
---

# Tasks

- [x] Cargo.toml: `default-features = false` on every dependency, only the features the code uses
- [x] env_logger without `auto-color` and `color`; clap `help` in place of `wrap_help`
- [x] Prompt, wizard, table and spinner dependencies keep today's features, written out
- [x] Check help output and log lines from a scratch binary on a pseudo-terminal
- [x] fmt, clippy with all features, clippy with `terminal` alone and `build` alone, tests, cargo deny
- [x] `cargo tree` with `terminal` alone shows no anstream, anstyle-query, colorchoice, is_terminal_polyfill, terminal_size, rustix
- [x] CHANGELOG under [Unreleased]
- [x] Fold delta.md into cairn/spec/terminal.md; log; status `landed`
- [ ] Left to each consumer's own change: drop clap's defaults and `wrap_help`, and env_logger's defaults, from its own Cargo.toml
