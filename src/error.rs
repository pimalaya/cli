//! Command failure reporting to stdout.

use std::{
    backtrace::{Backtrace, BacktraceStatus},
    fmt, process,
};

use anyhow::{Error, Result};
use log::{Level, log_enabled};
use serde::{Serialize, Serializer, ser::SerializeStruct};

use crate::printer::Printer;

/// Wraps an [`anyhow::Error`] to render it, with its source chain and
/// backtrace, as text or JSON.
pub struct ErrorReport(Error);

impl ErrorReport {
    /// Unwraps a result, or prints the error to the printer and exits
    /// with status 1.
    pub fn eval<T>(printer: &mut impl Printer, result: Result<T, Error>) -> T {
        match result {
            Ok(res) => res,
            Err(err) => {
                printer.out(ErrorReport::from(err)).unwrap();
                process::exit(1);
            }
        }
    }

    fn sources(&self) -> impl Iterator<Item = String> + '_ {
        self.0.chain().skip(1).map(ToString::to_string)
    }

    fn suggestions(&self) -> Option<&str> {
        if !log_enabled!(Level::Debug) || !log_enabled!(Level::Trace) {
            Some("Run with --log-level to enable more verbose logs")
        } else {
            None
        }
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        let backtrace = self.0.backtrace();

        if let BacktraceStatus::Captured = backtrace.status() {
            Some(backtrace)
        } else {
            None
        }
    }
}

impl From<Error> for ErrorReport {
    fn from(err: Error) -> Self {
        Self(err)
    }
}

impl fmt::Display for ErrorReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {}", self.0)?;

        let mut header_printed = false;
        for err in self.sources() {
            if !header_printed {
                writeln!(f)?;
                writeln!(f)?;
                write!(f, "Caused by:")?;
                header_printed = true;
            }

            writeln!(f)?;
            write!(f, " - {}", err.trim())?;
        }

        if let Some(backtrace) = self.backtrace() {
            writeln!(f)?;
            writeln!(f)?;
            writeln!(f, "Backtrace:")?;
            write!(f, "{backtrace}")?;
        }

        if let Some(suggestion) = self.suggestions() {
            writeln!(f)?;
            writeln!(f)?;
            writeln!(f, "Suggestions:")?;
            write!(f, " - {suggestion}")?;
        }

        writeln!(f)
    }
}

impl Serialize for ErrorReport {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let sources: Vec<_> = self.0.chain().skip(1).map(ToString::to_string).collect();
        let backtrace = self.backtrace().map(ToString::to_string);

        let mut s = serializer.serialize_struct("ErrorReport", 3)?;
        s.serialize_field("error", &self.0.to_string())?;
        s.serialize_field("sources", &sources)?;
        s.serialize_field("backtrace", &backtrace)?;
        s.end()
    }
}
