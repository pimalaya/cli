use crate::clap::args::LogFlags;

pub struct Logger;

impl Logger {
    pub fn init(log: &LogFlags) {
        env_logger::Builder::new()
            .filter_level(log.level.into())
            .parse_default_env()
            .init();
    }
}
