#![cfg_attr(docsrs, feature(doc_cfg))]

//! Shared building blocks for the Pimalaya command-line tools.
//! Published for internal Pimalaya usage: the API follows the needs of
//! its consumers (himalaya, neverest, cardamum, calendula and friends)
//! and may change without notice. Third parties should treat it as
//! private; the io-* protocol crates and pimalaya-stream are the
//! supported surface.
//!
//! This crate factors out the pieces every Pimalaya CLI would
//! otherwise reimplement: argument parsing glue, output rendering,
//! logging and interactive prompts. Each piece sits behind its own
//! cargo feature, so a binary pulls in only what it uses.
//!
//! Unlike the io-* protocol libraries this crate is deliberately std:
//! it exists to talk to a terminal, a filesystem and a human, so it
//! never targets no_std.
//!
//! The clap module wires shared arguments and the completion, manual
//! and JSON Schema commands into a binary's parser. The printer and error
//! modules render command output, events and failures to stdout, the
//! output module holds the shapes every binary shares (bytes, pages),
//! and the log module initializes the logger.
//!
//! Text output is plain text: no border, no colour, nothing sized to the
//! terminal. The fields module lays a record out as aligned `Label: value`
//! lines, the columns module a list as aligned columns under a header,
//! and the sanitize module defuses untrusted text before either prints
//! it.
//!
//! A binary has no setup wizard: on a first run, the welcome module tells
//! a person which configuration file is missing and prints the commands
//! creating it from the sample shipped next to the binary, then the first
//! command to run. The shell module quotes such commands for a paste.
//!
//! The prompt and spinner modules handle interactive input and progress.
//! The build module exposes helpers for build scripts.

#[cfg(feature = "build")]
pub mod build;
#[cfg(feature = "terminal")]
pub mod clap;
#[cfg(feature = "terminal")]
pub mod columns;
#[cfg(feature = "terminal")]
pub mod error;
#[cfg(feature = "terminal")]
pub mod fields;
#[cfg(feature = "terminal")]
pub mod log;
#[cfg(feature = "terminal")]
pub mod output;
#[cfg(feature = "terminal")]
pub mod printer;
#[cfg(feature = "prompt")]
pub mod prompt;
#[cfg(feature = "terminal")]
pub mod sanitize;
#[cfg(feature = "terminal")]
pub mod shell;
#[cfg(feature = "spinner")]
pub mod spinner;
#[cfg(feature = "terminal")]
pub mod welcome;
