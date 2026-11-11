//! A tool's input derived from a clap command, and the argv a call turns
//! back into, so a tool and the command it runs share one grammar.
//!
//! Each argument of the command becomes one property of the tool's input
//! schema, named as the CLI spells it: the long flag, or the value name
//! of a positional. A call's arguments are written back as the words a
//! command line would carry and parsed by that same command, so what clap
//! accepts or refuses, the tool does too.

use anyhow::{Result, bail};
use clap::{Arg, ArgAction, Command, ValueHint};
use serde_json::{Map, Value, json};

use crate::mcp::unknown_argument;

/// A clap command as a tool's input.
#[derive(Clone, Debug)]
pub struct Grammar {
    path: Vec<String>,
    args: Vec<GrammarArg>,
    fixed: Vec<String>,
}

impl Grammar {
    /// The grammar of `command`, reached from the root of its tree by the
    /// words of `path`, as in `["envelope", "list"]`.
    ///
    /// `command` comes from a built tree ([`Command::build`]), so its
    /// arguments carry their action, value count and index. Global,
    /// hidden, help and version arguments are left out, and so are those
    /// taking a path (a path value hint or a `PATH` value name): a tool
    /// reads and writes no local file.
    pub fn new(path: &[&str], command: &Command) -> Self {
        let args = command
            .get_arguments()
            .filter(|arg| exposes(arg))
            .map(GrammarArg::new)
            .collect();

        Self {
            path: path.iter().map(ToString::to_string).collect(),
            args,
            fixed: Vec::new(),
        }
    }

    /// Keeps the arguments whose name `keep` accepts, for the gates of the
    /// binary.
    pub fn retain(mut self, mut keep: impl FnMut(&str) -> bool) -> Self {
        self.args.retain(|arg| keep(&arg.name));
        self
    }

    /// Adds words every argv carries before its positionals, as in
    /// `--stdout`: the command runs as with them typed, and the arguments
    /// they set, matched by their long flag, are no properties, so a call
    /// cannot unset them.
    pub fn fixed(mut self, words: &[&str]) -> Self {
        let fixes = |long: &str| {
            words.iter().any(|word| {
                word.strip_prefix("--")
                    .and_then(|word| word.split('=').next())
                    == Some(long)
            })
        };

        self.args
            .retain(|arg| !arg.long.as_deref().is_some_and(fixes));
        self.fixed.extend(words.iter().map(ToString::to_string));
        self
    }

    /// Takes exactly one value of the repeatable arguments named `names`,
    /// making each a required array of one item.
    pub fn one(mut self, names: &[&str]) -> Self {
        for arg in &mut self.args {
            if arg.multiple && names.contains(&arg.name.as_str()) {
                arg.one = true;
            }
        }

        self
    }

    /// The names of the properties, in the order of the command.
    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.args.iter().map(|arg| arg.name.as_str())
    }

    /// The tool's `inputSchema`, refusing undeclared properties.
    pub fn input_schema(&self) -> Value {
        let mut properties = Map::new();
        let mut required = Vec::new();

        for arg in &self.args {
            let mut schema = arg.schema.clone();

            if arg.one {
                schema["minItems"] = Value::from(1);
                schema["maxItems"] = Value::from(1);
            }

            if arg.required || arg.one {
                required.push(Value::from(arg.name.as_str()));
            }

            properties.insert(arg.name.clone(), schema);
        }

        json!({
            "type": "object",
            "properties": properties,
            "required": required,
            "additionalProperties": false,
        })
    }

    /// The argv of a call's `arguments`, for the root command to parse
    /// without a binary name.
    ///
    /// The command path comes first, then each option with its value
    /// glued as `--<long>=<value>` and each true flag as `--<long>`, then
    /// the fixed words, then `--` and the positionals in their order, so
    /// a value starting with a dash is never read as a flag. An undeclared
    /// key, a value of the wrong type and a count other than one for an
    /// argument taking one are refused.
    pub fn argv(&self, arguments: &Map<String, Value>) -> Result<Vec<String>> {
        let declared = |key: &String| self.args.iter().any(|arg| &arg.name == key);

        if let Some(key) = arguments.keys().find(|key| !declared(key)) {
            return Err(unknown_argument(key, &self.path.join(" "), self.names()));
        }

        let mut argv = self.path.clone();
        let mut positionals = Vec::new();

        for arg in &self.args {
            let Some(value) = arguments.get(&arg.name).filter(|value| !value.is_null()) else {
                continue;
            };

            let name = &arg.name;

            if arg.boolean {
                let Some(enabled) = value.as_bool() else {
                    bail!("Argument `{name}` must be a boolean");
                };

                if enabled && let Some(long) = &arg.long {
                    argv.push(format!("--{long}"));
                }

                continue;
            }

            let values = match value {
                Value::Array(items) if arg.multiple => items.iter().map(scalar).collect(),
                value if !arg.multiple => scalar(value).map(|value| vec![value]),
                _ => None,
            };

            let Some(values) = values else {
                let expected = if arg.multiple {
                    "an array of strings"
                } else {
                    "a string"
                };

                bail!("Argument `{name}` must be {expected}");
            };

            if arg.one && values.len() != 1 {
                bail!("Argument `{name}` takes exactly one value");
            }

            match (&arg.long, arg.index) {
                (Some(long), _) => {
                    argv.extend(values.iter().map(|value| format!("--{long}={value}")))
                }
                (None, Some(index)) => positionals.push((index, values)),
                (None, None) => (),
            }
        }

        // Before the `--`, after which a flag would be a value.
        argv.extend(self.fixed.iter().cloned());

        if !positionals.is_empty() {
            positionals.sort_by_key(|(index, _)| *index);
            argv.push(String::from("--"));
            argv.extend(positionals.into_iter().flat_map(|(_, values)| values));
        }

        Ok(argv)
    }
}

/// One argument of a [`Grammar`], and how it goes back into an argv.
#[derive(Clone, Debug)]
struct GrammarArg {
    name: String,
    long: Option<String>,
    index: Option<usize>,
    boolean: bool,
    multiple: bool,
    required: bool,
    one: bool,
    schema: Value,
}

impl GrammarArg {
    fn new(arg: &Arg) -> Self {
        // Named as the CLI spells it, clap's id only for a nameless value.
        let name = match arg.get_long() {
            Some(long) => long.to_owned(),
            None => arg
                .get_value_names()
                .and_then(|names| names.first())
                .map(|name| name.as_str().to_lowercase())
                .unwrap_or_else(|| arg.get_id().to_string()),
        };

        let boolean = matches!(arg.get_action(), ArgAction::SetTrue | ArgAction::Count);
        let num_args = arg.get_num_args();
        let multiple = matches!(arg.get_action(), ArgAction::Append)
            || num_args.is_some_and(|n| n.max_values() > 1);

        // A repeatable positional needing a value is required unmarked.
        let at_least_one =
            arg.is_positional() && multiple && num_args.is_some_and(|n| n.min_values() > 0);

        let enumerated: Vec<Value> = arg
            .get_possible_values()
            .iter()
            .filter(|value| !value.is_hide_set())
            .map(|value| Value::from(value.get_name()))
            .collect();

        let mut item = json!({ "type": "string" });

        if !enumerated.is_empty() {
            item["enum"] = Value::Array(enumerated);
        }

        let mut schema = match (boolean, multiple) {
            (true, _) => json!({ "type": "boolean" }),
            (false, true) => json!({ "type": "array", "items": item }),
            (false, false) => item,
        };

        if let Some(help) = arg.get_long_help().or(arg.get_help()) {
            schema["description"] = Value::from(help.to_string());
        }

        Self {
            name,
            long: arg.get_long().map(ToString::to_string),
            index: arg.get_index(),
            boolean,
            multiple,
            required: arg.is_required_set() || at_least_one,
            one: false,
            schema,
        }
    }
}

/// Whether an argument becomes a property: not global, not help or
/// version, not hidden, reachable by a long flag or a position, and not
/// a path.
fn exposes(arg: &Arg) -> bool {
    let builtin = matches!(
        arg.get_action(),
        ArgAction::Help | ArgAction::HelpShort | ArgAction::HelpLong | ArgAction::Version
    );

    let path = matches!(
        arg.get_value_hint(),
        ValueHint::FilePath | ValueHint::DirPath | ValueHint::AnyPath | ValueHint::ExecutablePath
    ) || arg
        .get_value_names()
        .is_some_and(|names| names.iter().any(|name| name.as_str() == "PATH"));

    let reachable = arg.get_long().is_some() || arg.is_positional();

    !builtin && !path && reachable && !arg.is_global_set() && !arg.is_hide_set()
}

/// A JSON scalar as the word an argv carries.
fn scalar(value: &Value) -> Option<String> {
    match value {
        Value::String(value) => Some(value.clone()),
        Value::Number(value) => Some(value.to_string()),
        Value::Bool(value) => Some(value.to_string()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use clap::{Arg, ArgAction, Command, ValueHint};
    use serde_json::{Map, Value, json};

    use crate::mcp::grammar::Grammar;

    /// A tree whose `attachment download` mirrors a real one: an option, a
    /// flag the tool fixes, a path, a positional and a repeatable one.
    fn tree() -> Command {
        let download = Command::new("download")
            .about("Download attachments")
            .arg(
                Arg::new("mailbox")
                    .long("mailbox")
                    .help("The mailbox holding the message"),
            )
            .arg(Arg::new("stdout").long("stdout").action(ArgAction::SetTrue))
            .arg(Arg::new("dir").long("dir").value_hint(ValueHint::DirPath))
            .arg(Arg::new("out").long("out").value_name("PATH"))
            .arg(
                Arg::new("format")
                    .long("format")
                    .value_parser(["plain", "html"]),
            )
            .arg(Arg::new("header").long("header").action(ArgAction::Append))
            .arg(Arg::new("secret").long("secret").hide(true))
            .arg(
                Arg::new("message_id")
                    .value_name("MESSAGE-ID")
                    .required(true),
            )
            .arg(
                Arg::new("attachment_id")
                    .value_name("ATTACHMENT-ID")
                    .num_args(0..),
            );

        let add = Command::new("add")
            .arg(Arg::new("target").value_name("TARGET"))
            .arg(Arg::new("id").value_name("ID").num_args(1..));

        let mut tree = Command::new("app")
            .no_binary_name(true)
            .arg(
                Arg::new("verbose")
                    .long("verbose")
                    .global(true)
                    .action(ArgAction::SetTrue),
            )
            .subcommand(Command::new("attachment").subcommand(download))
            .subcommand(Command::new("flag").subcommand(add));

        tree.build();
        tree
    }

    fn grammar(tree: &Command) -> Grammar {
        let leaf = tree
            .find_subcommand("attachment")
            .and_then(|group| group.find_subcommand("download"))
            .unwrap();

        Grammar::new(&["attachment", "download"], leaf)
            .fixed(&["--stdout"])
            .one(&["attachment-id"])
    }

    fn arguments(value: Value) -> Map<String, Value> {
        let Value::Object(arguments) = value else {
            panic!("not an object");
        };

        arguments
    }

    #[test]
    fn the_schema_names_the_arguments_as_the_cli_spells_them() {
        let schema = grammar(&tree()).input_schema();
        let properties = schema["properties"].as_object().unwrap();

        assert_eq!(
            properties.keys().collect::<Vec<_>>(),
            ["attachment-id", "format", "header", "mailbox", "message-id"]
        );

        assert_eq!(
            properties["mailbox"],
            json!({ "type": "string", "description": "The mailbox holding the message" })
        );
        assert_eq!(
            properties["format"],
            json!({ "type": "string", "enum": ["plain", "html"] })
        );
        assert_eq!(
            properties["header"],
            json!({ "type": "array", "items": { "type": "string" } })
        );
        assert_eq!(schema["additionalProperties"], false);
        assert_eq!(schema["required"], json!(["message-id", "attachment-id"]));
    }

    #[test]
    fn an_argument_taking_one_value_is_a_required_array_of_one() {
        let schema = grammar(&tree()).input_schema();
        let property = &schema["properties"]["attachment-id"];

        assert_eq!(property["type"], "array");
        assert_eq!(property["minItems"], 1);
        assert_eq!(property["maxItems"], 1);
        assert!(
            schema["required"]
                .as_array()
                .unwrap()
                .contains(&json!("attachment-id"))
        );
    }

    #[test]
    fn a_fixed_argument_goes_before_the_positionals_and_is_no_property() {
        let tree = tree();
        let grammar = grammar(&tree);

        let argv = grammar
            .argv(&arguments(json!({
                "mailbox": "-odd",
                "message-id": "m2",
                "attachment-id": ["3"],
            })))
            .unwrap();

        assert_eq!(
            argv,
            [
                "attachment",
                "download",
                "--mailbox=-odd",
                "--stdout",
                "--",
                "m2",
                "3"
            ]
        );

        let properties = &grammar.input_schema()["properties"];
        assert!(properties.get("stdout").is_none(), "{properties}");

        let err = grammar
            .argv(&arguments(json!({ "stdout": true })))
            .unwrap_err()
            .to_string();
        assert_eq!(
            err,
            "Unknown argument `stdout`, attachment download takes `mailbox`, `format`, `header`, `message-id`, `attachment-id`"
        );
    }

    #[test]
    fn the_argv_is_parsed_back_by_the_same_command() {
        let tree = tree();
        let grammar = grammar(&tree);

        let argv = grammar
            .argv(&arguments(json!({
                "mailbox": "INBOX",
                "header": ["a", "-b"],
                "format": "html",
                "message-id": "-m",
                "attachment-id": [3],
            })))
            .unwrap();

        let matches = tree.clone().try_get_matches_from(&argv).unwrap();
        let (_, group) = matches.subcommand().unwrap();
        let (_, leaf) = group.subcommand().unwrap();
        let strings = |id: &str| -> Vec<&String> { leaf.get_many::<String>(id).unwrap().collect() };

        assert_eq!(strings("mailbox"), ["INBOX"]);
        assert_eq!(strings("header"), ["a", "-b"]);
        assert_eq!(strings("format"), ["html"]);
        assert_eq!(strings("message_id"), ["-m"]);
        assert_eq!(strings("attachment_id"), ["3"]);
        assert!(leaf.get_flag("stdout"));
    }

    #[test]
    fn a_repeatable_positional_needing_a_value_is_required() {
        let tree = tree();
        let leaf = tree
            .find_subcommand("flag")
            .and_then(|group| group.find_subcommand("add"))
            .unwrap();
        let grammar = Grammar::new(&["flag", "add"], leaf);
        let schema = grammar.input_schema();

        assert_eq!(schema["required"], json!(["id"]));
        assert_eq!(schema["properties"]["target"]["type"], "string");

        let argv = grammar
            .argv(&arguments(json!({ "id": ["1", "2"], "target": "t" })))
            .unwrap();
        assert_eq!(argv, ["flag", "add", "--", "t", "1", "2"]);
        assert!(tree.clone().try_get_matches_from(&argv).is_ok());
    }

    #[test]
    fn a_flag_is_a_boolean_written_only_when_true() {
        let tree = tree();
        let leaf = tree
            .find_subcommand("attachment")
            .and_then(|group| group.find_subcommand("download"))
            .unwrap();
        let grammar = Grammar::new(&["attachment", "download"], leaf);

        assert_eq!(
            grammar.input_schema()["properties"]["stdout"],
            json!({ "type": "boolean" })
        );

        let on = grammar
            .argv(&arguments(json!({ "stdout": true, "mailbox": null })))
            .unwrap();
        let off = grammar
            .argv(&arguments(json!({ "stdout": false })))
            .unwrap();

        assert_eq!(on, ["attachment", "download", "--stdout"]);
        assert_eq!(off, ["attachment", "download"]);
    }

    #[test]
    fn a_value_of_the_wrong_shape_is_refused() {
        let tree = tree();
        let grammar = grammar(&tree);

        for (arguments_, message) in [
            (
                json!({ "mailbox": ["a"] }),
                "Argument `mailbox` must be a string",
            ),
            (
                json!({ "mailbox": {} }),
                "Argument `mailbox` must be a string",
            ),
            (
                json!({ "header": "a" }),
                "Argument `header` must be an array of strings",
            ),
            (
                json!({ "header": [[1]] }),
                "Argument `header` must be an array of strings",
            ),
            (
                json!({ "attachment-id": ["1", "2"] }),
                "Argument `attachment-id` takes exactly one value",
            ),
            (
                json!({ "attachment-id": [] }),
                "Argument `attachment-id` takes exactly one value",
            ),
        ] {
            let err = grammar.argv(&arguments(arguments_)).unwrap_err();
            assert_eq!(err.to_string(), message);
        }

        let leaf = tree
            .find_subcommand("attachment")
            .and_then(|group| group.find_subcommand("download"))
            .unwrap();
        let err = Grammar::new(&["attachment", "download"], leaf)
            .argv(&arguments(json!({ "stdout": "yes" })))
            .unwrap_err();
        assert_eq!(err.to_string(), "Argument `stdout` must be a boolean");
    }

    #[test]
    fn retain_drops_the_arguments_a_gate_refuses() {
        let tree = tree();
        let grammar = grammar(&tree).retain(|name| name != "header");

        assert!(grammar.names().all(|name| name != "header"));

        let err = grammar
            .argv(&arguments(json!({ "header": ["a"] })))
            .unwrap_err();
        assert!(err.to_string().starts_with("Unknown argument `header`"));
    }
}
