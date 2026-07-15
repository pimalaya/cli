//! Reusable clap value parsers.

use std::path::PathBuf;

/// Shell-expands then canonicalizes a path argument, falling back to
/// the expanded path when it does not yet exist.
pub fn path_parser(path: &str) -> Result<PathBuf, String> {
    match shellexpand::full(path) {
        Ok(path) => {
            let path = PathBuf::from(&*path);
            Ok(path.canonicalize().unwrap_or(path))
        }
        Err(err) => Err(err.to_string()),
    }
}
