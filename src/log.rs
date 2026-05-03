use std::env;

use crate::clap::args::LogFlags;

pub struct Logger;

impl Logger {
    pub fn init(log: &LogFlags) {
        if log.quiet {
            env::set_var("RUST_LOG", "off");
        } else if log.debug {
            env::set_var("RUST_LOG", "debug");
        } else if log.trace {
            env::set_var("RUST_LOG", "trace");
            env::set_var("RUST_BACKTRACE", "1");
        }

        env_logger::init();
    }
}
