//! A tool's input derived from a clap command, and the argv a call turns
//! back into, so a tool and the command it runs share one grammar.
//!
//! Each argument of the command becomes one property of the tool's input
//! schema, named as the CLI spells it: the long flag, or the value name
//! of a positional. A call's arguments are written back as the words a
//! command line would carry and parsed by that same command, so what clap
//! accepts or refuses, the tool does too. A refusal names the argument as
//! the tool's input does, never clap's usage text.

use std::{any::TypeId, error::Error as _};

use anyhow::{Error, Result};
use clap::{
    Arg, ArgAction, Command, ValueHint,
    error::{ContextKind, ContextValue, ErrorKind},
};
use serde_json::{Map, Value, json};

use crate::{
    error::{usage_error, usage_invalid},
    mcp::unknown_argument,
};

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
    ///
    /// # Panics
    ///
    /// Panics when `command` sets `allow_missing_positional`: [`argv`]
    /// writes the positionals after `--`, where clap no longer skips a
    /// missing one, so such a tool would refuse or misplace its values.
    ///
    /// [`argv`]: Grammar::argv
    pub fn new(path: &[&str], command: &Command) -> Self {
        assert!(
            !command.is_allow_missing_positional_set(),
            "the tool `{}` cannot be built from a command set with \
             allow_missing_positional, its positionals follow `--`",
            path.join(" "),
        );

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
    /// making each a required scalar.
    pub fn one(mut self, names: &[&str]) -> Self {
        for arg in &mut self.args {
            if arg.multiple && names.contains(&arg.name.as_str()) {
                arg.one = true;
            }
        }

        self
    }

    /// Replaces the description of the argument named `name`, for a help
    /// written for the command line that does not hold for the tool.
    pub fn describe(mut self, name: &str, description: &str) -> Self {
        for arg in &mut self.args {
            if arg.name == name {
                arg.description = Some(description.to_owned());
            }
        }

        self
    }

    /// The names of the properties, in the order of the command.
    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.args.iter().map(|arg| arg.name.as_str())
    }

    /// The tool's `inputSchema`, refusing undeclared properties.
    ///
    /// A description naming the long flag of a property, as in `--send`
    /// between backquotes, names the property instead.
    pub fn input_schema(&self) -> Value {
        let mut properties = Map::new();
        let mut required = Vec::new();

        for arg in &self.args {
            let mut schema = arg.schema();

            if let Some(description) = arg.description.as_deref() {
                schema["description"] = Value::from(self.unflag(description));
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

    /// `description` with each backquoted long flag of a property written
    /// as the property's name.
    fn unflag(&self, description: &str) -> String {
        let mut description = description.to_owned();

        for long in self.args.iter().filter_map(|arg| arg.long.as_deref()) {
            description = description.replace(&format!("`--{long}`"), &format!("`{long}`"));
        }

        description
    }

    /// The argv of a call's `arguments`, for the root command to parse
    /// without a binary name.
    ///
    /// The command path comes first, then each option with its value
    /// glued as `--<long>=<value>` and each true flag as `--<long>`, then
    /// the fixed words, then `--` and the positionals in their order, so
    /// a value starting with a dash is never read as a flag. An undeclared
    /// key, a required argument missing or `null`, a value of the wrong
    /// type and a count other than one for an argument taking one are
    /// refused.
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
                    return Err(usage_invalid(format!(
                        "Argument `{name}` must be a boolean"
                    )));
                };

                if enabled && let Some(long) = &arg.long {
                    argv.push(format!("--{long}"));
                }

                continue;
            }

            // A lone value stands for an array of one.
            let values = match value {
                Value::Array(items) if arg.multiple => items.iter().map(scalar).collect(),
                value => scalar(value).map(|value| vec![value]),
            };

            let Some(values) = values else {
                let (one, many) = if arg.integer {
                    ("an integer", "integers")
                } else {
                    ("a string", "strings")
                };

                if arg.multiple && !arg.one {
                    return Err(usage_invalid(format!(
                        "Argument `{name}` must be {one} or an array of {many}"
                    )));
                }

                return Err(usage_invalid(format!("Argument `{name}` must be {one}")));
            };

            if arg.one && values.len() != 1 {
                return Err(usage_invalid(format!(
                    "Argument `{name}` takes exactly one value"
                )));
            }

            match (&arg.long, arg.index) {
                (Some(long), _) => {
                    argv.extend(values.iter().map(|value| format!("--{long}={value}")))
                }
                (None, Some(index)) => positionals.push((index, values)),
                (None, None) => (),
            }
        }

        // Once the values given are checked, the ones not given.
        let missing: Vec<&str> = self
            .args
            .iter()
            .filter(|arg| arg.required || arg.one)
            .filter(|arg| arguments.get(&arg.name).is_none_or(Value::is_null))
            .map(|arg| arg.name.as_str())
            .collect();

        if !missing.is_empty() {
            return Err(usage_invalid(missing_arguments(&missing)));
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

    /// The failure of a call whose argv clap refused, coded `usage-invalid`
    /// and naming the argument as the tool's input does, or clap's message
    /// as [`usage_error`] words it when it names no property.
    ///
    /// [`usage_error`]: crate::error::usage_error
    pub fn usage_error(&self, err: &clap::Error) -> Error {
        let property = |rendered: &str| {
            let rendered = rendered.trim_start_matches('[');

            let arg = match rendered.strip_prefix("--") {
                Some(flag) => {
                    let long = flag.split([' ', '=', '[', ']']).next();
                    self.args.iter().find(|arg| arg.long.as_deref() == long)
                }
                None => {
                    let value = rendered.trim_start_matches('<');
                    let value = value.split(['>', ']']).next()?;
                    let name = value.to_lowercase();
                    let mut positionals = self.args.iter().filter(|arg| arg.long.is_none());
                    positionals.find(|arg| arg.name == name)
                }
            };

            arg.map(|arg| arg.name.as_str())
        };

        let names: Option<Vec<&str>> = match err.get(ContextKind::InvalidArg) {
            Some(ContextValue::String(arg)) => property(arg).map(|name| vec![name]),
            Some(ContextValue::Strings(args)) => args.iter().map(|arg| property(arg)).collect(),
            _ => None,
        };

        let value = match err.get(ContextKind::InvalidValue) {
            Some(ContextValue::String(value)) => Some(value.as_str()),
            _ => None,
        };

        let invalid = |name: &str, value: &str, reason: Option<String>| match reason {
            Some(reason) => format!("Invalid value `{value}` for `{name}`: {reason}"),
            None => format!("Invalid value `{value}` for `{name}`"),
        };

        let message = match (err.kind(), names.as_deref(), value) {
            (ErrorKind::MissingRequiredArgument, Some(names @ [_, ..]), _) => {
                missing_arguments(names)
            }
            (ErrorKind::ValueValidation, Some([name]), Some(value)) => {
                invalid(name, value, err.source().map(ToString::to_string))
            }
            (ErrorKind::InvalidValue, Some([name]), Some(value)) => {
                let possible = match err.get(ContextKind::ValidValue) {
                    Some(ContextValue::Strings(values)) => values.as_slice(),
                    _ => &[],
                };
                let possible: Vec<String> = possible.iter().map(|v| format!("`{v}`")).collect();
                let reason = format!("the possible values are {}", possible.join(", "));
                invalid(name, value, (!possible.is_empty()).then_some(reason))
            }
            _ => return usage_error(err),
        };

        usage_invalid(message)
    }
}

/// The message refusing a call that misses the arguments `names`.
fn missing_arguments(names: &[&str]) -> String {
    let names: Vec<String> = names.iter().map(|name| format!("`{name}`")).collect();

    match names.as_slice() {
        [name] => format!("Missing argument {name}"),
        names => format!("Missing arguments {}", names.join(", ")),
    }
}

/// One argument of a [`Grammar`], and how it goes back into an argv.
#[derive(Clone, Debug)]
struct GrammarArg {
    name: String,
    long: Option<String>,
    index: Option<usize>,
    boolean: bool,
    integer: bool,
    multiple: bool,
    required: bool,
    one: bool,
    item: Value,
    description: Option<String>,
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
        let integer = parses_integer(arg);
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

        let mut item = match integer {
            true => json!({ "type": "integer" }),
            false => json!({ "type": "string" }),
        };

        if !enumerated.is_empty() {
            item["enum"] = Value::Array(enumerated);
        }

        Self {
            name,
            long: arg.get_long().map(ToString::to_string),
            index: arg.get_index(),
            boolean,
            integer,
            multiple,
            required: arg.is_required_set() || at_least_one,
            one: false,
            item,
            description: arg
                .get_long_help()
                .or(arg.get_help())
                .map(ToString::to_string),
        }
    }

    /// The property's schema without its description: a boolean, one
    /// item, or an array of items.
    fn schema(&self) -> Value {
        match (self.boolean, self.multiple && !self.one) {
            (true, _) => json!({ "type": "boolean" }),
            (false, true) => json!({ "type": "array", "items": self.item }),
            (false, false) => self.item.clone(),
        }
    }
}

/// Whether an argument parses its values as integers, the way
/// `value_parser!(u32)` and its ranges do.
fn parses_integer(arg: &Arg) -> bool {
    let id = arg.get_value_parser().type_id();

    [
        TypeId::of::<u8>(),
        TypeId::of::<u16>(),
        TypeId::of::<u32>(),
        TypeId::of::<u64>(),
        TypeId::of::<u128>(),
        TypeId::of::<usize>(),
        TypeId::of::<i8>(),
        TypeId::of::<i16>(),
        TypeId::of::<i32>(),
        TypeId::of::<i64>(),
        TypeId::of::<i128>(),
        TypeId::of::<isize>(),
    ]
    .into_iter()
    .any(|integer| id == integer)
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
    use clap::{Arg, ArgAction, Command, ValueHint, value_parser};
    use serde_json::{Map, Value, json};

    use crate::{
        error::NoErrorCode,
        mcp::{ToolResult, grammar::Grammar},
    };

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
            .arg(Arg::new("id").value_name("ID").num_args(1..))
            .arg(
                Arg::new("page")
                    .long("page")
                    .value_parser(value_parser!(u32).range(1..)),
            );

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
    fn an_argument_taking_one_value_is_a_required_scalar() {
        let tree = tree();
        let grammar = grammar(&tree).describe("attachment-id", "The attachment");
        let schema = grammar.input_schema();

        assert_eq!(
            schema["properties"]["attachment-id"],
            json!({ "type": "string", "description": "The attachment" })
        );
        assert!(
            schema["required"]
                .as_array()
                .unwrap()
                .contains(&json!("attachment-id"))
        );

        for value in [json!("3"), json!(3), json!(["3"])] {
            let argv = grammar
                .argv(&arguments(
                    json!({ "message-id": "m", "attachment-id": value }),
                ))
                .unwrap();
            assert_eq!(argv.last().unwrap(), "3");
        }
    }

    #[test]
    fn a_description_names_properties_rather_than_flags() {
        let mut tree = Command::new("app").subcommand(
            Command::new("send")
                .arg(
                    Arg::new("save")
                        .long("save")
                        .help("With `--copy`, from `--file` or `--hidden`"),
                )
                .arg(Arg::new("copy").long("copy").action(ArgAction::SetTrue))
                .arg(Arg::new("file").long("file").value_name("PATH"))
                .arg(Arg::new("hidden").long("hidden").hide(true)),
        );
        tree.build();

        let leaf = tree.find_subcommand("send").unwrap();
        let schema = Grammar::new(&["send"], leaf).input_schema();

        assert_eq!(
            schema["properties"]["save"]["description"],
            "With `copy`, from `--file` or `--hidden`"
        );
    }

    #[test]
    fn a_lone_value_stands_for_an_array_of_one() {
        let tree = tree();
        let grammar = grammar(&tree);

        let argv = grammar
            .argv(&arguments(
                json!({ "header": "a b", "message-id": "m", "attachment-id": "3" }),
            ))
            .unwrap();

        assert_eq!(
            argv,
            [
                "attachment",
                "download",
                "--header=a b",
                "--stdout",
                "--",
                "m",
                "3"
            ]
        );
    }

    #[test]
    fn an_integer_value_parser_is_an_integer() {
        let tree = tree();
        let leaf = tree
            .find_subcommand("flag")
            .and_then(|group| group.find_subcommand("add"))
            .unwrap();
        let grammar = Grammar::new(&["flag", "add"], leaf);

        assert_eq!(
            grammar.input_schema()["properties"]["page"],
            json!({ "type": "integer" })
        );

        let argv = grammar
            .argv(&arguments(json!({ "page": 2, "id": "1" })))
            .unwrap();
        assert_eq!(argv, ["flag", "add", "--page=2", "--", "1"]);
        assert!(tree.clone().try_get_matches_from(&argv).is_ok());

        let err = grammar
            .argv(&arguments(json!({ "page": [2] })))
            .unwrap_err();
        assert_eq!(err.to_string(), "Argument `page` must be an integer");
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

    /// After `--`, clap gives the first value to the last positional of
    /// such a command: `[CALENDAR] <ID>` called with both refuses `<ID>`
    /// given twice.
    #[test]
    #[should_panic(expected = "allow_missing_positional")]
    fn a_command_allowing_a_missing_positional_is_refused() {
        let mut list = Command::new("list")
            .allow_missing_positional(true)
            .arg(Arg::new("calendar").value_name("CALENDAR"))
            .arg(Arg::new("id").value_name("ID").required(true));
        list.build();

        Grammar::new(&["list"], &list);
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
            .argv(&arguments(
                json!({ "stdout": true, "mailbox": null, "message-id": "m" }),
            ))
            .unwrap();
        let off = grammar
            .argv(&arguments(json!({ "stdout": false, "message-id": "m" })))
            .unwrap();

        assert_eq!(on, ["attachment", "download", "--stdout", "--", "m"]);
        assert_eq!(off, ["attachment", "download", "--", "m"]);
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
                json!({ "header": {} }),
                "Argument `header` must be a string or an array of strings",
            ),
            (
                json!({ "header": [[1]] }),
                "Argument `header` must be a string or an array of strings",
            ),
            (
                json!({ "attachment-id": {} }),
                "Argument `attachment-id` must be a string",
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
        assert_eq!(
            crate::error::code_of(&err),
            Some(crate::error::CommonCode::UsageInvalid)
        );
    }

    /// The failure object a call failing with `err` answers, checking the
    /// result is an error.
    fn refusal(err: anyhow::Error) -> Value {
        let result = serde_json::to_value(ToolResult::error::<NoErrorCode>(err)).unwrap();
        assert_eq!(result["isError"], true, "{result}");

        let text = result["content"][0]["text"].as_str().unwrap();
        serde_json::from_str(text).unwrap()
    }

    #[test]
    fn a_missing_argument_is_named_as_the_input_names_it() {
        let tree = tree();
        let grammar = grammar(&tree);

        let err = grammar
            .argv(&arguments(json!({ "message-id": "m" })))
            .unwrap_err();
        let report = refusal(err);
        assert_eq!(report["error"], "Missing argument `attachment-id`");
        assert_eq!(report["code"], "usage-invalid");

        let err = grammar
            .argv(&arguments(json!({ "message-id": null })))
            .unwrap_err();
        assert_eq!(
            err.to_string(),
            "Missing arguments `message-id`, `attachment-id`"
        );

        // A refusal clap makes itself reads the same.
        let err = tree
            .clone()
            .try_get_matches_from(["attachment", "download"])
            .unwrap_err();
        let report = refusal(grammar.usage_error(&err));
        assert_eq!(report["error"], "Missing argument `message-id`");
        assert_eq!(report["code"], "usage-invalid");
    }

    #[test]
    fn a_refused_value_names_its_property_and_clap_reason() {
        let tree = tree();
        let leaf = tree
            .find_subcommand("flag")
            .and_then(|group| group.find_subcommand("add"))
            .unwrap();
        let add = Grammar::new(&["flag", "add"], leaf);
        let download = grammar(&tree);

        for (grammar, arguments_, message) in [
            (
                &add,
                json!({ "page": "x", "id": "1" }),
                "Invalid value `x` for `page`: invalid digit found in string",
            ),
            (
                &add,
                json!({ "page": 0, "id": "1" }),
                "Invalid value `0` for `page`: 0 is not in 1..=4294967295",
            ),
            (
                &download,
                json!({ "format": "pdf", "message-id": "m", "attachment-id": "1" }),
                "Invalid value `pdf` for `format`: the possible values are `plain`, `html`",
            ),
        ] {
            let argv = grammar.argv(&arguments(arguments_)).unwrap();
            let err = tree.clone().try_get_matches_from(&argv).unwrap_err();
            let report = refusal(grammar.usage_error(&err));

            assert_eq!(report["error"], message);
            assert_eq!(report["code"], "usage-invalid");
        }
    }

    #[test]
    fn a_refused_value_of_an_optional_positional_names_its_property() {
        let mut tree = Command::new("app").no_binary_name(true).subcommand(
            Command::new("get").arg(
                Arg::new("transport")
                    .value_name("TRANSPORT")
                    .value_parser(["socket", "tcp"]),
            ),
        );
        tree.build();

        let leaf = tree.find_subcommand("get").unwrap();
        let grammar = Grammar::new(&["get"], leaf);
        let argv = grammar
            .argv(&arguments(json!({ "transport": "udp" })))
            .unwrap();
        let err = tree.clone().try_get_matches_from(&argv).unwrap_err();
        let report = refusal(grammar.usage_error(&err));

        assert_eq!(
            report["error"],
            "Invalid value `udp` for `transport`: the possible values are `socket`, `tcp`"
        );
        assert_eq!(report["code"], "usage-invalid");
    }

    #[test]
    fn a_refusal_naming_no_property_keeps_clap_message() {
        let mut tree = Command::new("app").no_binary_name(true).subcommand(
            Command::new("send")
                .arg(
                    Arg::new("draft")
                        .long("draft")
                        .action(ArgAction::SetTrue)
                        .conflicts_with("now"),
                )
                .arg(Arg::new("now").long("now").action(ArgAction::SetTrue)),
        );
        tree.build();

        let leaf = tree.find_subcommand("send").unwrap();
        let grammar = Grammar::new(&["send"], leaf);
        let argv = grammar
            .argv(&arguments(json!({ "draft": true, "now": true })))
            .unwrap();
        let err = tree.clone().try_get_matches_from(&argv).unwrap_err();

        assert_eq!(
            grammar.usage_error(&err).to_string(),
            crate::error::usage_error(&err).to_string()
        );
        assert_eq!(refusal(grammar.usage_error(&err))["code"], "usage-invalid");
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
