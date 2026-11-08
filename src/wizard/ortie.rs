//! # Ortie sign-in
//!
//! Hands an OAuth 2.0 sign-in off to Ortie, the Pimalaya token broker:
//! `ortie configure` sets the account up, `ortie auth get` grants it, and
//! the configuration reads tokens through [`TokenBroker::Ortie`]. Ortie's
//! stdout goes to this process's stderr, so its prompts and output reach
//! the terminal and never a caller's redirected stdout.

use std::{
    io,
    process::{Command, ExitStatus, Stdio},
};

use thiserror::Error;

use crate::wizard::keyring::TokenBroker;

/// Error raised when signing in through Ortie fails.
#[derive(Debug, Error)]
pub enum OrtieSignInError {
    /// The `ortie` program is not on the `PATH`.
    #[error("Cannot find ortie on PATH")]
    NotInstalled,
    /// Duplicating the stderr handle for Ortie's stdout failed.
    #[error("Cannot send ortie output to stderr")]
    Stderr(#[source] io::Error),
    /// An Ortie command could not be started.
    #[error("Cannot run {command}")]
    Spawn {
        /// The command, as `ortie configure`.
        command: &'static str,
        /// The cause.
        #[source]
        err: io::Error,
    },
    /// An Ortie command exited unsuccessfully.
    #[error("Cannot sign in through {command}, {status}")]
    Failed {
        /// The command, as `ortie auth get`.
        command: &'static str,
        /// Its exit status.
        status: ExitStatus,
    },
}

/// Signs the Ortie account `name` in for `input` (an address) and the
/// given `services`, answering the argv reading its access token.
pub fn sign_in(
    input: &str,
    name: &str,
    services: &[&str],
) -> Result<Vec<String>, OrtieSignInError> {
    if !TokenBroker::Ortie.installed() {
        return Err(OrtieSignInError::NotInstalled);
    }

    run("ortie configure", &configure_args(input, name, services))?;
    run("ortie auth get", &["auth", "get", "-a", name])?;

    Ok(TokenBroker::Ortie.read_command(name))
}

/// The `ortie configure` arguments, `--` keeping a dashed input positional.
fn configure_args<'a>(input: &'a str, name: &'a str, services: &[&'a str]) -> Vec<&'a str> {
    let mut args = vec!["configure", "--name", name];

    for service in services {
        args.push("--service");
        args.push(service);
    }

    args.push("--");
    args.push(input);

    args
}

/// Runs `ortie` with `args`, its stdout sent to stderr.
fn run(command: &'static str, args: &[&str]) -> Result<(), OrtieSignInError> {
    let stdout = stderr_copy().map_err(OrtieSignInError::Stderr)?;

    let status = Command::new("ortie")
        .args(args)
        .stdout(stdout)
        .status()
        .map_err(|err| OrtieSignInError::Spawn { command, err })?;

    if !status.success() {
        return Err(OrtieSignInError::Failed { command, status });
    }

    Ok(())
}

/// A duplicate of this process's stderr, for a child's stdout.
#[cfg(unix)]
fn stderr_copy() -> io::Result<Stdio> {
    use std::os::fd::AsFd;

    let fd = io::stderr().as_fd().try_clone_to_owned()?;

    Ok(Stdio::from(fd))
}

/// A duplicate of this process's stderr, for a child's stdout.
#[cfg(windows)]
fn stderr_copy() -> io::Result<Stdio> {
    use std::os::windows::io::AsHandle;

    let handle = io::stderr().as_handle().try_clone_to_owned()?;

    Ok(Stdio::from(handle))
}

#[cfg(test)]
mod tests {
    use crate::wizard::ortie::*;

    #[test]
    fn configure_names_the_account_and_each_service_before_the_input() {
        assert_eq!(
            configure_args("jane@example.org", "example", &["imap", "smtp"]),
            [
                "configure",
                "--name",
                "example",
                "--service",
                "imap",
                "--service",
                "smtp",
                "--",
                "jane@example.org"
            ],
        );
        assert_eq!(
            configure_args("-h", "example", &[]),
            ["configure", "--name", "example", "--", "-h"],
        );
    }
}
