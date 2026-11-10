//! Text records printed as aligned fields.
//!
//! A command printing one record as text, rather than a table of many,
//! renders it as [`Fields`]: one `Label: value` line per field, the
//! values aligned in one column the way clap aligns its help. Each line
//! is greppable by its label, and the record ends with a newline like
//! every text output.

use std::fmt;

/// A record printed as aligned `Label: value` lines.
///
/// Values start two spaces past the colon of the longest label. A value
/// spanning several lines continues under its first line, and an empty
/// one leaves its label alone. Values print as given: an untrusted one
/// is sanitized first, as a table cell is.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Fields<'a> {
    /// The labels and their rendered values, in print order.
    pub rows: Vec<(&'a str, String)>,
}

impl<'a> Fields<'a> {
    /// Appends a field, rendering its value.
    pub fn push(&mut self, label: &'a str, value: impl fmt::Display) {
        self.rows.push((label, value.to_string()));
    }
}

impl fmt::Display for Fields<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = self.rows.iter().map(|(label, _)| label.chars().count());
        let width = width.max().unwrap_or_default();
        let indent = width + 3;

        for (label, value) in &self.rows {
            let mut lines = value.lines();

            match lines.next().filter(|line| !line.is_empty()) {
                Some(first) => {
                    let pad = width - label.chars().count() + 2;
                    writeln!(f, "{label}:{:pad$}{first}", "")?;
                }
                None => writeln!(f, "{label}:")?,
            }

            for line in lines {
                match line {
                    "" => writeln!(f)?,
                    line => writeln!(f, "{:indent$}{line}", "")?,
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::fields::*;

    #[test]
    fn values_align_past_the_longest_label() {
        let mut fields = Fields::default();
        fields.push("Type", "Bearer");
        fields.push("Refresh token", "yes");
        fields.push("Scopes", "mail\ncontacts");
        fields.push("Note", "");

        let expected = "\
Type:           Bearer
Refresh token:  yes
Scopes:         mail
                contacts
Note:
";
        assert_eq!(fields.to_string(), expected);
    }

    #[test]
    fn an_empty_record_prints_nothing() {
        assert_eq!(Fields::default().to_string(), "");
    }
}
