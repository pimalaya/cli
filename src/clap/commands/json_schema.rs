use std::{collections::BTreeMap, fs, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use log::info;
use serde_json::Value;

use crate::{clap::parsers::path_parser, printer::Printer};

/// Generate JSON Schemas of every command's JSON output to the given
/// directory.
///
/// This command allows you to generate one JSON Schema per command
/// describing the structure of its `--json` output, to the given
/// directory. If the directory does not exist, it will be created. Any
/// existing schema will be overriden.
#[derive(Debug, Parser)]
pub struct JsonSchemaCommand {
    /// Directory where JSON Schema files should be generated in.
    #[arg(value_parser = path_parser)]
    pub dir: PathBuf,
}

impl JsonSchemaCommand {
    /// Generates one `<command>.json` schema file per entry and reports
    /// how many landed where.
    ///
    /// The map is keyed by command name (e.g. `himalaya-envelope-list`)
    /// and valued by the already-built JSON Schema of that command's
    /// output, mirroring how [`ManualCommand`] renders one man page per
    /// subcommand. Callers own the command-to-schema mapping since the
    /// output shapes live in the binary, not in this toolkit.
    ///
    /// [`ManualCommand`]: crate::clap::commands::ManualCommand
    pub fn execute(
        self,
        printer: &mut impl Printer,
        schemas: BTreeMap<String, Value>,
    ) -> Result<()> {
        let dir = &self.dir;
        let count = schemas.len();

        fs::create_dir_all(dir)?;

        for (name, schema) in schemas {
            let json = serde_json::to_vec_pretty(&schema)?;
            info!("generate JSON Schema for command {name}");
            fs::write(dir.join(format!("{name}.json")), json)?;
        }

        printer.out(format!(
            "{count} JSON Schema(s) successfully generated in {}",
            dir.display()
        ))
    }
}
