//! Untrusted text made safe to print on a terminal.
//!
//! A text output prints its values as given, so a string from a message
//! or a server could drive the terminal with escape sequences (rewriting
//! the clipboard via OSC 52, hiding text, clearing the screen) or reorder
//! what is displayed with bidi overrides (`invoice\u{202e}fdp.exe` reads
//! as `invoiceexe.pdf`). [`text`] defuses such a string before it goes
//! into a field, a column or any other text form.

use std::borrow::Cow;

/// Replaces the control and bidi characters of `s` with U+FFFD.
///
/// Control characters are C0, DEL and C1; bidi characters are the
/// embeddings, overrides and isolates of Unicode UAX #9. Clean text is
/// borrowed.
pub fn text(s: &str) -> Cow<'_, str> {
    if !s.chars().any(is_unsafe) {
        return Cow::Borrowed(s);
    }

    s.chars()
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

    use crate::sanitize;

    #[test]
    fn sanitize_borrows_clean_text() {
        assert!(matches!(
            sanitize::text("Re: café ☕"),
            Cow::Borrowed("Re: café ☕")
        ));
    }

    #[test]
    fn sanitize_replaces_control_characters() {
        // ESC and BEL (C0), DEL, the single-byte CSI (C1) and tab.
        assert_eq!(
            sanitize::text("a\x1b[2Jb\x07c\x7fd\u{9b}e\tf"),
            "a�[2Jb�c�d�e�f"
        );
    }

    #[test]
    fn sanitize_replaces_bidi_characters() {
        assert_eq!(sanitize::text("invoice\u{202e}fdp.exe"), "invoice�fdp.exe");
        assert_eq!(sanitize::text("a\u{2066}b\u{2069}c"), "a�b�c");
    }
}
