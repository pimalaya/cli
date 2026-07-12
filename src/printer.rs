use std::{
    fmt,
    io::{IsTerminal, Stdout, Write, stdout},
};

use anyhow::{Context, Result};
use serde::Serialize;

use crate::clap::args::JsonFlag;

pub trait PrintTable {
    fn print(&self, writer: &mut dyn Write, table_max_width: Option<u16>) -> Result<()>;
}

pub trait Printer {
    fn out<T: fmt::Display + Serialize>(&mut self, data: T) -> Result<()>;

    fn is_json(&self) -> bool {
        false
    }
}

pub struct StdoutPrinter {
    stdout: Stdout,
    json: bool,
}

impl StdoutPrinter {
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
                writeln!(self.stdout, "")?;
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

/// Defines a struct-wrapper to provide a JSON output.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Message {
    message: String,
}

impl Message {
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
