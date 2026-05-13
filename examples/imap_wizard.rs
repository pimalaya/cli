//! Runs the IMAP configuration wizard against an email address read
//! from a CLI arg or the `EMAIL` env var. No autoconfig discovery is
//! performed — the wizard falls back to standard defaults
//! (`imap.<domain>`, port 993, TLS).
//!
//! ```sh
//! EMAIL=alice@example.com cargo run --example imap-wizard
//! # or
//! cargo run --example imap-wizard -- alice@example.com
//! ```

use std::env;

use pimalaya_cli::wizard;

fn main() {
    let email = env::args()
        .nth(1)
        .or_else(|| env::var("EMAIL").ok())
        .expect("EMAIL env var or first CLI arg");

    let (local, domain) = email.rsplit_once('@').expect("EMAIL must contain `@`");

    let account = format!("{local}-{domain}");
    let config = wizard::imap::run(&account, local, domain, None).unwrap();

    println!();
    println!("--- IMAP config ---");
    println!("host:       {}", config.host);
    println!("port:       {}", config.port);
    println!("encryption: {}", config.encryption);
    println!("login:      {}", config.login);
}
