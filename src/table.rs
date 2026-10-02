//! Re-export of the `comfy-table` tabular output crate.
//!
//! Consumers of `pimalaya-cli` enable the `table` feature and reach
//! the underlying types via [`crate::table`] instead of pulling
//! `comfy-table` in directly.

use std::borrow::Cow;

pub use comfy_table::*;

/// Replaces the control and bidi characters of `text` with U+FFFD.
///
/// A cell prints its content as given, so a string from a message or a
/// server could drive the terminal with escape sequences (rewriting the
/// clipboard via OSC 52, hiding text, clearing the screen) or reorder
/// what is displayed with bidi overrides (`invoice\u{202e}fdp.exe` reads
/// as `invoiceexe.pdf`). Control characters are C0, DEL and C1; bidi
/// characters are the embeddings, overrides and isolates of Unicode
/// UAX #9. Clean text is borrowed.
pub fn sanitize(text: &str) -> Cow<'_, str> {
    if !text.chars().any(is_unsafe) {
        return Cow::Borrowed(text);
    }

    text.chars()
        .map(|c| {
            if is_unsafe(c) {
                char::REPLACEMENT_CHARACTER
            } else {
                c
            }
        })
        .collect()
}

fn is_unsafe(c: char) -> bool {
    c.is_control() || matches!(c, '\u{202a}'..='\u{202e}' | '\u{2066}'..='\u{2069}')
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::sanitize;

    #[test]
    fn sanitize_borrows_clean_text() {
        assert!(matches!(
            sanitize("Re: café ☕"),
            Cow::Borrowed("Re: café ☕")
        ));
    }

    #[test]
    fn sanitize_replaces_control_characters() {
        // ESC and BEL (C0), DEL, the single-byte CSI (C1) and tab.
        assert_eq!(sanitize("a\x1b[2Jb\x07c\x7fd\u{9b}e\tf"), "a�[2Jb�c�d�e�f");
    }

    #[test]
    fn sanitize_replaces_bidi_characters() {
        assert_eq!(sanitize("invoice\u{202e}fdp.exe"), "invoice�fdp.exe");
        assert_eq!(sanitize("a\u{2066}b\u{2069}c"), "a�b�c");
    }
}
