//! Commands printed for a person to paste into a shell.
//!
//! A binary sometimes answers with the next command to run: the first
//! one after the welcome, the one resuming a flow. [`invocation`] starts
//! such a command with the binary and the flags it ran with, and
//! [`quote`] makes each value survive the paste.

use std::path::PathBuf;

/// Quotes `s` for a paste into a command line: single quotes for a POSIX
/// shell, double quotes on Windows, which cmd and PowerShell both read,
/// a Windows path never holding one.
pub fn quote(s: &str) -> String {
    if cfg!(windows) {
        return format!("\"{s}\"");
    }

    let mut out = String::with_capacity(s.len() + 2);
    out.push('\'');
    for ch in s.chars() {
        if ch == '\'' {
            out.push_str("'\\''");
        } else {
            out.push(ch);
        }
    }
    out.push('\'');
    out
}

/// Builds the invocation a printed command starts with: `bin`, then one
/// `-c` per configuration path and `-a` when an account is named, each
/// value quoted.
///
/// It carries the `-c` and `-a` the binary ran with, so the command
/// reaches the same configuration and account rather than the default
/// ones.
pub fn invocation(bin: &str, config_paths: &[PathBuf], account: Option<&str>) -> String {
    let mut invocation = String::from(bin);

    for path in config_paths {
        invocation.push_str(" -c ");
        invocation.push_str(&quote(&path.to_string_lossy()));
    }

    if let Some(name) = account.filter(|name| !name.is_empty()) {
        invocation.push_str(" -a ");
        invocation.push_str(&quote(name));
    }

    invocation
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::shell;

    #[test]
    #[cfg(not(windows))]
    fn quote_escapes_embedded_quotes() {
        assert_eq!(shell::quote("plain"), "'plain'");
        assert_eq!(shell::quote("a;b"), "'a;b'");
        assert_eq!(shell::quote("it's"), "'it'\\''s'");
    }

    #[test]
    #[cfg(not(windows))]
    fn invocation_carries_the_account_and_config() {
        assert_eq!(shell::invocation("app", &[], None), "app");
        assert_eq!(shell::invocation("app", &[], Some("")), "app");
        assert_eq!(shell::invocation("app", &[], Some("mail")), "app -a 'mail'");

        let paths = [PathBuf::from("/a.toml"), PathBuf::from("/b c.toml")];
        assert_eq!(
            shell::invocation("app", &paths, Some("work's")),
            "app -c '/a.toml' -c '/b c.toml' -a 'work'\\''s'"
        );
    }
}
