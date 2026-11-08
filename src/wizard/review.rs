//! # Review
//!
//! The last step of an account wizard: one row per picked domain (mail,
//! contacts, calendars) printed on stderr, then a pick list to continue
//! or to change one row. The caller builds the options behind a change.

use core::fmt;

use crate::prompt::{self, PromptResult};

/// One reviewed domain: its name and what was picked for it.
#[derive(Clone, Debug)]
pub struct ReviewRow {
    /// The domain name, as `Mail` or `Contacts`.
    pub name: String,
    /// What was picked, as `IMAP at imap.example.org, with your password`.
    pub line: String,
}

/// The answer to the review.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReviewAction {
    /// Keep the picks.
    Continue,
    /// Change the row at this index.
    Change(usize),
}

/// Prints `rows` on stderr, names padded to the widest, then asks whether
/// to continue or which row to change.
pub fn prompt(rows: &[ReviewRow]) -> PromptResult<ReviewAction> {
    let width = rows
        .iter()
        .map(|row| row.name.chars().count())
        .max()
        .unwrap_or_default();

    for row in rows {
        eprintln!("{:<width$}  {}", row.name, row.line);
    }

    let mut items = vec![Item {
        action: ReviewAction::Continue,
        label: String::from("Continue"),
    }];

    items.extend(rows.iter().enumerate().map(|(index, row)| Item {
        action: ReviewAction::Change(index),
        label: format!("Change {}", row.name.to_lowercase()),
    }));

    let item = prompt::item("What next?", items, None)?;

    Ok(item.action)
}

/// One entry in the review pick list.
#[derive(Eq, PartialEq)]
struct Item {
    action: ReviewAction,
    label: String,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.label)
    }
}
