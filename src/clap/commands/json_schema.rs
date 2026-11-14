//! The JSON Schema command and the registry of what a binary prints.

use std::{
    collections::{BTreeMap, BTreeSet},
    fmt, fs,
    path::PathBuf,
};

use anyhow::{Result, bail};
use clap::{Command, Parser};
use log::debug;
use schemars::{JsonSchema, generate::SchemaSettings};
use serde::Serialize;
use serde_json::Value;

use crate::{
    clap::{
        commands::{completion::CompletionOutput, manual::ManualOutput},
        parsers::path_parser,
    },
    printer::Printer,
};

/// A binary's command-to-schema maps, with what stamps them as one
/// version of the machine contract.
///
/// Keys are command keys, the command path joined with dashes (e.g.
/// `himalaya-envelope-list`); values are the already-built JSON Schemas
/// of what that command prints. `schemas` describes each command's
/// result, `events` the events a command prints before it. Callers own
/// the mapping since the output shapes live in the binary, not in this
/// toolkit.
pub struct JsonSchemaRegistry {
    /// The binary name, as in `himalaya`.
    pub bin: &'static str,
    /// The binary version, usually `env!("CARGO_PKG_VERSION")`.
    pub version: &'static str,
    /// The result schemas, keyed by command key.
    pub schemas: BTreeMap<String, Value>,
    /// The event schemas, keyed by command key: one type per command,
    /// whose alternatives are its tagged events.
    pub events: BTreeMap<String, Value>,
}

impl JsonSchemaRegistry {
    /// Builds a registry holding the outputs of the toolkit commands,
    /// `completion`, `manual` and `json-schema`, at the root of the
    /// binary's command tree.
    pub fn new(bin: &'static str, version: &'static str) -> Self {
        let mut registry = Self {
            bin,
            version,
            schemas: BTreeMap::new(),
            events: BTreeMap::new(),
        };

        registry.insert::<CompletionOutput>(format!("{bin}-completion"));
        registry.insert::<ManualOutput>(format!("{bin}-manual"));
        registry.insert::<JsonSchemaOutput>(format!("{bin}-json-schema"));

        registry
    }

    /// Registers `T` as the result schema of the command `key`.
    pub fn insert<T: JsonSchema>(&mut self, key: impl Into<String>) {
        self.schemas.insert(key.into(), output_schema::<T>());
    }

    /// Registers `T` as the event schema of the command `key`.
    pub fn insert_events<T: JsonSchema>(&mut self, key: impl Into<String>) {
        self.events.insert(key.into(), output_schema::<T>());
    }

    /// The names of the schemas whose root is not a JSON object.
    ///
    /// A machine caller such as an MCP client takes a structured result
    /// only as an object, so a binary's registry test asserts this is
    /// empty. A root counts as an object when it is typed `object`, when
    /// it references an object definition, or when every alternative of
    /// its `anyOf` or `oneOf` is one. An event schema is named
    /// `<key>.events`.
    pub fn non_object_roots(&self) -> Vec<String> {
        self.files()
            .into_iter()
            .filter(|(_, schema)| !is_object(schema, schema))
            .map(|(name, _)| name)
            .collect()
    }

    /// The problems of the registry against the binary's command tree,
    /// one line each, for a binary's registry test to assert empty.
    ///
    /// Reports every runnable command without a schema in either map,
    /// unless listed in `silent`; every key naming no runnable command,
    /// `<bin>-error` and `<bin>-item-error` aside; every result alternative
    /// holding an `event` key; and every event alternative without one. A
    /// runnable command is one without subcommands or whose subcommand is
    /// optional.
    pub fn check(&self, command: &Command, silent: &[&str]) -> Vec<String> {
        let mut problems = Vec::new();

        let mut runnable = BTreeSet::new();
        collect_runnable(command, command.get_name().to_owned(), &mut runnable);

        for key in &runnable {
            let described = self.schemas.contains_key(key) || self.events.contains_key(key);

            if !described && !silent.contains(&key.as_str()) {
                problems.push(format!("{key}: command without schema"));
            }
        }

        let error_key = format!("{}-error", self.bin);
        let item_error_key = format!("{}-item-error", self.bin);

        for key in self.schemas.keys().chain(self.events.keys()) {
            if !runnable.contains(key) && *key != error_key && *key != item_error_key {
                problems.push(format!("{key}: schema naming no command"));
            }
        }

        for (key, schema) in &self.schemas {
            let mut alts = Vec::new();
            collect_alternatives(schema, schema, &mut alts);

            if alts
                .iter()
                .any(|alt| alt["properties"].get("event").is_some())
            {
                problems.push(format!("{key}: result holding an event key"));
            }
        }

        for (key, schema) in &self.events {
            let mut alts = Vec::new();
            collect_alternatives(schema, schema, &mut alts);

            let tagged = |alt: &&Value| {
                alt["required"]
                    .as_array()
                    .is_some_and(|keys| keys.iter().any(|k| k == "event"))
            };

            if alts.is_empty() || !alts.iter().all(tagged) {
                problems.push(format!("{key}: event without an event key"));
            }
        }

        problems
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

    /// Every schema by file name: `<key>` for a result, `<key>.events`
    /// for events.
    fn files(&self) -> BTreeMap<String, Value> {
        let results = self.schemas.clone().into_iter();

        let events = self
            .events
            .clone()
            .into_iter()
            .map(|(key, schema)| (format!("{key}.events"), schema));

        results.chain(events).collect()
    }

    /// Every schema by file name, each stamped with its `$id` naming the
    /// binary, the contract version and the file.
    fn stamped_files(&self) -> BTreeMap<String, Value> {
        let bin = self.bin;
        let version = self.contract_version();
        let mut files = self.files();

        for (name, schema) in &mut files {
            if let Value::Object(schema) = schema {
                let id = format!("https://pimalaya.org/schemas/{bin}/v{version}/{name}.json");
                schema.insert("$id".into(), Value::String(id));
            }
        }

        files
    }
}

/// The JSON Schema of `T` as printed, rather than as read back.
///
/// The serialize contract marks a field required unless the serializer
/// may skip it, so an `Option` printed as `null` stays required and a
/// field defaulted on deserialization does not turn optional.
pub fn output_schema<T: JsonSchema>() -> Value {
    let schema = SchemaSettings::default()
        .for_serialize()
        .into_generator()
        .into_root_schema_for::<T>();

    serde_json::to_value(schema).expect("serialize JSON Schema")
}

/// Collects the key of every runnable command under `command`, itself
/// included.
fn collect_runnable(command: &Command, key: String, runnable: &mut BTreeSet<String>) {
    let mut subcommands = command.get_subcommands().peekable();

    if subcommands.peek().is_none() || !command.is_subcommand_required_set() {
        runnable.insert(key.clone());
    }

    for subcommand in subcommands {
        let key = format!("{key}-{}", subcommand.get_name());
        collect_runnable(subcommand, key, runnable);
    }
}

/// Collects the alternatives `schema` describes, resolved against the
/// `root` holding its definitions: itself, or the alternatives of its
/// reference, `anyOf` or `oneOf`.
fn collect_alternatives<'a>(root: &'a Value, schema: &'a Value, alts: &mut Vec<&'a Value>) {
    if let Some(pointer) = schema["$ref"].as_str().and_then(|r| r.strip_prefix('#')) {
        if let Some(def) = root.pointer(pointer) {
            collect_alternatives(root, def, alts);
        }

        return;
    }

    let mut branched = false;

    for key in ["anyOf", "oneOf"] {
        if let Some(branches) = schema[key].as_array() {
            branched = true;

            for branch in branches {
                collect_alternatives(root, branch, alts);
            }
        }
    }

    if !branched || schema.get("properties").is_some() {
        alts.push(schema);
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
    /// A command is named after its full path: the program name followed
    /// by each subcommand name, dash-separated; the events a command
    /// prints before its result are named after it with an `.events`
    /// suffix. Defaults to every schema, which requires a directory.
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
    /// [`ManualCommand`]: crate::clap::commands::manual::ManualCommand
    pub fn execute(self, printer: &mut impl Printer, registry: JsonSchemaRegistry) -> Result<()> {
        let mut schemas = registry.stamped_files();

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

            return printer.out(JsonSchemaOutput::Schema { cmd, schema });
        };

        let dir = dir.canonicalize().unwrap_or(dir);
        fs::create_dir_all(&dir)?;

        let mut paths = Vec::with_capacity(schemas.len());

        for (cmd, schema) in schemas {
            let json = serde_json::to_vec_pretty(&schema)?;

            let path = dir.join(format!("{cmd}.json"));
            fs::write(&path, json)?;
            debug!("generated {cmd} JSON Schema at {}", path.display());
            paths.push(SchemaFile { cmd, path })
        }

        printer.out(JsonSchemaOutput::Files {
            dir,
            schemas: paths,
        })
    }
}

/// What `json-schema` prints: the one schema asked, or the files written.
#[derive(Serialize, JsonSchema)]
#[serde(untagged)]
pub(crate) enum JsonSchemaOutput {
    /// The one schema asked for.
    Schema {
        /// The schema name.
        cmd: String,
        /// The JSON Schema.
        schema: Value,
    },
    /// The schema files written.
    Files {
        /// The directory holding the files.
        dir: PathBuf,
        /// The files written, one per schema.
        schemas: Vec<SchemaFile>,
    },
}

impl fmt::Display for JsonSchemaOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Schema { schema, .. } => {
                let json = serde_json::to_string_pretty(schema).map_err(|_| fmt::Error)?;
                writeln!(f, "{json}")
            }
            Self::Files { dir, schemas } => {
                let n = schemas.len();
                let p = dir.display();
                writeln!(f, "{n} JSON Schema(s) successfully generated in {p}:")?;

                for SchemaFile { cmd, path } in schemas {
                    let p = path.display();
                    writeln!(f, " - {cmd} JSON Schema at {p}")?;
                }

                Ok(())
            }
        }
    }
}

/// A schema file written.
#[derive(Serialize, JsonSchema)]
pub(crate) struct SchemaFile {
    /// The schema name.
    cmd: String,
    /// The path of the file.
    path: PathBuf,
}

#[cfg(test)]
mod tests {
    use clap::Command;
    use serde_json::json;

    use crate::{clap::commands::json_schema::*, printer::TestPrinter};

    fn registry(version: &'static str) -> JsonSchemaRegistry {
        JsonSchemaRegistry {
            bin: "test",
            version,
            schemas: BTreeMap::from_iter([
                ("test-a".into(), json!({ "type": "object" })),
                ("test-b".into(), json!({ "type": "array" })),
            ]),
            events: BTreeMap::new(),
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

        let schema: Value = serde_json::from_str(&printer.output).unwrap();
        assert_eq!(
            schema,
            json!({
                "$id": "https://pimalaya.org/schemas/test/v2/test-b.json",
                "type": "array",
            })
        );
    }

    #[test]
    fn events_are_named_and_stamped_apart() {
        let mut registry = registry("2.3.1");
        registry
            .events
            .insert("test-a".into(), json!({ "type": "object" }));

        let cmd = JsonSchemaCommand {
            cmds: vec!["test-a.events".into()],
            dir: None,
        };

        let mut printer = TestPrinter::default();
        cmd.execute(&mut printer, registry).unwrap();

        let schema: Value = serde_json::from_str(&printer.output).unwrap();
        assert_eq!(
            schema["$id"],
            "https://pimalaya.org/schemas/test/v2/test-a.events.json"
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

        registry
            .events
            .insert("test-a".into(), json!({ "type": "string" }));

        assert_eq!(
            registry.non_object_roots(),
            ["test-a.events", "test-any", "test-b"]
        );
    }

    #[test]
    fn the_toolkit_outputs_are_registered_and_objects() {
        let registry = JsonSchemaRegistry::new("test", "1.0.0");

        assert_eq!(
            registry.schemas.keys().collect::<Vec<_>>(),
            ["test-completion", "test-json-schema", "test-manual"]
        );
        assert!(registry.non_object_roots().is_empty());
    }

    fn tree() -> Command {
        Command::new("test")
            .subcommand_required(true)
            .subcommand(Command::new("a"))
            .subcommand(Command::new("b"))
            .subcommand(Command::new("repl"))
            .subcommand(
                Command::new("c")
                    .subcommand_required(true)
                    .subcommand(Command::new("d")),
            )
    }

    #[test]
    fn the_check_reports_undescribed_commands_and_dangling_keys() {
        let mut registry = registry("1.0.0");
        registry.schemas.insert("test-error".into(), json!({}));
        registry.schemas.insert("test-item-error".into(), json!({}));
        registry.schemas.insert("test-x".into(), json!({}));

        assert_eq!(
            registry.check(&tree(), &["test-repl"]),
            [
                "test-c-d: command without schema",
                "test-x: schema naming no command",
            ]
        );
    }

    #[test]
    fn the_check_reports_event_keys_out_of_place() {
        let mut registry = registry("1.0.0");
        registry.schemas.insert(
            "test-c-d".into(),
            json!({ "type": "object", "properties": { "event": {} } }),
        );
        registry.events.insert(
            "test-a".into(),
            json!({ "oneOf": [
                { "type": "object", "properties": { "event": {} }, "required": ["event"] },
                { "type": "object", "properties": { "kind": {} }, "required": ["kind"] },
            ] }),
        );

        assert_eq!(
            registry.check(&tree(), &["test-repl"]),
            [
                "test-c-d: result holding an event key",
                "test-a: event without an event key",
            ]
        );
    }

    #[test]
    fn a_command_with_events_only_needs_no_result_schema() {
        let mut registry = registry("1.0.0");
        registry.schemas.remove("test-a");
        registry.schemas.insert("test-c-d".into(), json!({}));
        registry.events.insert(
            "test-a".into(),
            json!({ "type": "object", "properties": { "event": {} }, "required": ["event"] }),
        );

        assert!(registry.check(&tree(), &["test-repl"]).is_empty());
    }
}
