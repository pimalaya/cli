//! The welcome a person meets on a first run.
//!
//! A binary finding no configuration file answers a person with a
//! [`Welcome`]: the missing file, the commands creating it from the
//! sample configuration, then the first command to run. The sample is
//! the one a release or a package ships next to the binary
//! ([`shipped_sample`]), downloaded when none ships. A script or a JSON
//! caller gets the error instead, which [`is_wanted`] tells apart.

use std::{
    env, fmt, fs,
    io::{IsTerminal, stderr},
    path::{Path, PathBuf},
};

use crate::{printer::Printer, shell};

/// The welcome a person meets when no configuration exists at `path`,
/// written to stderr.
///
/// It prints the intro, the commands creating the file from the shipped
/// sample or downloading it without one, then the first command, each
/// paragraph on one line for the terminal to fold.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Welcome<'a> {
    /// The binary's name, which the first command starts with.
    pub bin: &'a str,
    /// The first line, introducing the binary.
    pub intro: &'a str,
    /// The configuration file found missing.
    pub path: &'a Path,
    /// The `-c` paths the binary ran with, which the first command
    /// carries.
    pub config_paths: &'a [PathBuf],
    /// The sample configuration shipped next to the binary, as
    /// [`shipped_sample`] finds it.
    pub sample: Option<&'a Path>,
    /// The raw URL of the sample configuration, downloaded when none
    /// ships.
    pub sample_url: &'a str,
    /// The sentence leading to the first command, once the file exists.
    pub next: &'a str,
    /// The first command to run, after the binary and its `-c` flags.
    pub command: &'a str,
}

impl fmt::Display for Welcome<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let path = self.path.display();
        let file = shell::quote(&self.path.to_string_lossy());

        writeln!(f, "{}", self.intro)?;
        writeln!(f)?;
        writeln!(
            f,
            "No configuration found at {path}. Create it from the sample configuration:"
        )?;
        writeln!(f)?;

        if let Some(sample) = self.sample {
            let parent = self.path.parent().filter(|dir| !dir.as_os_str().is_empty());

            if let Some(dir) = parent.filter(|dir| !dir.is_dir()) {
                let mkdir = if cfg!(windows) { "mkdir" } else { "mkdir -p" };
                let dir = shell::quote(&dir.to_string_lossy());
                writeln!(f, "  {mkdir} {dir}")?;
            }

            // A sample from the Nix store is read-only, which `cp` would keep.
            let copy = if cfg!(windows) {
                "copy"
            } else {
                "install -m 644"
            };
            let sample = shell::quote(&sample.to_string_lossy());
            writeln!(f, "  {copy} {sample} {file}")?;
        } else {
            // In Windows PowerShell 5.1, `curl` is Invoke-WebRequest.
            let curl = if cfg!(windows) { "curl.exe" } else { "curl" };
            let url = self.sample_url;
            writeln!(f, "  {curl} -sSL --create-dirs -o {file} {url}")?;
        }

        writeln!(f)?;
        writeln!(f, "{}", self.next)?;
        writeln!(f)?;

        let invocation = shell::invocation(self.bin, self.config_paths, None);

        match self.command {
            "" => writeln!(f, "  {invocation}"),
            command => writeln!(f, "  {invocation} {command}"),
        }
    }
}

/// Finds the sample configuration a release or a package ships next to
/// the running binary `bin`.
pub fn shipped_sample(bin: &str) -> Option<PathBuf> {
    let exe = env::current_exe().ok()?;

    // A symlinked binary (a Nix profile, Homebrew) ships it beside its
    // target, and a canonical Windows path carries a `\\?\` prefix.
    let exe = if cfg!(windows) {
        exe
    } else {
        fs::canonicalize(exe).ok()?
    };

    find_sample(exe.parent()?, bin)
}

/// Whether a person reads the output, who then meets the welcome rather
/// than the error: the printer does not print JSON and stderr is a
/// terminal.
pub fn is_wanted(printer: &impl Printer) -> bool {
    !printer.is_json() && stderr().is_terminal()
}

/// Finds share/doc/<bin>/config.sample.toml under the prefix holding
/// `dir`, the directory of the binary, or under `dir` itself, where an
/// extracted release archive puts it.
fn find_sample(dir: &Path, bin: &str) -> Option<PathBuf> {
    let sample: PathBuf = ["share", "doc", bin, "config.sample.toml"].iter().collect();

    [dir.parent(), Some(dir)]
        .into_iter()
        .flatten()
        .map(|prefix| prefix.join(&sample))
        .find(|path| path.is_file())
}

#[cfg(test)]
mod tests {
    use std::{
        env, fs,
        path::{Path, PathBuf},
        process,
    };

    use crate::welcome::*;

    const URL: &str = "https://example.org/config.sample.toml";

    fn welcome<'a>(paths: &'a [PathBuf], sample: Option<&'a Path>) -> Welcome<'a> {
        Welcome {
            bin: "app",
            intro: "Welcome to App.",
            path: &paths[0],
            config_paths: paths,
            sample,
            sample_url: URL,
            next: "Edit it, then run a first command:",
            command: "run",
        }
    }

    #[test]
    #[cfg(not(windows))]
    fn the_welcome_reads_in_paragraphs() {
        let paths = [PathBuf::from("/nonexistent/app/config.toml")];

        let expected = "\
Welcome to App.

No configuration found at /nonexistent/app/config.toml. Create it from the sample configuration:

  curl -sSL --create-dirs -o '/nonexistent/app/config.toml' https://example.org/config.sample.toml

Edit it, then run a first command:

  app -c '/nonexistent/app/config.toml' run
";
        assert_eq!(welcome(&paths, None).to_string(), expected);
    }

    #[test]
    #[cfg(not(windows))]
    fn the_welcome_commands_run_as_printed() {
        let paths = [PathBuf::from("/a b/it's.toml")];
        let file = r"'/a b/it'\''s.toml'";

        let downloaded = welcome(&paths, None).to_string();
        let curl = format!("\n  curl -sSL --create-dirs -o {file} https://");
        assert!(downloaded.contains(&curl));
        assert!(downloaded.contains(&format!("\n  app -c {file} run\n")));

        let sample = Path::new("/usr/share/doc/app/config.sample.toml");
        let copied = welcome(&paths, Some(sample)).to_string();
        let copy = format!(
            "\n  mkdir -p '/a b'\n  install -m 644 '{}' {file}\n",
            sample.display()
        );
        assert!(copied.contains(&copy));
        assert!(!copied.contains("curl"));
    }

    #[test]
    #[cfg(windows)]
    fn the_welcome_commands_run_in_cmd_and_powershell() {
        let paths = [PathBuf::from(r"C:\a b\c.toml")];
        let welcome = welcome(&paths, None).to_string();
        let curl = r#"  curl.exe -sSL --create-dirs -o "C:\a b\c.toml" https://"#;
        assert!(welcome.contains(curl));
        assert!(welcome.contains(r#"  app -c "C:\a b\c.toml" run"#));
    }

    #[test]
    fn the_shipped_sample_is_found_installed_or_extracted() {
        let name = format!("pimalaya-cli-welcome-{}", process::id());
        let prefix = env::temp_dir().join(name);
        let bin = prefix.join("bin");
        let sample = prefix.join("share/doc/app/config.sample.toml");
        fs::create_dir_all(&bin).unwrap();
        fs::create_dir_all(sample.parent().unwrap()).unwrap();
        assert_eq!(find_sample(&bin, "app"), None);

        fs::write(&sample, "").unwrap();
        assert_eq!(find_sample(&bin, "app"), Some(sample.clone()));
        assert_eq!(find_sample(&prefix, "app"), Some(sample));

        fs::remove_dir_all(&prefix).unwrap();
    }
}
