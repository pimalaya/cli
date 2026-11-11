//! Text lists printed as records.
//!
//! A command printing a list whose items each carry more than one value
//! of unbounded length (an address, a subject, a path) renders it as
//! [`Records`], the way `git log` prints commits: per item, a title line,
//! its [`Fields`], then its free text as an indented body, one blank line
//! between items. A line is never wider than its own value, so nothing
//! depends on the terminal's width. [`Columns`] stays the list of short,
//! bounded rows.
//!
//! [`Columns`]: crate::columns::Columns

use std::fmt;

use crate::fields::Fields;

/// A list printed as records, one block per item.
///
/// Items are separated by one blank line, and the list ends with a
/// newline like every text output. Titles, state words, fields and
/// bodies print as given: an untrusted one is sanitized first with
/// [`sanitize::text`], a body line by line.
///
/// [`sanitize::text`]: crate::sanitize::text
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Records<'a> {
    /// The items, in print order.
    pub items: Vec<Record<'a>>,
}

impl<'a> Records<'a> {
    /// Appends an item.
    pub fn push(&mut self, record: Record<'a>) {
        self.items.push(record);
    }
}

impl fmt::Display for Records<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, record) in self.items.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }

            write!(f, "{record}")?;
        }

        Ok(())
    }
}

/// One item of [`Records`].
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Record<'a> {
    /// The item's identity, alone on its first line.
    pub title: String,
    /// Words naming the item's state, printed after the title in
    /// parentheses and comma-separated, omitted when empty.
    pub state: Vec<String>,
    /// The item's properties, under its title.
    pub fields: Fields<'a>,
    /// The item's free text, after a blank line, each line indented by
    /// four spaces, omitted when empty.
    pub body: String,
}

impl fmt::Display for Record<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.title)?;

        if !self.state.is_empty() {
            write!(f, " ({})", self.state.join(", "))?;
        }

        writeln!(f)?;
        write!(f, "{}", self.fields)?;

        if self.body.is_empty() {
            return Ok(());
        }

        writeln!(f)?;

        for line in self.body.lines() {
            match line {
                "" => writeln!(f)?,
                line => writeln!(f, "    {line}")?,
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{fields::Fields, records::*};

    fn envelope(id: &str, state: &[&str], from: &str, subject: &str) -> Record<'static> {
        let mut fields = Fields::default();
        fields.push("From", from);
        fields.push("Date", "2026-11-11 10:00+01:00");

        Record {
            title: String::from(id),
            state: state.iter().map(|word| word.to_string()).collect(),
            fields,
            body: String::from(subject),
        }
    }

    #[test]
    fn items_print_like_git_log() {
        let mut records = Records::default();
        let state = ["unread", "flagged"];
        records.push(envelope("42", &state, "Alice <a@example.org>", "Hi"));
        records.push(envelope("41", &[], "Carol <c@example.org>", "Re: Hi"));

        let expected = "\
42 (unread, flagged)
From:  Alice <a@example.org>
Date:  2026-11-11 10:00+01:00

    Hi

41
From:  Carol <c@example.org>
Date:  2026-11-11 10:00+01:00

    Re: Hi
";
        assert_eq!(records.to_string(), expected);
    }

    #[test]
    fn a_body_keeps_its_lines_and_an_empty_one_is_omitted() {
        let mut records = Records::default();
        let mut snippet = envelope("1", &[], "a@example.org", "first\n\nthird");
        snippet.fields = Fields::default();
        records.push(snippet);
        records.push(envelope("2", &[], "b@example.org", ""));

        let expected = "\
1

    first

    third

2
From:  b@example.org
Date:  2026-11-11 10:00+01:00
";
        assert_eq!(records.to_string(), expected);
    }

    #[test]
    fn an_empty_list_prints_nothing() {
        assert_eq!(Records::default().to_string(), "");
    }
}
