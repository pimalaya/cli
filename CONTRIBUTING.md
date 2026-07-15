# Contributing guide

Thank you for investing your time in contributing to Pimalaya CLI.

Whether you are a human or an AI agent, read these in order before touching the code:

1. the [Pimalaya README](https://github.com/pimalaya) for what the project is and how its repositories stack;
2. the [Pimalaya CONTRIBUTING](https://github.com/pimalaya/.github/blob/master/CONTRIBUTING.md) guide, which chains to the shared architecture and guidelines;
3. the inline header documentation, starting with src/lib.rs: it is the architecture document of this crate;
4. the docs/ folder for the development history and living plans.

Everything below documents only what differs from the Pimalaya standards.

## Deliberately std

Unlike the io-* protocol libraries, Pimalaya CLI is not no_std and never will be: it exists to talk to a terminal, a filesystem and a human, so it depends on std unconditionally. The `#![no_std]` and `extern crate alloc` conventions do not apply here.

## Feature matrix

Every tool sits behind its own feature, so binaries pull in only what they use:

```sh
cargo build                                              # every tool (default features)
cargo build --no-default-features --features build       # build-script helpers only
cargo build --no-default-features --features terminal    # clap glue, printer, logger, error report
cargo build --no-default-features --features spinner      # cancellable spinner
cargo build --no-default-features --features table        # formatted tables
cargo build --no-default-features --features imap         # account wizard (pulls in wizard)
```

The `imap`, `smtp`, `jmap`, `caldav` and `carddav` features each enable the `wizard` feature, since they only add their protocol-specific wizard submodule.
