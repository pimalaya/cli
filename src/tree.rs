//! Text lists printed as trees.
//!
//! A command printing items that hang from something (the envelopes of a
//! mailbox, the replies of a message) renders them as [`Trees`], the way
//! `tree` prints a directory: per item, its title on a branch, then one
//! line of detail under it, its own items hanging below. Nothing is
//! aligned after a value, so a line is never wider than its own values
//! and nothing depends on the terminal's width.

use std::fmt;

/// The branch of an item followed by a sibling.
const BRANCH: &str = "├─ ";

/// The branch of the last item of its parent.
const LAST: &str = "└─ ";

/// The rule running past an item down to the next sibling.
const RULE: &str = "│  ";

/// The indent under the last item of its parent.
const BLANK: &str = "   ";

/// Lists printed as trees, one per root.
///
/// Roots are separated by one blank line, and every line ends with a
/// newline like every text output, so an empty list prints nothing.
/// Titles and details print as given, each on one line: an untrusted one
/// is sanitized first with [`sanitize::text`].
///
/// [`sanitize::text`]: crate::sanitize::text
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Trees {
    /// The roots, in print order.
    pub items: Vec<Node>,
}

impl Trees {
    /// Appends a root.
    pub fn push(&mut self, node: Node) {
        self.items.push(node);
    }
}

impl fmt::Display for Trees {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, root) in self.items.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }

            writeln!(f, "{}", root.title)?;

            if !root.detail.is_empty() {
                // At the margin, a root's detail hangs one indent in.
                let gutter = if root.children.is_empty() {
                    BLANK
                } else {
                    RULE
                };
                writeln!(f, "{gutter}{}", root.detail)?;
            }

            write_children(f, &root.children, "")?;
        }

        Ok(())
    }
}

/// One item of [`Trees`].
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Node {
    /// The item's name, on its branch.
    pub title: String,
    /// The item's properties on one line under its title, after the rule
    /// down to its children when it has some, omitted when empty.
    pub detail: String,
    /// The items hanging from this one, in print order.
    pub children: Vec<Node>,
}

/// Writes the items hanging from a node, each line after `prefix`.
fn write_children(f: &mut fmt::Formatter<'_>, children: &[Node], prefix: &str) -> fmt::Result {
    for (i, child) in children.iter().enumerate() {
        let (branch, indent) = if i + 1 == children.len() {
            (LAST, BLANK)
        } else {
            (BRANCH, RULE)
        };

        // An untitled item leaves no trailing space after its branch.
        let branch = match child.title.is_empty() {
            true => branch.trim_end(),
            false => branch,
        };

        writeln!(f, "{prefix}{branch}{}", child.title)?;

        let prefix = format!("{prefix}{indent}");

        if !child.detail.is_empty() {
            let gutter = if child.children.is_empty() { "" } else { RULE };
            writeln!(f, "{prefix}{gutter}{}", child.detail)?;
        }

        write_children(f, &child.children, &prefix)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::tree::*;

    fn node(title: &str, detail: &str, children: Vec<Node>) -> Node {
        Node {
            title: String::from(title),
            detail: String::from(detail),
            children,
        }
    }

    #[test]
    fn items_hang_from_their_root() {
        let mut trees = Trees::default();
        trees.push(node(
            "INBOX",
            "",
            vec![
                node("Hi", "42 (unread)  Alice <a@example.org>", Vec::new()),
                node("Re: Hi", "41  Carol <c@example.org>", Vec::new()),
            ],
        ));

        let expected = "\
INBOX
├─ Hi
│  42 (unread)  Alice <a@example.org>
└─ Re: Hi
   41  Carol <c@example.org>
";
        assert_eq!(trees.to_string(), expected);
    }

    #[test]
    fn replies_nest_under_their_parent_and_roots_stand_apart() {
        let carol = node("Re: Lunch", "7  Carol", Vec::new());
        let alice = node("Re: Lunch", "2  Alice", vec![carol]);
        let bob = node("Re: Lunch", "3  Bob", Vec::new());

        let mut trees = Trees::default();
        trees.push(node("Lunch", "1  Me", vec![alice, bob]));
        trees.push(node("Invoice", "4  Bob", Vec::new()));
        trees.push(node("Notes", "", vec![node("", "5  Me", Vec::new())]));

        let expected = "\
Lunch
│  1  Me
├─ Re: Lunch
│  │  2  Alice
│  └─ Re: Lunch
│     7  Carol
└─ Re: Lunch
   3  Bob

Invoice
   4  Bob

Notes
└─
   5  Me
";
        assert_eq!(trees.to_string(), expected);
    }

    #[test]
    fn an_empty_list_prints_nothing() {
        assert_eq!(Trees::default().to_string(), "");
    }
}
