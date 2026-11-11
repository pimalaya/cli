//! Text lists printed as aligned columns.
//!
//! A command printing a list as text renders it as [`Columns`]: a header
//! row of labels, then one row per item, the cells aligned in columns
//! two spaces apart, with no border and no colour. Each line is one item,
//! greppable and split on blanks, and the list ends with a newline like
//! every text output. [`Fields`] is its twin for a single record.
//!
//! [`Fields`]: crate::fields::Fields

use std::{borrow::Cow, fmt, iter};

use unicode_width::UnicodeWidthStr;

/// A list printed as borderless aligned columns.
///
/// Cells are padded to the widest cell of their column, measured in
/// terminal display columns, and separated by two spaces; the last column
/// is not padded. An empty or missing cell prints as `-`, and a line break
/// inside a cell as a space. Nothing is truncated: the terminal folds a
/// long line. Labels and cells print as given: the caller passes the
/// labels uppercase, and sanitizes an untrusted cell first with
/// [`sanitize::text`].
///
/// [`sanitize::text`]: crate::sanitize::text
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Columns<'a> {
    /// The column labels, printed as the header row unless empty.
    pub header: Vec<&'a str>,
    /// The rendered cells of each row, in print order.
    pub rows: Vec<Vec<String>>,
}

impl<'a> Columns<'a> {
    /// Builds a list of the given column labels, without rows.
    pub fn new(header: impl IntoIterator<Item = &'a str>) -> Self {
        Self {
            header: header.into_iter().collect(),
            rows: Vec::new(),
        }
    }

    /// Appends a row, rendering its cells.
    pub fn push(&mut self, row: impl IntoIterator<Item = impl fmt::Display>) {
        let row = row.into_iter().map(|cell| cell.to_string());
        self.rows.push(row.collect());
    }
}

impl fmt::Display for Columns<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let header = (!self.header.is_empty()).then(|| {
            let labels = self.header.iter().copied();
            labels.map(cell).collect::<Vec<_>>()
        });
        let rows = self.rows.iter().map(|row| {
            let cells = row.iter().map(String::as_str);
            cells.map(cell).collect::<Vec<_>>()
        });
        let lines: Vec<_> = iter::once(header).flatten().chain(rows).collect();

        let count = lines.iter().map(Vec::len).max().unwrap_or_default();
        let mut widths = vec![1; count];

        for line in &lines {
            for (width, cell) in widths.iter_mut().zip(line) {
                *width = cell.width().max(*width);
            }
        }

        for line in &lines {
            for (i, width) in widths.iter().enumerate() {
                let cell = line.get(i).map_or("-", |cell| cell);

                if i > 0 {
                    f.write_str("  ")?;
                }

                f.write_str(cell)?;

                if i + 1 < count {
                    let pad = width - cell.width();
                    write!(f, "{:pad$}", "")?;
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

/// Renders a cell on one line, `-` when empty.
fn cell(s: &str) -> Cow<'_, str> {
    if s.is_empty() {
        Cow::Borrowed("-")
    } else if s.contains(['\n', '\r']) {
        Cow::Owned(s.replace(['\n', '\r'], " "))
    } else {
        Cow::Borrowed(s)
    }
}

#[cfg(test)]
mod tests {
    use crate::columns::*;

    #[test]
    fn a_wide_last_column_is_not_padded() {
        let mut columns = Columns::new(["ID", "NAME", "DESCRIPTION"]);
        columns.push(["1", "Inbox", "Mail received"]);
        columns.push(["42", "Sent", ""]);

        let expected = "\
ID  NAME   DESCRIPTION
1   Inbox  Mail received
42  Sent   -
";
        assert_eq!(columns.to_string(), expected);
    }

    #[test]
    fn wide_characters_count_their_display_width() {
        let mut columns = Columns::new(["NAME", "ID"]);
        columns.push(["日本語", "1"]);
        columns.push(["🦀", "2"]);
        columns.push(["ab", "3"]);

        let expected = "\
NAME    ID
日本語  1
🦀      2
ab      3
";
        assert_eq!(columns.to_string(), expected);
    }

    #[test]
    fn empty_and_missing_cells_print_a_dash() {
        let mut columns = Columns::new(["A", "B", "C"]);
        columns.push(["", "b"]);
        columns.push(["a", "", "c"]);

        let expected = "\
A  B  C
-  b  -
a  -  c
";
        assert_eq!(columns.to_string(), expected);
    }

    #[test]
    fn a_line_break_in_a_cell_prints_as_a_space() {
        let mut columns = Columns::new(["SUBJECT", "FROM"]);
        columns.push(["Re:\nhello", "a\r\nb"]);

        let expected = "\
SUBJECT    FROM
Re: hello  a  b
";
        assert_eq!(columns.to_string(), expected);
    }

    #[test]
    fn a_list_without_rows_prints_its_header() {
        let columns = Columns::new(["ID", "NAME"]);
        assert_eq!(columns.to_string(), "ID  NAME\n");
        assert_eq!(Columns::default().to_string(), "");
    }
}
