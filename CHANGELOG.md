# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Added `columns::Columns`, a text list printed as borderless aligned columns under a header row.

  Columns are two spaces apart, widths count terminal display columns (unicode-width), an empty cell prints as `-` and a line break inside a cell as a space. Nothing is truncated or sized to the terminal, and the last column is never padded.

- Added `shell::quote`, quoting a string for a paste into a POSIX shell, or into cmd and PowerShell on Windows.
- Added `shell::invocation`, the binary with its quoted `-c` and `-a` flags that a printed command starts with.
- Added `welcome::Welcome`, the first-run welcome naming the missing configuration, the commands creating it from the sample configuration, and the first command to run.

  It creates the file from the shipped sample with `install -m 644` (`copy` on Windows), or downloads it with `curl` when none ships.

- Added `welcome::shipped_sample`, finding the share/doc/<bin>/config.sample.toml a release or a package ships next to the binary.
- Added `welcome::is_wanted`, telling whether a person reads the output: the printer is not JSON and stderr is a terminal.
- Added the `repl` feature and `repl::Repl`, a read-eval-print loop running a binary's own commands over stdin.

  Each line is split as a shell splits it and parsed with the binary's `clap::Subcommand` tree plus `--json`, which switches that line alone to JSON. Results and failure objects go to stdout, a partial outcome prints its result alone, a session flag such as `-a` or `-c` on a line is refused by name, and help in a JSON session goes to stderr.

- Added `session::claim_stdin`, `session::check_stdin` and `session::in_session`, keeping the commands a REPL or an MCP server runs off its stdin.
- Added the `mcp` feature and the `mcp` module, a Model Context Protocol server over stdio.

  `mcp::read` answers the protocol (parse errors, invalid requests and batches, `initialize` with version negotiation, `ping`, `tools/list`) and checks each tool call, its undeclared arguments refused. `mcp::ToolResult::printed` turns what a command printed into a tool result. `mcp::serve` runs a blocking loop, `mcp::spawn_stdin_reader` feeds a binary's own channel, and `mcp::exit_on_failure` reports a failure on stderr, stdout carrying protocol messages only.

- Added `mcp::grammar::Grammar`, a tool's input schema derived from a clap command and the argv a call turns back into.
- Added `mcp::tool_output_schema`, keeping a JSON Schema as a tool's `outputSchema` only when its root is an object.
- Added `mcp::error_object`, the failure object of an error as a failed tool call holds it.
- Added `printer::CapturePrinter`, keeping what a command prints as JSON values, whose `into_single` fails naming the number of values printed.

### Changed

- **BREAKING**: Changed `clap::args::ConfigFlags` to take one path per `-c`, no longer splitting a value on `:`.

  Repeat the flag to merge several files. A Windows path, or any path holding `:`, now stays whole. Its help no longer mentions the wizard.

- **BREAKING**: Moved `table::sanitize` to `sanitize::text`, available with the `terminal` feature.
- **BREAKING**: Changed the `completion`, `json_schema` and `manual` modules of `clap::commands` to public, dropping the re-exports of their items.

  Import `clap::commands::completion::CompletionCommand`, `clap::commands::manual::ManualCommand` and `clap::commands::json_schema::{JsonSchemaCommand, JsonSchemaRegistry, output_schema}`.

### Removed

- **BREAKING**: Removed the `wizard` module, the `wizard`, `imap`, `smtp`, `jmap`, `caldav` and `carddav` features, and the `imap-wizard` example.

  Wizards leave the Pimalaya CLIs: a first run without a configuration prints a `welcome::Welcome` instead.

- **BREAKING**: Removed `prompt::u16`, `prompt::usize`, `prompt::secret`, `prompt::some_secret`, `prompt::password` and `prompt::items`, with the `PromptError` variants only they raised and the secrecy dependency.

  `prompt::bool`, `prompt::text`, `prompt::some_text` and `prompt::item` stay.

- **BREAKING**: Removed the `validator` module.
- **BREAKING**: Removed the `table` module and feature, and the comfy-table dependency.

  A list prints as `columns::Columns`, a record as `fields::Fields`.

- **BREAKING**: Removed `printer::PrintTable`, which nothing implemented.

## [0.3.2] - 2026-11-11

### Added

- Added `fields::Fields`, a text record printed as `Label: value` lines aligned the way clap aligns its help.
- Added `wizard::keyring::KeyringProvider::store`, storing a secret through the provider's CLI.

  The secret goes on the program's stdin, never on its arguments.

- Added `wizard::keyring::prompt_password`, asking once for a password after naming the endpoints it goes to.

  The line of an endpoint outside the address's domain says so.

- Added `wizard::keyring::prompt_store`, storing a tested password in an installed keyring and reading it back.

  A keyring failing either step is reported and dropped from the list, which also offers a custom command and the raw value.

- Added `wizard::review::prompt`, listing a wizard's picks and asking whether to continue or which one to change.
- Added `wizard::review::ReviewRow`, a row of the review.
- Added `wizard::review::ReviewAction`, the answer to the review.
- Added `wizard::ortie::sign_in`, handing an OAuth 2.0 sign-in off to Ortie.

  Ortie's output goes to stderr, so a redirected stdout keeps only what the caller prints.

- Added `wizard::ortie::OrtieSignInError`.

### Changed

- Changed the `terminal` feature to stop enabling env_logger's `auto-color` feature, so log lines carry no colour.

  A binary wanting coloured logs enables env_logger's `auto-color` feature itself.

- Changed the `terminal` feature to enable clap's `help` feature in place of `wrap_help`, so help no longer asks the terminal for its size and prints each paragraph on one line.

  A binary wanting help wrapped to the terminal enables clap's `wrap_help` feature itself.

- Changed every dependency to `default-features = false`, enabling only the features the code uses.

  A binary built with `terminal` alone links 10 fewer crates (anstream, anstyle-parse, anstyle-query, bitflags, colorchoice, is_terminal_polyfill, linux-raw-sys, rustix, terminal_size, utf8parse), and one built with `build` alone drops toml_writer.

- Changed `clap::parsers::path_parser` to take the home directory from dirs 7, dropping shellexpand's `tilde` feature, which pinned dirs below 7.

  A binary already on dirs 7 no longer carries dirs 6 beside it. `~` and variable expansion are unchanged.

- Changed `build::git_envs` to run the git command in place of git2, so the `build` feature compiles no C.

  `GIT_DESCRIBE` comes from `git describe --all --always` and `GIT_REV` from `git rev-parse HEAD`, run on the repository whose .git sits in the build script's working directory, never a parent's. A build script using `build` alone links 11 fewer crates (bitflags, cc, find-msvc-tools, git2, jobserver, libc, libgit2-sys, libz-sys, pkg-config, shlex, vcpkg).

### Fixed

- Fixed `build::git_envs` panicking in a repository it cannot describe, such as one without commits: `GIT_DESCRIBE` and `GIT_REV` are `unknown` there, as outside a repository or without git.

## [0.3.1] - 2026-11-03

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

[unreleased]: https://github.com/pimalaya/cli/compare/v0.3.2..HEAD
[0.3.2]: https://github.com/pimalaya/cli/compare/v0.3.1..v0.3.2
[0.3.1]: https://github.com/pimalaya/cli/compare/v0.3.0..v0.3.1
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
