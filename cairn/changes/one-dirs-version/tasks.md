---
cairn: tasks
change: one-dirs-version
---

# Tasks

- [x] src/clap/parsers.rs: `shellexpand::full_with_context` with dirs 7's home directory and the environment
- [x] Cargo.toml: shellexpand `base-0` alone, dirs 7 in the `terminal` feature
- [x] Tests: `~`, `$HOME` and `${HOME}` expand to the home directory; an unset variable is rejected; both pass on the old code, and each fails when its context is broken
- [x] fmt, clippy with all features, with `terminal` alone and `build` alone, tests, cargo deny
- [x] `cargo tree -e normal --no-default-features --features terminal -i dirs@6.0.0` finds nothing
- [x] CHANGELOG under [Unreleased]
- [x] Fold delta.md into cairn/spec/terminal.md; log; status `landed`
- [ ] Left to each consumer's own change: move to a pimalaya-cli carrying this change, so dirs 6 leaves its lockfile
