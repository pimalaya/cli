use std::{collections::BTreeMap, fmt, fs, path::PathBuf};

use anyhow::{Result, bail};
use clap::Parser;
use log::debug;
use serde::Serialize;
use serde_json::Value;

use crate::{clap::parsers::path_parser, printer::Printer};

/// Generate JSON Schema of the given command(s) JSON output.
///
/// This command allows you to generate the JSON Schema describing the
/// `--json` output of a given command. The schema is printed to the
/// standard output, so it can be written to a file using unix
/// redirection. Giving a directory instead writes one schema per
/// command in it, creating the directory if it does not exist.
#[derive(Debug, Parser)]
pub struct JsonSchemaCommand {
    /// Command(s) for which JSON Schema should be generated for.
    ///
    /// A command is named after its full path, dash-separated, as in
    /// `himalaya-envelope-list`. Defaults to every command, which
    /// requires a directory.
    pub cmds: Vec<String>,

    /// Save JSON Schemas to the given directory.
    #[arg(short, long, value_name = "PATH", value_parser = path_parser)]
    pub dir: Option<PathBuf>,
}

impl JsonSchemaCommand {
    /// Generates the JSON Schemas, either to the standard output or
    /// to the given directory.
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
        mut schemas: BTreeMap<String, Value>,
    ) -> Result<()> {
        for cmd in &self.cmds {
            if !schemas.contains_key(cmd) {
                bail!("Cannot find command {cmd}");
            }
        }

        if !self.cmds.is_empty() {
            schemas.retain(|cmd, _| self.cmds.contains(cmd));
        }

        let Some(dir) = self.dir else {
            let mut schemas = schemas.into_iter();

            let (Some((cmd, schema)), None) = (schemas.next(), schemas.next()) else {
                bail!(
                    "Writing several JSON Schemas to the standard output is ambiguous, use --dir to generate them as files"
                );
            };

            debug!("generated {cmd} JSON Schema");

            return printer.out(JsonSchema { cmd, schema });
        };

        let dir = dir.canonicalize().unwrap_or(dir);
        fs::create_dir_all(&dir)?;

        let mut paths = Vec::with_capacity(schemas.len());

        for (cmd, schema) in schemas {
            let json = serde_json::to_vec_pretty(&schema)?;

            let path = dir.join(format!("{cmd}.json"));
            fs::write(&path, json)?;
            debug!("generated {cmd} JSON Schema at {}", path.display());
            paths.push(Schema { cmd, path })
        }

        printer.out(JsonSchemas {
            dir,
            schemas: paths,
        })
    }
}

/// Defines a struct-wrapper to provide a JSON output.
#[derive(Serialize)]
struct JsonSchema {
    pub cmd: String,
    pub schema: Value,
}

impl fmt::Display for JsonSchema {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(&self.schema).map_err(|_| fmt::Error)?;
        writeln!(f, "{json}")
    }
}

/// Defines a struct-wrapper to provide a JSON output.
#[derive(Serialize)]
struct JsonSchemas {
    dir: PathBuf,
    schemas: Vec<Schema>,
}

impl fmt::Display for JsonSchemas {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n = self.schemas.len();
        let p = self.dir.display();
        writeln!(f, "{n} JSON Schema(s) successfully generated in {p}:")?;

        for Schema { cmd, path } in &self.schemas {
            let p = path.display();
            writeln!(f, " - {cmd} JSON Schema at {p}")?;
        }

        Ok(())
    }
}

/// Defines a struct-wrapper to provide a JSON output.
#[derive(Serialize)]
struct Schema {
    pub cmd: String,
    pub path: PathBuf,
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use serde_json::json;

    use super::JsonSchemaCommand;
    use crate::printer::TestPrinter;

    fn schemas() -> BTreeMap<String, serde_json::Value> {
        BTreeMap::from_iter([
            ("test-a".into(), json!({ "type": "string" })),
            ("test-b".into(), json!({ "type": "number" })),
        ])
    }

    #[test]
    fn single_command_schema_goes_to_output() {
        let cmd = JsonSchemaCommand {
            cmds: vec!["test-b".into()],
            dir: None,
        };

        let mut printer = TestPrinter::default();
        cmd.execute(&mut printer, schemas()).unwrap();

        assert_eq!(printer.0, "{\n  \"type\": \"number\"\n}\n");
    }

    #[test]
    fn several_commands_without_dir_are_rejected() {
        let cmd = JsonSchemaCommand {
            cmds: Vec::new(),
            dir: None,
        };

        let err = cmd
            .execute(&mut TestPrinter::default(), schemas())
            .unwrap_err();

        assert!(err.to_string().contains("use --dir"), "{err}");
    }

    #[test]
    fn unknown_command_is_rejected() {
        let cmd = JsonSchemaCommand {
            cmds: vec!["test-c".into()],
            dir: None,
        };

        let err = cmd
            .execute(&mut TestPrinter::default(), schemas())
            .unwrap_err();

        assert_eq!(err.to_string(), "Cannot find command test-c");
    }
}
