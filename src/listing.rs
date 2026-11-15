//! # Listing
//!
//! The text frame of every list: the fields placing it (the account, what
//! is listed, the page it holds) as [`Fields`], one blank line, the items
//! as [`Columns`] with no header row, then the `Next page:` line reaching
//! the next page and the item errors as [`ItemErrors`] prints them, each
//! one blank line apart.
//!
//! [`Fields`]: crate::fields::Fields
//! [`ItemErrors`]: crate::error::ItemErrors

use std::fmt;

use crate::{columns::Columns, error::ItemErrors, fields::Fields, sanitize};

/// A list printed one item per line under the fields placing it.
#[derive(Clone, Debug, Default)]
pub struct Listing<'a> {
    /// The fields above the items, their values sanitized.
    pub header: Vec<(&'static str, String)>,
    /// The items, one per line, with no header row.
    pub items: Columns<'a>,
    /// The line printed in place of the items when there are none.
    pub empty: Option<&'static str>,
    /// The flag reaching the next page, sanitized, printed after the
    /// items as `Next page: <flag> <value>`.
    pub next_page: Option<String>,
    /// The items the command could not list, printed last.
    pub errors: ItemErrors,
}

/// The page a listing holds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ListingPage {
    /// The whole listing, unpaged.
    All,
    /// A page of a listing paged by number, reached by `--page`.
    Numbered {
        /// The page held, from 1.
        page: u64,
        /// The number of items a full page holds.
        size: u64,
    },
    /// A page reached by an opaque token, reached by `--page-token`.
    Token,
}

impl<'a> Listing<'a> {
    /// A listing headed by the account, its backend in parentheses.
    pub fn new(account: &str, backend: Option<&str>) -> Self {
        let mut value = sanitize::text(account).into_owned();

        if let Some(backend) = backend {
            value.push_str(&format!(" ({backend})"));
        }

        Self {
            header: vec![("Account", value)],
            ..Default::default()
        }
    }

    /// Adds a field above the items, its value sanitized.
    pub fn head(&mut self, label: &'static str, value: &str) {
        let value = sanitize::text(value).into_owned();
        self.header.push((label, value));
    }

    /// Adds the page the `count` items fill, named by their singular and
    /// plural `nouns`, and sets the flag reaching the `next` page.
    pub fn page(&mut self, page: ListingPage, count: usize, nouns: [&str; 2], next: Option<&str>) {
        let count = count as u64;
        let [one, many] = nouns;

        let (number, first) = match page {
            ListingPage::All => (Some(String::from("all")), Some(1)),
            ListingPage::Numbered { page, size } => {
                let first = page.saturating_sub(1) * size + 1;
                (Some(page.to_string()), Some(first))
            }
            ListingPage::Token => (None, None),
        };

        let held = match (count, first) {
            (0, _) => String::from("empty"),
            (1, Some(first)) => format!("{one} {first}"),
            (n, Some(first)) => format!("{many} {first} to {}", first + n - 1),
            (1, None) => format!("1 {one}"),
            (n, None) => format!("{n} {many}"),
        };

        let last = match (page, next) {
            (ListingPage::All, _) | (_, Some(_)) => "",
            _ if count == 0 => "",
            _ => ", the last",
        };

        let value = match number {
            Some(number) => format!("{number}, {held}{last}"),
            None => format!("{held}{last}"),
        };

        self.header.push(("Page", value));

        if let Some(token) = next {
            let flag = match page {
                ListingPage::All | ListingPage::Numbered { .. } => "--page",
                ListingPage::Token => "--page-token",
            };

            let token = sanitize::text(token);
            self.next_page = Some(format!("{flag} {token}"));
        }
    }
}

impl fmt::Display for Listing<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut header = Fields::default();

        for (label, value) in &self.header {
            header.push(label, value);
        }

        let mut printed = header.to_string();

        let body = match (self.items.rows.is_empty(), self.empty) {
            (false, _) => Some(self.items.to_string()),
            (true, Some(empty)) => Some(format!("{empty}\n")),
            (true, None) => None,
        };

        let next = self.next_page.as_ref().map(|n| format!("Next page: {n}\n"));

        for block in [body, next].into_iter().flatten() {
            if !printed.is_empty() {
                printed.push('\n');
            }

            printed.push_str(&block);
        }

        f.write_str(&printed)?;

        if printed.is_empty() {
            return write!(f, "{}", self.errors);
        }

        self.errors.write_after(f)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::anyhow;
    use schemars::JsonSchema;
    use serde::Serialize;

    use crate::{
        error::{CodedError, ItemError},
        listing::*,
    };

    #[derive(Clone, Debug, Serialize, JsonSchema)]
    #[serde(rename_all = "kebab-case")]
    enum TestCode {
        MessageUnreadable,
    }

    fn listing(rows: usize) -> Listing<'static> {
        let mut listing = Listing::new("work", Some("imap"));

        for i in 0..rows {
            listing.items.push([i.to_string(), String::from("x")]);
        }

        listing
    }

    #[test]
    fn a_numbered_page_counts_its_items_and_names_the_next_after_them() {
        let mut first = listing(2);
        first.head("Mailbox", "INBOX");
        first.page(
            ListingPage::Numbered { page: 2, size: 2 },
            2,
            ["envelope", "envelopes"],
            Some("3"),
        );

        let expected = "\
Account:  work (imap)
Mailbox:  INBOX
Page:     2, envelopes 3 to 4

0  x
1  x

Next page: --page 3
";
        assert_eq!(first.to_string(), expected);

        let mut last = listing(1);
        last.page(
            ListingPage::Numbered { page: 3, size: 2 },
            1,
            ["envelope", "envelopes"],
            None,
        );

        let expected = "\
Account:  work (imap)
Page:     3, envelope 5, the last

0  x
";
        assert_eq!(last.to_string(), expected);
    }

    #[test]
    fn a_token_page_counts_what_it_holds() {
        let mut first = listing(2);
        first.page(ListingPage::Token, 2, ["draft", "drafts"], Some("abc"));

        let expected = "\
Account:  work (imap)
Page:     2 drafts

0  x
1  x

Next page: --page-token abc
";
        assert_eq!(first.to_string(), expected);

        let mut empty = listing(0);
        empty.page(ListingPage::Token, 0, ["draft", "drafts"], None);
        assert_eq!(
            empty.to_string(),
            "Account:  work (imap)\nPage:     empty\n"
        );
    }

    #[test]
    fn an_empty_page_still_names_the_next() {
        let mut empty = listing(0);
        empty.empty = Some("No draft");
        empty.page(ListingPage::Token, 0, ["draft", "drafts"], Some("abc"));

        let expected = "\
Account:  work (imap)
Page:     empty

No draft

Next page: --page-token abc
";
        assert_eq!(empty.to_string(), expected);
    }

    #[test]
    fn header_values_align_past_the_longest_label() {
        let mut listing = listing(0);
        listing.head("Addressbook", "contacts");
        listing.empty = Some("No card");

        let expected = "\
Account:      work (imap)
Addressbook:  contacts

No card
";
        assert_eq!(listing.to_string(), expected);
    }

    #[test]
    fn item_errors_print_last_after_the_next_page() {
        let mut listing = listing(1);
        listing.page(
            ListingPage::Numbered { page: 1, size: 1 },
            1,
            ["envelope", "envelopes"],
            Some("2"),
        );

        let unreadable = CodedError::new(TestCode::MessageUnreadable, "Cannot decode it");
        listing
            .errors
            .push(&ItemError::<TestCode>::new(None, &unreadable.into()));
        listing
            .errors
            .push(&ItemError::<TestCode>::new(Some("42"), &anyhow!("Gone")));

        let expected = "\
Account:  work (imap)
Page:     1, envelope 1

0  x

Next page: --page 2

Error:  -   message-unreadable  Cannot decode it
Error:  42  -                   Gone
";
        assert_eq!(listing.to_string(), expected);
    }

    #[test]
    fn a_bare_listing_prints_its_items_alone() {
        let mut bare = Listing::default();
        bare.items.push(["a", "b"]);
        assert_eq!(bare.to_string(), "a  b\n");

        let mut errors_only = Listing::default();
        errors_only
            .errors
            .push(&ItemError::<TestCode>::new(None, &anyhow!("Gone")));
        assert_eq!(errors_only.to_string(), "Error:  -  -  Gone\n");
    }
}
