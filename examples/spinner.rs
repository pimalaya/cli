//! Demonstrates the spinner during a fake discovery flow.
//!
//! ```sh
//! cargo run --example spinner
//! ```

use std::{thread::sleep, time::Duration};

use pimalaya_cli::spinner::Spinner;

fn main() {
    let spinner = Spinner::start("Resolving MX records for example.com…");
    sleep(Duration::from_secs(1));

    spinner.set_message("Probing imap.example.com:993");
    sleep(Duration::from_secs(1));

    spinner.set_message("Probing smtp.example.com:465");
    sleep(Duration::from_secs(1));

    spinner.success("Found IMAP at imap.example.com:993 and SMTP at smtp.example.com:465");

    let spinner = Spinner::start("Probing pop.example.com:995");
    sleep(Duration::from_secs(1));
    spinner.failure("No POP server reachable at pop.example.com:995");
}
