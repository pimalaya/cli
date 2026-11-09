---
cairn: log
change: one-dirs-version
landed: 2026-11-09
---

# One dirs version

`clap::parsers::path_parser` (src/clap/parsers.rs) calls `shellexpand::full_with_context` with dirs 7's home directory and the environment, in place of `shellexpand::full`, and shellexpand is built with `base-0` alone. Its `tilde` feature pinned dirs to `>=4, <7` (shellexpand 3.1.2), so every binary built with `terminal` carried dirs 6 beside the dirs 7 of pimalaya-config and the binaries themselves. pimalaya-config landed the same fix today (its one-dirs-version change).

The two contexts are the ones `full` builds in shellexpand's src/funcs.rs: the home directory is `dirs::home_dir()` turned into a `String` with `into_os_string().into_string().ok()`, so a home directory that is not valid Unicode leaves the `~` as written, and variables come from `std::env::var(name).map(Some)`, so an unset variable is still an error, `error looking key '<NAME>' up: environment variable not found`. dirs 6.0.0 and 7.0.0 share `home_dir` and dirs-sys 0.5.0 (their sources differ only in the Windows `preference_dir`). dirs has no features; it is declared `default-features = false` like every other dependency.

Tests: `tilde_and_home_variable_expand_to_the_home_directory` (Unix: `~/x`, `$HOME/x` and `${HOME}/x` give `$HOME` joined with `x`) and `an_unset_variable_is_rejected` (the exact message). Both were written first and passed on `shellexpand::full`; each fails when its context is broken (a home directory closure returning `None`, a lookup returning `Ok(None)` for an unset variable).

Verified: `cargo tree -e normal --no-default-features --features terminal -i dirs@6.0.0` finds nothing (dirs 6.0.0 left Cargo.lock, replaced by 7.0.0). With `-e normal,build --features terminal` the tree stays at 38 crates, dirs 7 in place of dirs 6; a consumer already on dirs 7 links one dirs less. fmt, clippy `-D warnings` with all features and all targets, with `terminal` alone and `build` alone, the 45 unit tests and the doc test pass; cargo deny passes with the same warnings as before.

Spec: terminal (2 requirements added: "Path arguments expand like a shell", "One dirs version").

Left: each consumer moves to a pimalaya-cli carrying this change for dirs 6 to leave its lockfile.
