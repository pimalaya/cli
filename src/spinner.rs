//! Lightweight terminal spinner built on crossterm.
//!
//! Renders a single-line braille spinner with a live message on
//! stderr (so stdout stays clean for piped data), runs on a
//! background OS thread, and disables itself when stderr is not a
//! TTY. Its glyphs are coloured on a TTY only, and never under a
//! non-empty `NO_COLOR` (<https://no-color.org>).
//!
//! Typical use around a long-running operation:
//!
//! ```no_run
//! use pimalaya_cli::spinner::Spinner;
//!
//! let spinner = Spinner::start("Connecting…");
//! spinner.set_message("Listing mailboxes");
//! // … run the long work …
//! spinner.success("Listed 12 mailboxes");
//! ```

use std::{
    env,
    ffi::OsString,
    io::{IsTerminal, Write, stderr},
    sync::{
        Arc, Mutex, OnceLock,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, JoinHandle},
    time::Duration,
};

use crossterm::{
    QueueableCommand, cursor,
    style::{Color, Print, PrintStyledContent, Stylize},
    terminal::{Clear, ClearType},
};

use crate::signal;

const FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

const FRAME_INTERVAL: Duration = Duration::from_millis(80);

/// A single-line terminal spinner with a live message.
pub struct Spinner {
    message: Arc<Mutex<String>>,
    stop: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
    colour: bool,
}

impl Spinner {
    /// Start the spinner with an initial message.
    ///
    /// When stderr is not a TTY no thread is spawned and ticks are
    /// suppressed; terminal completion lines (`success`/`failure`)
    /// are still printed, as plain text, so logs remain informative.
    ///
    /// The first TTY-bound call also installs a process-wide SIGINT /
    /// SIGTERM / SIGHUP handler and a panic hook that restore the
    /// cursor before letting the process die: Drop is skipped on
    /// signal-induced termination and on panic = abort, so without
    /// these hooks `cursor::Hide` from the render loop would leak
    /// past process exit.
    pub fn start(message: impl Into<String>) -> Self {
        let message = Arc::new(Mutex::new(message.into()));
        let stop = Arc::new(AtomicBool::new(false));
        let terminal = stderr().is_terminal();
        let colour = colour_wanted(terminal, env::var_os("NO_COLOR"));

        let handle = if terminal {
            install_terminal_guards();
            let message = message.clone();
            let stop = stop.clone();
            Some(thread::spawn(move || render_loop(message, stop, colour)))
        } else {
            None
        };

        Self {
            message,
            stop,
            handle,
            colour,
        }
    }

    /// Replace the message displayed next to the spinner frame.
    pub fn set_message(&self, message: impl Into<String>) {
        if let Ok(mut current) = self.message.lock() {
            *current = message.into();
        }
    }

    /// Stop the spinner and print `✓` (green on a TTY) and `message`.
    pub fn success(self, message: impl AsRef<str>) {
        self.finish("✓", Color::Green, message.as_ref());
    }

    /// Stop the spinner and print `✗` (red on a TTY) and `message`.
    pub fn failure(self, message: impl AsRef<str>) {
        self.finish("✗", Color::Red, message.as_ref());
    }

    /// Stop the spinner and clear its line without printing anything
    /// in its place.
    pub fn clear(mut self) {
        self.shutdown();
    }

    fn finish(mut self, glyph: &str, color: Color, message: &str) {
        self.shutdown();

        let mut out = stderr();
        queue_glyph(&mut out, glyph, color, self.colour);
        out.queue(Print(format!(" {message}\n"))).ok();
        out.flush().ok();
    }

    fn shutdown(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            handle.join().ok();
        }
    }
}

impl Drop for Spinner {
    fn drop(&mut self) {
        self.shutdown();
    }
}

static TERMINAL_GUARDS: OnceLock<()> = OnceLock::new();

fn install_terminal_guards() {
    TERMINAL_GUARDS.get_or_init(|| {
        // NOTE: a signal ending the process bypasses Drop, so the render
        // loop never reaches its `cursor::Show` tail: the shared handler
        // restores the terminal first, then exits 130. Under an
        // installed Shutdown the first signal only requests it, and the
        // spinner is dropped as the command stops.
        let _ = signal::install_handler_with(restore_terminal);

        // panic = abort skips Drop too. Chain a hook in front of the
        // default so the panic message still reaches stderr after the
        // cursor is restored.
        let default_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            restore_terminal();
            default_hook(info);
        }));
    });
}

fn restore_terminal() {
    let mut out = stderr();
    out.queue(cursor::MoveToColumn(0)).ok();
    out.queue(Clear(ClearType::CurrentLine)).ok();
    out.queue(cursor::Show).ok();
    out.flush().ok();
}

/// Whether to colour: on a TTY, unless `NO_COLOR` is set and not empty.
fn colour_wanted(terminal: bool, no_color: Option<OsString>) -> bool {
    terminal && no_color.is_none_or(|value| value.is_empty())
}

/// Queue `glyph`, plain unless `colour`: crossterm under `NO_COLOR` still
/// writes `ESC[m` around it.
fn queue_glyph(out: &mut impl Write, glyph: &str, color: Color, colour: bool) {
    if colour {
        out.queue(PrintStyledContent(glyph.with(color))).ok();
    } else {
        out.queue(Print(glyph)).ok();
    }
}

fn render_loop(message: Arc<Mutex<String>>, stop: Arc<AtomicBool>, colour: bool) {
    let mut out = stderr();

    out.queue(cursor::Hide).ok();
    out.flush().ok();

    let mut frame = 0;
    while !stop.load(Ordering::Relaxed) {
        let snapshot = message.lock().map(|m| m.clone()).unwrap_or_default();

        out.queue(cursor::MoveToColumn(0)).ok();
        out.queue(Clear(ClearType::CurrentLine)).ok();
        queue_glyph(&mut out, FRAMES[frame], Color::Cyan, colour);
        out.queue(Print(format!(" {snapshot}"))).ok();
        out.flush().ok();

        frame = (frame + 1) % FRAMES.len();
        thread::sleep(FRAME_INTERVAL);
    }

    out.queue(cursor::MoveToColumn(0)).ok();
    out.queue(Clear(ClearType::CurrentLine)).ok();
    out.queue(cursor::Show).ok();
    out.flush().ok();
}

#[cfg(test)]
mod tests {
    use std::{env, ffi::OsString, process::Command};

    use super::{Spinner, colour_wanted};

    const CHILD: &str = "PIMALAYA_CLI_SPINNER_CHILD";

    #[test]
    fn an_empty_no_color_keeps_the_colour() {
        assert!(colour_wanted(true, None));
        assert!(colour_wanted(true, Some(OsString::new())));
        assert!(!colour_wanted(true, Some(OsString::from("0"))));
        assert!(!colour_wanted(false, None));
    }

    #[test]
    fn a_stream_that_is_no_terminal_gets_plain_lines() {
        if env::var_os(CHILD).is_some() {
            Spinner::start("Checking…").success("Checked");
            Spinner::start("Sending…").failure("Not sent");
            return;
        }

        // The test binary runs this test again, its stderr a pipe.
        for no_color in [None, Some("1")] {
            let mut child = Command::new(env::current_exe().unwrap());
            child
                .args([
                    "--exact",
                    "spinner::tests::a_stream_that_is_no_terminal_gets_plain_lines",
                ])
                .env(CHILD, "1");
            match no_color {
                Some(value) => child.env("NO_COLOR", value),
                None => child.env_remove("NO_COLOR"),
            };

            let output = child.output().unwrap();
            assert!(output.status.success());
            assert_eq!(
                String::from_utf8_lossy(&output.stderr),
                "✓ Checked\n✗ Not sent\n",
                "NO_COLOR={no_color:?}",
            );
        }
    }
}
