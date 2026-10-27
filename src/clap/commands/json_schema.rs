use std::{collections::BTreeMap, fmt, fs, path::PathBuf};

use anyhow::{Result, bail};
use clap::Parser;
use log::debug;
use serde::Serialize;
use serde_json::Value;

use crate::{clap::parsers::path_parser, printer::Printer};

/// A binary's command-to-schema map, with what stamps it as one
/// version of the machine contract.
///
/// Keys are command names (e.g. `himalaya-envelope-list`), values the
/// already-built JSON Schema of that command's output. Callers own the
/// mapping since the output shapes live in the binary, not in this
/// toolkit.
pub struct JsonSchemaRegistry {
    /// The binary name, as in `himalaya`.
    pub bin: &'static str,
    /// The binary version, usually `env!("CARGO_PKG_VERSION")`.
    pub version: &'static str,
    /// The schemas, keyed by command name.
    pub schemas: BTreeMap<String, Value>,
}

impl JsonSchemaRegistry {
    /// The keys whose schema root is not a JSON object.
    ///
    /// A machine caller such as an MCP client takes a structured result
    /// only as an object, so a binary's registry test asserts this is
    /// empty. A root counts as an object when it is typed `object`, when
    /// it references an object definition, or when every alternative of
    /// its `anyOf` or `oneOf` is one.
    pub fn non_object_roots(&self) -> Vec<&str> {
        self.schemas
            .iter()
            .filter(|(_, schema)| !is_object(schema, schema))
            .map(|(key, _)| key.as_str())
            .collect()
    }

    /// The contract version: the major version, or `0.<minor>` before
    /// 1.0, since semver breaks on the minor there.
    fn contract_version(&self) -> String {
        let mut parts = self.version.split('.');
        let major = parts.next().unwrap_or_default();

        match (major, parts.next()) {
            ("0", Some(minor)) => format!("0.{minor}"),
            _ => major.to_owned(),
        }
    }

    /// Stamps each schema with its `$id`, naming the binary, the
    /// contract version and the command.
    fn stamp(&mut self) {
        let bin = self.bin;
        let version = self.contract_version();

        for (key, schema) in &mut self.schemas {
            if let Value::Object(schema) = schema {
                let id = format!("https://pimalaya.org/schemas/{bin}/v{version}/{key}.json");
                schema.insert("$id".into(), Value::String(id));
            }
        }
    }
}

/// Whether `schema`, resolved against the `root` holding its
/// definitions, describes a JSON object.
fn is_object(root: &Value, schema: &Value) -> bool {
    if schema["type"] == "object" {
        return true;
    }

    if let Some(pointer) = schema["$ref"].as_str().and_then(|r| r.strip_prefix('#')) {
        return root
            .pointer(pointer)
            .is_some_and(|def| is_object(root, def));
    }

    ["anyOf", "oneOf"].iter().any(|key| {
        schema[key]
            .as_array()
            .is_some_and(|alts| !alts.is_empty() && alts.iter().all(|alt| is_object(root, alt)))
    })
}

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
    /// Each schema is stamped with its `$id` first, then written the way
    /// [`ManualCommand`] renders one man page per subcommand.
    ///
    /// [`ManualCommand`]: crate::clap::commands::ManualCommand
    pub fn execute(
        self,
        printer: &mut impl Printer,
        mut registry: JsonSchemaRegistry,
    ) -> Result<()> {
        registry.stamp();
        let mut schemas = registry.schemas;

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

    use super::{JsonSchemaCommand, JsonSchemaRegistry};
    use crate::printer::TestPrinter;

    fn registry(version: &'static str) -> JsonSchemaRegistry {
        JsonSchemaRegistry {
            bin: "test",
            version,
            schemas: BTreeMap::from_iter([
                ("test-a".into(), json!({ "type": "object" })),
                ("test-b".into(), json!({ "type": "array" })),
            ]),
        }
    }

    #[test]
    fn single_command_schema_goes_to_output_stamped() {
        let cmd = JsonSchemaCommand {
            cmds: vec!["test-b".into()],
            dir: None,
        };

        let mut printer = TestPrinter::default();
        cmd.execute(&mut printer, registry("2.3.1")).unwrap();

        let schema: serde_json::Value = serde_json::from_str(&printer.0).unwrap();
        assert_eq!(
            schema,
            json!({
                "$id": "https://pimalaya.org/schemas/test/v2/test-b.json",
                "type": "array",
            })
        );
    }

    #[test]
    fn a_pre_release_contract_is_versioned_by_its_minor() {
        assert_eq!(registry("0.4.3").contract_version(), "0.4");
        assert_eq!(registry("1.0.0").contract_version(), "1");
    }

    #[test]
    fn several_commands_without_dir_are_rejected() {
        let cmd = JsonSchemaCommand {
            cmds: Vec::new(),
            dir: None,
        };

        let err = cmd
            .execute(&mut TestPrinter::default(), registry("1.0.0"))
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
            .execute(&mut TestPrinter::default(), registry("1.0.0"))
            .unwrap_err();

        assert_eq!(err.to_string(), "Cannot find command test-c");
    }

    #[test]
    fn non_object_roots_are_reported() {
        let mut registry = registry("1.0.0");

        registry.schemas.extend([
            (
                "test-ref".into(),
                json!({ "$ref": "#/$defs/A", "$defs": { "A": { "type": "object" } } }),
            ),
            (
                "test-any".into(),
                json!({ "anyOf": [{ "type": "object" }, { "$ref": "#/$defs/B" }],
                        "$defs": { "B": { "type": "string" } } }),
            ),
            (
                "test-one".into(),
                json!({ "oneOf": [{ "type": "object" }, { "type": "object" }] }),
            ),
        ]);

        assert_eq!(registry.non_object_roots(), ["test-any", "test-b"]);
    }
}
