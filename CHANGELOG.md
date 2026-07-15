# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-07-15

### Added

- Documented every public item, so docs.rs now builds the full API reference.

### Changed

- Relicensed from MIT to `MIT OR Apache-2.0`, matching the rest of the Pimalaya crates.
- Made the `imap`, `smtp`, `jmap`, `caldav` and `carddav` features enable `wizard`, since each only gates its wizard submodule and was inert without it.

## [0.0.2] - 2026-07-12

### Added

- Initial published toolkit: clap arguments and commands, printer, logger, prompt, spinner, table, account wizards, error reporting, validators and build helpers.

[unreleased]: https://github.com/pimalaya/cli/compare/v0.1.0..HEAD
[0.1.0]: https://github.com/pimalaya/cli/compare/v0.0.2..v0.1.0
[0.0.2]: https://github.com/pimalaya/cli/compare/root..v0.0.2
