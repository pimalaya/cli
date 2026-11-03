# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Changed the base64 encoding of `output::Bytes` from base64 to base64-codec.

  It runs on the CPU token the binary installs with `simd_cpu::SimdCpu::install_detected`, the portable baseline when none is.

- Raised the minimum supported Rust version from 1.88 to 1.89.

## [0.3.0] - 2026-11-03

### Added

- Added the `secret-tool` attribute prompt to `wizard::keyring`, defaulting to `account` ([pimalaya/himalaya#745](https://github.com/pimalaya/himalaya/issues/745)).
- Added `error::ErrorCode`, the trait of a binary's error codes.
- Added `error::NoErrorCode`, the code type of a binary without codes.
- Added `error::CodedError`, an error carrying a code.
- Added `error::code_of`, finding the code in an error chain.
- Added `error::ErrorReport::eval_coded`, printing the failure object with its code.
- Added `error::PartialOutcome`, exiting 2 or above after a printed result.
- Added `error::partial_outcome_of`, finding the partial outcome in an error chain.
- Added `error::EXIT_FAILURE` and `error::EXIT_PARTIAL`.
- Added the `JsonSchema` implementation of `error::ErrorReport`.
- Added `printer::Printer::event`, printing an event before the result.

  Under `--json` it prints one JSON line, in text mode on a terminal only.

- Added `printer::Printer::out_bytes`, printing bytes, base64-encoded under `--json`.
- Added `output::Bytes`, the payload of `out_bytes`.
- Added `output::Paginated`, a listing with its `nextPage` cursor.
- Added `clap::args::PageTokenArg`, the `--page-token` argument.
- Added `clap::commands::JsonSchemaRegistry`, the result and event schemas of a binary.
- Added `clap::commands::output_schema`, building the schema of a command output.

### Changed

- **BREAKING**: Changed `printer::Printer` to require `event` and `out_bytes`.
- **BREAKING**: Changed `clap::commands::JsonSchemaCommand::execute` to take a `JsonSchemaRegistry`.

  Each schema gets an `$id` carrying the binary's contract version (`v<major>`, `v0.<minor>` before 1.0).

- Changed the `--json` failure object to carry a `code`, `null` when none applies.
- Changed piped `--json` output to newline-delimited JSON.

### Removed

- **BREAKING**: Removed `printer::Message`.

## [0.2.6] - 2026-10-02

### Added

- Added `table::sanitize`, replacing control and bidi characters with U+FFFD ([pimalaya/himalaya#771](https://github.com/pimalaya/himalaya/pull/771)).

## [0.2.5] - 2026-09-27

### Changed

- Raised the minimum supported Rust version from 1.87 to 1.88.

### Fixed

- Fixed `wizard::keyring::KeyringProvider::write_command` omitting the macOS Keychain service ([pimalaya/ortie#15](https://github.com/pimalaya/ortie/issues/15)).

## [0.2.4] - 2026-08-24

### Fixed

- Fixed the logger printing library errors when neither `--log-level` nor `RUST_LOG` is given.
- Fixed error reports suggesting `--log-level` when `--log-level debug` was already given.

## [0.2.3] - 2026-08-23

### Changed

- Changed `clap::commands::CompletionCommand`, `ManualCommand` and `JsonSchemaCommand` to take a positional selection and an optional `--dir`.

  Without `--dir`, the one selected item goes to stdout. `<cmd> manual ./man` becomes `<cmd> manual --dir ./man`.

- Changed their `--json` output to `{shell, script}`, `{cmd, page}` and `{cmd, schema}`, or `{dir, scripts | pages | schemas}` with `--dir`.

## [0.2.2] - 2026-08-16

### Added

- Added `wizard::keyring::KeyringProvider::installed`, telling whether the provider's CLI is on the `PATH`.
- Added `wizard::keyring::TokenBroker::installed`, telling whether the broker's CLI is on the `PATH`.

### Changed

- Changed `wizard::keyring::KeyringProvider::available` and `TokenBroker::available` to list installed entries first.

  Missing entries stay offered, labelled `not found on PATH`.

## [0.2.1] - 2026-08-14

### Added

- Added `footer!`, building the `--help` footer every Pimalaya binary shares.

### Changed

- Documented the crate as internal to Pimalaya.

### Fixed

- Fixed `error::ErrorReport::eval` panicking when stdout is gone.
- Fixed `wizard::keyring` compiling empty without the `imap`, `smtp` or `jmap` feature.

## [0.2.0] - 2026-08-07

### Changed

- **BREAKING**: Changed the `comfy-table` dependency, re-exported by `table`, from 7 to 8.

  `Table::load_preset(&str)` becomes `Table::load_style(TableStyle)`, and the truncation indicator `...` becomes `…`.

## [0.1.3] - 2026-07-25

### Added

- Added `clap::commands::JsonSchemaCommand`, generating one JSON Schema file per command output.

## [0.1.2] - 2026-07-25

### Added

- Added `wizard::keyring`, an OS-aware credential provider picker for the account wizards.

### Changed

- Changed the `imap` and `smtp` wizards to read credentials through `wizard::keyring`.

## [0.1.1] - 2026-07-17

### Added

- Added `prompt::items`, a multi-select over `inquire::MultiSelect`.

### Changed

- Changed the default features to the full toolkit.

### Removed

- Removed filtering from `prompt::item` and `prompt::items`.

## [0.1.0] - 2026-07-15

### Added

- Added documentation to every public item.

### Changed

- Relicensed under `MIT OR Apache-2.0`.
- Changed the `imap`, `smtp`, `jmap`, `caldav` and `carddav` features to enable `wizard`.

## [0.0.2] - 2026-07-12

### Added

- Added the initial toolkit: clap arguments and commands, printer, logger, prompt, spinner, table, account wizards, error reporting, validators and build helpers.

[unreleased]: https://github.com/pimalaya/cli/compare/v0.3.0..HEAD
[0.3.0]: https://github.com/pimalaya/cli/compare/v0.2.6..v0.3.0
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
