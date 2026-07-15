//! Command output rendering to stdout.
//!
//! The [`Printer`] trait abstracts how a command emits its result, the
//! [`StdoutPrinter`] implementation writes plain text or JSON to
//! stdout, and [`PrintTable`] renders a value as a table.

use std::{
    fmt,
    io::{IsTerminal, Stdout, Write, stdout},
};

use anyhow::{Context, Result};
use serde::Serialize;

use crate::clap::args::JsonFlag;

/// A value that can render itself as a table.
pub trait PrintTable {
    /// Writes the table to the given writer, capped at an optional
    /// maximum width.
    fn print(&self, writer: &mut dyn Write, table_max_width: Option<u16>) -> Result<()>;
}

/// Sink a command writes its output to.
pub trait Printer {
    /// Writes one piece of command output, as text or JSON.
    fn out<T: fmt::Display + Serialize>(&mut self, data: T) -> Result<()>;

    /// Whether the printer emits JSON.
    fn is_json(&self) -> bool {
        false
    }
}

/// A [`Printer`] writing plain text or JSON to stdout.
pub struct StdoutPrinter {
    stdout: Stdout,
    json: bool,
}

impl StdoutPrinter {
    /// Builds a stdout printer, emitting JSON when the flag is set.
    pub fn new(json: &JsonFlag) -> Self {
        Self {
            stdout: stdout(),
            json: json.enabled,
        }
    }
}

impl Printer for StdoutPrinter {
    fn out<T: fmt::Display + serde::Serialize>(&mut self, data: T) -> Result<()> {
        if self.json {
            if self.stdout.is_terminal() {
                serde_json::to_writer_pretty(&mut self.stdout, &data)
                    .context("Print pretty JSON to stdout error")?;
                writeln!(self.stdout)?;
            } else {
                serde_json::to_writer(&mut self.stdout, &data)
                    .context("Print JSON to stdout error")?;
            }
        } else {
            write!(self.stdout, "{data}")?;
        }

        Ok(())
    }

    fn is_json(&self) -> bool {
        self.json
    }
}

/// A plain message wrapper providing text and JSON output.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Message {
    message: String,
}

impl Message {
    /// Wraps the given message.
    pub fn new(message: impl ToString) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", &self.message)
    }
}
