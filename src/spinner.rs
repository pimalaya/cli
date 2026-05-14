//! Lightweight terminal spinner built on crossterm.
//!
//! Renders a single-line braille spinner with a live message on
//! stderr (so stdout stays clean for piped data), runs on a
//! background OS thread, and disables itself when stderr is not a
//! TTY.
//!
//! Typical use during a discovery flow:
//!
//! ```no_run
//! use pimalaya_cli::spinner::Spinner;
//!
//! let spinner = Spinner::start("Probing autoconfig…");
//! spinner.set_message("Trying imap.example.com:993");
//! // … run the discovery work …
//! spinner.success("Found IMAP server at imap.example.com:993");
//! ```

use std::{
    io::{stderr, IsTerminal, Write},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

use crossterm::{
    cursor,
    style::{Color, Print, PrintStyledContent, Stylize},
    terminal::{Clear, ClearType},
    QueueableCommand,
};

const FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

const FRAME_INTERVAL: Duration = Duration::from_millis(80);

pub struct Spinner {
    message: Arc<Mutex<String>>,
    stop: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

impl Spinner {
    /// Start the spinner with an initial message.
    ///
    /// When stderr is not a TTY no thread is spawned and ticks are
    /// suppressed; terminal completion lines (`success`/`failure`)
    /// are still printed so logs remain informative.
    pub fn start(message: impl Into<String>) -> Self {
        let message = Arc::new(Mutex::new(message.into()));
        let stop = Arc::new(AtomicBool::new(false));

        let handle = if stderr().is_terminal() {
            let message = message.clone();
            let stop = stop.clone();
            Some(thread::spawn(move || render_loop(message, stop)))
        } else {
            None
        };

        Self {
            message,
            stop,
            handle,
        }
    }

    /// Replace the message displayed next to the spinner frame.
    pub fn set_message(&self, message: impl Into<String>) {
        if let Ok(mut current) = self.message.lock() {
            *current = message.into();
        }
    }

    /// Stop the spinner and print a green `✓` followed by `message`.
    pub fn success(self, message: impl AsRef<str>) {
        self.finish("✓", Color::Green, message.as_ref());
    }

    /// Stop the spinner and print a red `✗` followed by `message`.
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
        out.queue(PrintStyledContent(glyph.with(color))).ok();
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

fn render_loop(message: Arc<Mutex<String>>, stop: Arc<AtomicBool>) {
    let mut out = stderr();

    out.queue(cursor::Hide).ok();
    out.flush().ok();

    let mut frame = 0;
    while !stop.load(Ordering::Relaxed) {
        let snapshot = message.lock().map(|m| m.clone()).unwrap_or_default();

        out.queue(cursor::MoveToColumn(0)).ok();
        out.queue(Clear(ClearType::CurrentLine)).ok();
        out.queue(PrintStyledContent(FRAMES[frame].with(Color::Cyan)))
            .ok();
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
