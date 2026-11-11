//! The stdin of a session.
//!
//! A REPL or an MCP server reads its commands off stdin, so the commands
//! it runs must not read their own input there: the next lines would go
//! to the command instead of the session. The session claims stdin before
//! reading it, and a command that would read stdin checks it first.

use std::sync::atomic::{AtomicBool, Ordering};

use anyhow::Result;

use crate::error::{CodedError, ErrorCode};

/// Whether a session reads its commands off stdin.
static IN_SESSION: AtomicBool = AtomicBool::new(false);

/// Marks stdin as read by a session rather than by the commands it runs.
///
/// The REPL and the MCP server call it before reading stdin; it lasts
/// for the rest of the process.
pub fn claim_stdin() {
    IN_SESSION.store(true, Ordering::Relaxed);
}

/// Refuses to read a command's input off stdin inside a session, where it
/// carries the commands that follow, with the binary's `code`.
pub fn check_stdin<C: ErrorCode>(code: C) -> Result<()> {
    if in_session() {
        let message = "Cannot read input from stdin inside a session, \
                       pass it as an argument or a file path";
        return Err(CodedError::new(code, message).into());
    }

    Ok(())
}

/// Whether the commands run inside a session reading stdin.
pub fn in_session() -> bool {
    IN_SESSION.load(Ordering::Relaxed)
}

#[cfg(test)]
mod tests {
    use schemars::JsonSchema;
    use serde::Serialize;

    use crate::{
        error::code_of,
        session::{check_stdin, claim_stdin, in_session},
    };

    #[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, JsonSchema)]
    #[serde(rename_all = "kebab-case")]
    enum TestCode {
        StdinTaken,
    }

    // NOTE: the claim lasts for the process, no test may assert stdin free.
    #[test]
    fn a_claimed_stdin_refuses_command_input_with_the_code() {
        claim_stdin();

        assert!(in_session());
        let err = check_stdin(TestCode::StdinTaken).unwrap_err();
        assert_eq!(code_of(&err), Some(TestCode::StdinTaken));
        assert!(err.to_string().starts_with("Cannot read input from stdin"));
    }
}
