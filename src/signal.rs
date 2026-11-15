//! Termination signals: SIGINT, SIGTERM and SIGHUP, Ctrl-C and
//! Ctrl-Break on Windows.
//!
//! The process holds one handler for them, which the spinner and a
//! long-lived command share. With no [`Shutdown`] installed, a signal
//! ends the process at once with status 130, the terminal restored. Once
//! one is installed, the first signal only requests the shutdown, which
//! the command polls to stop cleanly and print its result; a second one
//! ends the process as before, for a command that does not stop.

use std::{
    process,
    sync::{
        OnceLock,
        atomic::{AtomicBool, Ordering},
    },
};

use anyhow::{Result, anyhow};

/// The exit status of a process a signal ended, 128 + SIGINT.
pub const EXIT_SIGNALED: i32 = 130;

static HANDLER: OnceLock<Result<(), String>> = OnceLock::new();
static CLEANUP: OnceLock<fn()> = OnceLock::new();
static GRACEFUL: AtomicBool = AtomicBool::new(false);
static REQUESTED: AtomicBool = AtomicBool::new(false);

/// The shutdown a long-lived command polls: requested by the first
/// termination signal once installed.
///
/// Every clone reads the same request, so a worker thread can hold one.
#[derive(Clone, Debug)]
pub struct Shutdown(());

impl Shutdown {
    /// Installs the shutdown: from now on the first termination signal
    /// requests it rather than ending the process.
    pub fn install() -> Result<Self> {
        GRACEFUL.store(true, Ordering::SeqCst);
        install_handler()?;
        Ok(Self(()))
    }

    /// Whether a termination signal, or [`Shutdown::request`], asked
    /// the command to stop.
    pub fn is_requested(&self) -> bool {
        REQUESTED.load(Ordering::SeqCst)
    }

    /// Requests the shutdown, as a termination signal does.
    pub fn request(&self) {
        REQUESTED.store(true, Ordering::SeqCst);
    }
}

/// Installs the process's one handler, once; `cleanup` restores what the
/// caller changed (the spinner's hidden cursor) before the process ends.
#[cfg(feature = "spinner")]
pub(crate) fn install_handler_with(cleanup: fn()) -> Result<()> {
    let _ = CLEANUP.set(cleanup);
    install_handler()
}

fn install_handler() -> Result<()> {
    HANDLER
        .get_or_init(|| ctrlc::set_handler(handle).map_err(|err| err.to_string()))
        .clone()
        .map_err(|err| anyhow!("Cannot install the termination signal handler: {err}"))
}

fn handle() {
    if GRACEFUL.load(Ordering::SeqCst) && !REQUESTED.swap(true, Ordering::SeqCst) {
        return;
    }

    if let Some(cleanup) = CLEANUP.get() {
        cleanup();
    }

    process::exit(EXIT_SIGNALED);
}
