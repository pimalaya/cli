# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.6] - 2026-10-02

### Added

- Added `table::sanitize`, which replaces control and bidi characters with U+FFFD so strings from a message or a server cannot drive the terminal from a table cell. See [himalaya#771](https://github.com/pimalaya/himalaya/pull/771).

## [0.2.5] - 2026-09-27

### Changed

- Raised the minimum supported Rust version to 1.88.

### Fixed

- Fixed the macOS Keychain write command generated without a service: `security add-generic-password` requires `-s`, so the entry now names both the service and the account. See [ortie#15](https://github.com/pimalaya/ortie/issues/15).

## [0.2.4] - 2026-08-24

### Fixed

- Silenced the logger when neither `--log-level` nor `RUST_LOG` is given. `env_logger` falls back to the `error` level when no directive is set, so error lines coming from the libraries were printed to the standard error stream on a bare invocation. The level now starts at `off`, `RUST_LOG` raising it back as before.
- Stopped suggesting `--log-level` in error reports once `--log-level debug` was already given.

## [0.2.3] - 2026-08-23

### Changed

- Gave `CompletionCommand`, `ManualCommand` and `JsonSchemaCommand` the same shape: a positional list selecting what to generate, defaulting to everything, and an optional `--dir` deciding where it lands. Without a directory the single selected item is printed to the standard output, ready for unix redirection, and asking for several at once fails rather than picking one arbitrarily or concatenating them.

  `CompletionCommand` used to default its directory to the working one, which silently turned `<cmd> completion bash` into a file-writing command printing a report, against what its own documentation said. It broke every packaging helper capturing stdout, Homebrew's `generate_completions_from_executable` among them.

  `ManualCommand` and `JsonSchemaCommand` take their directory through `--dir` too, where it was a required positional argument: `<cmd> manual ./man` becomes `<cmd> manual --dir ./man`, and both now accept command names (`himalaya`, `himalaya-envelope`, `himalaya-envelope-list`) to generate a single page or schema. An unknown name fails instead of generating nothing.

  Under `--json`, the item is wrapped in `{ "shell": ..., "script": ... }`, `{ "cmd": ..., "page": ... }` and `{ "cmd": ..., "schema": ... }` so the flag keeps its promise. The directory mode of the three now reports the same `{ "dir": ..., "scripts" | "pages" | "schemas": [...] }` structure, where the last two used to be a bare sentence.

## [0.2.2] - 2026-08-16

### Added

- Added `KeyringProvider::installed` and `TokenBroker::installed`, telling whether an entry's CLI is found on the `PATH`. Each derives the program name from its own read command, so no table of binary names is kept beside the commands themselves.

### Changed

- Ordered the credential picker by what the running system can actually do: `KeyringProvider::available` and `TokenBroker::available` now lead with the entries whose CLI is installed, the rest following in their wake. The sort is stable, so the native-first order of the providers and the reference-implementation-first order of the brokers survive inside each group.

  Nothing is hidden: an entry that is not there is still offered, labelled `, not found on PATH` in the pick list, since installing it afterwards makes the very same configuration work. The lookup also tries the Windows extensions (`.exe`, `.cmd`, `.bat`), where a bare name is not a filename and the token brokers are present even though the keyring providers are not.

## [0.2.1] - 2026-08-14

### Added

- Added `footer!`, a macro building the `--help` footer every Pimalaya binary shares.

  It expands to the bug tracker of the calling crate, taken from its `CARGO_PKG_REPOSITORY`, the sponsoring page, and the line saying the program is funded by grants and donations. Binaries pass it to clap as `#[command(after_help = footer!())]`, so the links land on the screen a bare invocation prints rather than only in a README nobody reaches from a terminal.

### Changed

- Documented the crate as internal to Pimalaya: its API follows the needs of its consumers and may change without notice, so third parties should treat it as private. The io-* protocol crates and pimalaya-stream are the supported surface.

### Fixed

- Fixed a panic when a failing command could not print its error report, which happens whenever stdout is gone (a closed pipe, most often): `ErrorReport::eval` unwrapped the write and died with "Print JSON to stdout error: Broken pipe" instead of the error the user was asking about. The write failure is now swallowed and the process still exits 1.

- Made `wizard::keyring` reachable from every wizard, not just the mail ones.

  Its module declaration is gated on `wizard`, but each item inside it carried a second `any(imap, smtp, jmap)` gate, so a consumer enabling only `caldav` or `carddav` compiled the module and found it empty. The picker is protocol-agnostic, so the inner gates are gone and the module declaration is the only gate, as elsewhere in the crate.

## [0.2.0] - 2026-08-07

### Changed

- **BREAKING** Bumped `comfy-table` from v7 to v8, which `table` re-exports wholesale.

  v8 removes the positional preset string: `Table::load_preset(&str)` becomes `Table::load_style(TableStyle)`, the `presets::*` constants change type from `&str` to `TableStyle`, and `Table::{set_style, style, remove_style, current_style_as_preset, apply_modifier}`, the `modifiers` module and the `TableComponent` enum are gone. Rounded corners and solid inner borders are now `TableStyle::{with_rounded_corners, with_solid_inner_borders}`. The default truncation indicator changed from `...` to `…`.

## [0.1.3] - 2026-07-25

### Added

- Added `clap::commands::JsonSchemaCommand`, a ready-made command generating one JSON Schema file per command output to a directory.

  It mirrors `ManualCommand`: the binary passes a map of command name to that command's output schema (as a `serde_json::Value`), and the command serialises each entry to `<name>.json`, reporting how many landed where. The toolkit stays schema-library-agnostic by taking pre-built `serde_json::Value` schemas rather than depending on schemars.

## [0.1.2] - 2026-07-25

### Added

- Added `wizard::keyring`, an OS-aware credential-provider picker shared by the account wizards.

  A password is read from a well-known keyring CLI (`secret-tool`, `kwallet-query`, `security` or `pass`), an OAuth 2.0 access token from a well-known token broker (`ortie`, `pizauth` or `oama`), a custom shell command, or stored raw in the configuration. Known providers and brokers yield an argv command serialized as a TOML array; only custom commands fall back to a shell string.

### Changed

- Reworked the `imap` and `smtp` wizards to read credentials through the new `wizard::keyring` picker.

## [0.1.1] - 2026-07-17

### Added

- Added `prompt::items`, a multi-select wrapping `inquire::MultiSelect`: it returns the chosen items and takes the item indices selected by default.

### Changed

- Removed filtering from `prompt::item` and `prompt::items`.
- Enabled the full toolkit by default, so `cargo add pimalaya-cli` pulls every feature; consumers selecting features keep using `default-features = false`.

## [0.1.0] - 2026-07-15

### Added

- Documented every public item, so docs.rs now builds the full API reference.

### Changed

- Relicensed from MIT to `MIT OR Apache-2.0`, matching the rest of the Pimalaya crates.
- Made the `imap`, `smtp`, `jmap`, `caldav` and `carddav` features enable `wizard`, since each only gates its wizard submodule and was inert without it.

## [0.0.2] - 2026-07-12

### Added

- Initial published toolkit: clap arguments and commands, printer, logger, prompt, spinner, table, account wizards, error reporting, validators and build helpers.

[unreleased]: https://github.com/pimalaya/cli/compare/v0.2.6..HEAD
[0.2.6]: https://github.com/pimalaya/cli/compare/v0.2.5..v0.2.6
[0.2.5]: https://github.com/pimalaya/cli/compare/v0.2.4..v0.2.5
[0.2.4]: https://github.com/pimalaya/cli/compare/v0.2.3..v0.2.4
[0.2.3]: https://github.com/pimalaya/cli/compare/v0.2.2..v0.2.3
[0.2.2]: https://github.com/pimalaya/cli/compare/v0.2.1..v0.2.2
[0.2.1]: https://github.com/pimalaya/cli/compare/v0.2.0..v0.2.1
[0.2.0]: https://github.com/pimalaya/cli/compare/v0.1.3..v0.2.0
[0.1.3]: https://github.com/pimalaya/cli/compare/v0.1.2..v0.1.3
[0.1.2]: https://github.com/pimalaya/cli/compare/v0.1.1..v0.1.2
[0.1.1]: https://github.com/pimalaya/cli/compare/v0.1.0..v0.1.1
[0.1.0]: https://github.com/pimalaya/cli/compare/v0.0.2..v0.1.0
[0.0.2]: https://github.com/pimalaya/cli/compare/root..v0.0.2
