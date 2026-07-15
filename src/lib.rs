#![cfg_attr(docsrs, feature(doc_cfg))]

//! Shared building blocks for the Pimalaya command-line tools.
//!
//! This crate factors out the pieces every Pimalaya CLI would
//! otherwise reimplement: argument parsing glue, output rendering,
//! logging, interactive prompts and the account setup wizards. Each
//! piece sits behind its own cargo feature, so a binary pulls in only
//! what it uses.
//!
//! Unlike the io-* protocol libraries this crate is deliberately std:
//! it exists to talk to a terminal, a filesystem and a human, so it
//! never targets no_std.
//!
//! The clap module wires shared arguments and the completion and
//! manual commands into a binary's parser. The printer and error
//! modules render command output and failures to stdout, while the log
//! module initializes the logger. The prompt, spinner and table
//! modules handle interactive input and formatted output, and the
//! validator module holds reusable value parsers. The wizard module
//! collects account settings per protocol (IMAP, SMTP, JMAP, CalDAV,
//! CardDAV), each protocol behind its matching feature. The build
//! module exposes helpers for build scripts.

#[cfg(feature = "build")]
pub mod build;
#[cfg(feature = "terminal")]
pub mod clap;
#[cfg(feature = "terminal")]
pub mod error;
#[cfg(feature = "terminal")]
pub mod log;
#[cfg(feature = "terminal")]
pub mod printer;
#[cfg(feature = "prompt")]
pub mod prompt;
#[cfg(feature = "spinner")]
pub mod spinner;
#[cfg(feature = "table")]
pub mod table;
#[cfg(feature = "prompt")]
pub mod validator;
#[cfg(feature = "wizard")]
pub mod wizard;
