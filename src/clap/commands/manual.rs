use std::{fs, path::PathBuf};

use anyhow::Result;
use clap::{Command, Parser};
use clap_mangen::Man;
use log::info;

use crate::{clap::parsers::path_parser, printer::Printer};

/// Generate manual pages to the given directory.
///
/// This command allows you to generate manual pages (following the
/// man page format) to the given directory. If the directory does not
/// exist, it will be created. Any existing man pages will be
/// overriden.
#[derive(Debug, Parser)]
pub struct ManualCommand {
    /// Directory where man files should be generated in.
    #[arg(value_parser = path_parser)]
    pub dir: PathBuf,
}

impl ManualCommand {
    /// Generates the man pages and reports how many landed where.
    pub fn execute(self, printer: &mut impl Printer, command: Command) -> Result<()> {
        let dir = &self.dir;
        let cmd_name = command.get_name().to_string();
        let subcmds = command.get_subcommands().cloned().collect::<Vec<_>>();
        let subcmds_len = subcmds.len() + 1;

        let mut buffer = Vec::new();
        Man::new(command).render(&mut buffer)?;

        fs::create_dir_all(dir)?;
        info!("generate man page for command {cmd_name}");
        fs::write(dir.join(format!("{}.1", cmd_name)), buffer)?;

        for subcmd in subcmds {
            let subcmd_name = subcmd.get_name().to_string();

            let mut buffer = Vec::new();
            Man::new(subcmd).render(&mut buffer)?;

            info!("generate man page for subcommand {subcmd_name}");
            fs::write(dir.join(format!("{}-{}.1", cmd_name, subcmd_name)), buffer)?;
        }

        printer.out(format!(
            "{subcmds_len} man page(s) successfully generated in {}",
            dir.display()
        ))
    }
}
