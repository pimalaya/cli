//! Reusable clap value parsers.

use std::{env, path::PathBuf};

/// Shell-expands then canonicalizes a path argument, falling back to
/// the expanded path when it does not yet exist.
pub fn path_parser(path: &str) -> Result<PathBuf, String> {
    // NOTE: `shellexpand::full` needs `tilde`, a feature pinning dirs below 7.
    let home_dir = || dirs::home_dir()?.into_os_string().into_string().ok();
    let context = |name: &str| env::var(name).map(Some);

    match shellexpand::full_with_context(path, home_dir, context) {
        Ok(path) => {
            let path = PathBuf::from(&*path);
            Ok(path.canonicalize().unwrap_or(path))
        }
        Err(err) => Err(err.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use crate::clap::parsers::path_parser;

    #[cfg(unix)]
    #[test]
    fn tilde_and_home_variable_expand_to_the_home_directory() {
        let home = std::env::var_os("HOME").unwrap();
        let expected = std::path::Path::new(&home).join("pimalaya-cli-missing");

        for path in [
            "~/pimalaya-cli-missing",
            "$HOME/pimalaya-cli-missing",
            "${HOME}/pimalaya-cli-missing",
        ] {
            assert_eq!(path_parser(path).unwrap(), expected, "{path}");
        }
    }

    #[test]
    fn an_unset_variable_is_rejected() {
        let err = path_parser("$PIMALAYA_CLI_UNSET/file").unwrap_err();

        assert_eq!(
            err,
            "error looking key 'PIMALAYA_CLI_UNSET' up: environment variable not found"
        );
    }
}
