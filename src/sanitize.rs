//! Untrusted text made safe to print on a terminal or to name a file.
//!
//! A text output prints its values as given, so a string from a message
//! or a server could drive the terminal with escape sequences (rewriting
//! the clipboard via OSC 52, hiding text, clearing the screen) or reorder
//! what is displayed with bidi overrides (`invoice\u{202e}fdp.exe` reads
//! as `invoiceexe.pdf`). [`text`] defuses such a string before it goes
//! into a field, a column or any other text form, [`lines`] one keeping
//! its lines, and [`path_component`] a name before it names a file.

use std::borrow::Cow;

/// Replaces the control and bidi characters of `s` with U+FFFD.
///
/// Control characters are C0, DEL and C1; bidi characters are the
/// embeddings, overrides and isolates of Unicode UAX #9. Clean text is
/// borrowed.
pub fn text(s: &str) -> Cow<'_, str> {
    replace(s, is_unsafe)
}

/// Replaces the control and bidi characters of `s` with U+FFFD as
/// [`text`] does, keeping the line feed and the tab.
///
/// For a multi-line text, such as an error message quoting a file. Clean
/// text is borrowed.
pub fn lines(s: &str) -> Cow<'_, str> {
    replace(s, |c| !matches!(c, '\n' | '\t') && is_unsafe(c))
}

/// The bidirectional formatting characters pimdir STORAGE §18 names.
const PATH_BIDI: [char; 12] = [
    '\u{061C}', '\u{200E}', '\u{200F}', '\u{202A}', '\u{202B}', '\u{202C}', '\u{202D}', '\u{202E}',
    '\u{2066}', '\u{2067}', '\u{2068}', '\u{2069}',
];

/// `name` as one path component, by the rule of pimdir STORAGE §18 that
/// io-pimdir's `security::path_component` applies: every path separator
/// (`/` and `\`), NUL, control character and bidirectional formatting
/// character replaced by `_`, and a name that would be empty, `.` or `..`
/// written as underscores.
///
/// The answer names no other directory, so a path joined from a directory
/// and it stays in that directory, and no bidi override disguises its
/// extension. For a name a server or a message gave, such as an
/// attachment's file name, in a binary built without io-pimdir.
pub fn path_component(name: &str) -> String {
    let mut out: String = name
        .chars()
        .map(|c| match c {
            '/' | '\\' => '_',
            c if c.is_control() || PATH_BIDI.contains(&c) => '_',
            c => c,
        })
        .collect();

    match out.as_str() {
        "" => out.push('_'),
        "." | ".." => out = "_".repeat(out.len()),
        _ => (),
    }

    out
}

fn replace(s: &str, is_unsafe: impl Fn(char) -> bool) -> Cow<'_, str> {
    if !s.chars().any(&is_unsafe) {
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
    fn sanitize_lines_keeps_line_feeds_and_tabs() {
        assert_eq!(
            sanitize::lines("a\n\tb\r\x1b[2J\u{202e}c"),
            "a\n\tb\u{fffd}\u{fffd}[2J\u{fffd}c"
        );
    }

    #[test]
    fn sanitize_replaces_bidi_characters() {
        assert_eq!(sanitize::text("invoice\u{202e}fdp.exe"), "invoice�fdp.exe");
        assert_eq!(sanitize::text("a\u{2066}b\u{2069}c"), "a�b�c");
    }

    /// The cases of io-pimdir's `security::path_component`, one answer for
    /// each, so a binary sharing this rule names a file as the store does.
    #[test]
    fn a_path_component_stays_one_component() {
        let cases = [
            ("report.pdf", "report.pdf"),
            ("../../etc/passwd", ".._.._etc_passwd"),
            ("..", "__"),
            (".", "_"),
            ("", "_"),
            ("...", "..."),
            ("a\\b", "a_b"),
            ("nul\0byte", "nul_byte"),
            ("tab\there\u{7f}\u{85}", "tab_here__"),
            ("invoice\u{202E}fdp.exe", "invoice_fdp.exe"),
            ("\u{2066}x\u{2069}\u{200E}\u{061C}", "_x___"),
            ("Résumé 2026", "Résumé 2026"),
        ];

        for (name, component) in cases {
            assert_eq!(sanitize::path_component(name), component, "{name:?}");
        }
    }
}
