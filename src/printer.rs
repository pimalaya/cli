//! Command output rendering to stdout.
//!
//! The [`Printer`] trait abstracts how a command emits what it prints:
//! events while it runs, then its result. The [`StdoutPrinter`]
//! implementation writes plain text or JSON to stdout.
//!
//! Every value a command prints is data: a named type whose `Display`
//! writes the text form and whose `Serialize` writes the JSON one. There
//! is no message wrapper, a sentence being the text form of data, never
//! data itself. A record's text form is [`Fields`], a list's [`Columns`].
//!
//! [`Columns`]: crate::columns::Columns
//! [`Fields`]: crate::fields::Fields

use std::{
    fmt,
    io::{IsTerminal, Stdout, Write, stdout},
};

use anyhow::{Context, Result};
use serde::Serialize;

use crate::{clap::args::JsonFlag, output::Bytes};

/// Sink a command writes its output to.
pub trait Printer {
    /// Writes the result of the command, as text or JSON.
    fn out<T: fmt::Display + Serialize>(&mut self, data: T) -> Result<()>;

    /// Writes an event, something the command observed while running,
    /// before its result.
    ///
    /// An event serializes as an object whose root carries `event`, a
    /// kebab-case name, which a result never carries.
    fn event<T: fmt::Display + Serialize>(&mut self, event: T) -> Result<()>;

    /// Writes bytes as the result of the command: verbatim in text mode,
    /// as a [`Bytes`] object under JSON.
    fn out_bytes(&mut self, bytes: Bytes) -> Result<()>;

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

    fn write_json_line<T: Serialize>(&mut self, data: &T) -> Result<()> {
        serde_json::to_writer(&mut self.stdout, data).context("Print JSON to stdout error")?;
        writeln!(self.stdout)?;
        self.stdout.flush()?;
        Ok(())
    }
}

impl Printer for StdoutPrinter {
    fn out<T: fmt::Display + Serialize>(&mut self, data: T) -> Result<()> {
        if !self.json {
            write!(self.stdout, "{data}")?;
            return Ok(());
        }

        if self.stdout.is_terminal() {
            serde_json::to_writer_pretty(&mut self.stdout, &data)
                .context("Print pretty JSON to stdout error")?;
            writeln!(self.stdout)?;
            return Ok(());
        }

        // NOTE: one value per line, flushed, so a process printing
        // several values (a REPL session, a watch) emits
        // newline-delimited JSON.
        self.write_json_line(&data)
    }

    fn event<T: fmt::Display + Serialize>(&mut self, event: T) -> Result<()> {
        // NOTE: an event is one compact line even on a terminal, so a
        // stream of them stays one value per line.
        if self.json {
            return self.write_json_line(&event);
        }

        // NOTE: text events go to a terminal only, so a redirected stdout
        // captures the result alone.
        if self.stdout.is_terminal() {
            write!(self.stdout, "{event}")?;
            self.stdout.flush()?;
        }

        Ok(())
    }

    fn out_bytes(&mut self, bytes: Bytes) -> Result<()> {
        if self.json {
            return self.out(bytes);
        }

        self.stdout
            .write_all(&bytes.data)
            .context("Write bytes to stdout error")?;
        self.stdout.flush().context("Flush stdout error")?;

        Ok(())
    }

    fn is_json(&self) -> bool {
        self.json
    }
}

/// A [`Printer`] keeping the rendered output in memory, for tests.
#[cfg(test)]
#[derive(Default)]
pub struct TestPrinter {
    /// The text form of the result.
    pub output: String,
    /// The text form of each event, in order.
    pub events: Vec<String>,
}

#[cfg(test)]
impl Printer for TestPrinter {
    fn out<T: fmt::Display + Serialize>(&mut self, data: T) -> Result<()> {
        self.output = data.to_string();
        Ok(())
    }

    fn event<T: fmt::Display + Serialize>(&mut self, event: T) -> Result<()> {
        self.events.push(event.to_string());
        Ok(())
    }

    fn out_bytes(&mut self, bytes: Bytes) -> Result<()> {
        self.output = String::from_utf8_lossy(&bytes.data).into_owned();
        Ok(())
    }
}
