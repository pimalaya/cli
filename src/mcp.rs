//! A Model Context Protocol server over stdio: the JSON-RPC answers every
//! server gives alike, and the tool results built from what a command
//! prints.
//!
//! A server reads one JSON-RPC message per line of stdin and writes one
//! per line of stdout, which carries protocol messages only. [`read`]
//! answers a line itself, the protocol errors, `initialize`, `ping` and
//! `tools/list` included, or hands the binary a checked [`ToolCall`]. The
//! binary runs it, usually a command printing into a [`CapturePrinter`],
//! and answers a [`ToolResult`] through [`ToolCall::respond`].
//!
//! A server with nothing else to wait for runs [`serve`], a blocking loop
//! over stdin. One with background events (a listener, a poll) reads
//! stdin on a thread with [`spawn_stdin_reader`], feeding its own
//! channel, and answers each line with [`answer`] and [`send`]. Either way
//! stdin is claimed ([`claim_stdin`]), so a command reading its input
//! there is refused, and [`exit_on_failure`] reports a failure on stderr.
//!
//! The [`grammar`] module derives a tool's input from a clap command.
//!
//! Refs: <https://modelcontextprotocol.io/specification/2025-06-18>
//!
//! [`CapturePrinter`]: crate::printer::CapturePrinter
//! [`claim_stdin`]: crate::session::claim_stdin

pub mod grammar;

use std::{
    io::{self, BufRead, Write},
    mem, process, str,
    sync::mpsc::Sender,
    thread::{self, JoinHandle},
};

use anyhow::{Context, Error, Result, anyhow};
use serde::Serialize;
use serde_json::{Map, Value, json};

use crate::{
    error::{EXIT_FAILURE, ErrorCode, ErrorReport, NoErrorCode, partial_outcome_of},
    printer::CapturePrinter,
    session::claim_stdin,
};

/// The MCP revisions the server speaks, latest first.
///
/// 2025-06-18 is the first carrying `structuredContent` and
/// `outputSchema`; 2025-03-26 asks for batches, which the server refuses.
pub const PROTOCOL_VERSIONS: &[&str] = &["2025-06-18"];

/// The JSON-RPC code of a message that is not JSON.
pub const PARSE_ERROR: i64 = -32700;

/// The JSON-RPC code of a message that is no valid request.
pub const INVALID_REQUEST: i64 = -32600;

/// The JSON-RPC code of an unknown method.
pub const METHOD_NOT_FOUND: i64 = -32601;

/// The JSON-RPC code of a request whose parameters are wrong.
pub const INVALID_PARAMS: i64 = -32602;

/// What a server tells the client at `initialize`.
#[derive(Clone, Copy, Debug)]
pub struct ServerInfo<'a> {
    /// The binary name.
    pub name: &'a str,
    /// The binary version, usually `env!("CARGO_PKG_VERSION")`.
    pub version: &'a str,
    /// How to use the tools, for the client to hand its model.
    pub instructions: Option<&'a str>,
}

/// What a line read off stdin asks of the server.
#[derive(Debug)]
pub enum Incoming {
    /// The answer to write back as it is.
    Reply(Value),
    /// A tool call to run, then answer with [`ToolCall::respond`].
    Call(ToolCall),
    /// Nothing to answer: a notification or a blank line.
    Ignore,
}

/// A `tools/call` request, checked: the tool is listed and the arguments
/// are an object whose keys the tool declares.
#[derive(Clone, Debug)]
pub struct ToolCall {
    /// The request id, to answer with.
    pub id: Value,
    /// The tool name.
    pub name: String,
    /// The tool arguments, empty when the call gave none.
    pub arguments: Map<String, Value>,
}

impl ToolCall {
    /// The JSON-RPC response carrying `result`.
    pub fn respond(&self, result: ToolResult) -> Value {
        json!({ "jsonrpc": "2.0", "id": self.id, "result": result })
    }
}

/// The result of a tool call.
///
/// A failure is a result with `isError` rather than a protocol error, so
/// the model reads it.
#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolResult {
    /// The content items, each `{type, ...}`.
    pub content: Vec<Value>,
    /// The structured result, an object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structured_content: Option<Value>,
    /// Whether the call failed.
    #[serde(skip_serializing_if = "is_false")]
    pub is_error: bool,
}

impl ToolResult {
    /// A success holding `value` as text, and as `structuredContent` when
    /// it is an object.
    pub fn success(value: Value) -> Self {
        Self {
            content: vec![text(value.to_string())],
            structured_content: value.is_object().then_some(value),
            is_error: false,
        }
    }

    /// A failure holding the failure object `report` as text.
    pub fn failure(report: Value) -> Self {
        Self {
            content: vec![text(report.to_string())],
            structured_content: None,
            is_error: true,
        }
    }

    /// A failure holding the failure object of `err`, with the code of
    /// its chain.
    pub fn error<C: ErrorCode>(err: Error) -> Self {
        let report = serde_json::to_value(ErrorReport::<C>::from(err))
            .unwrap_or_else(|err| Value::from(err.to_string()));

        Self::failure(report)
    }

    /// What a command printed into `printer`, given how it ended.
    ///
    /// The events come first, one text item each, then the single value
    /// the command printed. A partial outcome keeps that result and sets
    /// `isError`; a failure, or a count of values other than one, is a
    /// failure object.
    pub fn printed<C: ErrorCode>(mut printer: CapturePrinter, result: Result<()>) -> Self {
        let partial = match result {
            Ok(()) => false,
            Err(err) if partial_outcome_of(&err).is_some() => true,
            Err(err) => return Self::error::<C>(err),
        };

        let events = mem::take(&mut printer.events);

        let value = match printer.into_single() {
            Ok(value) => value,
            Err(err) => return Self::error::<C>(err),
        };

        let mut result = Self::success(value);
        let events = events.into_iter().map(|event| text(event.to_string()));
        result.content.splice(0..0, events);
        result.is_error = partial;
        result
    }
}

/// A line of stdin as a [`spawn_stdin_reader`] thread reads it.
#[derive(Debug)]
pub enum Input {
    /// One line, its newline included, as bytes.
    Line(Vec<u8>),
    /// stdin closed, or failed.
    Eof,
}

/// Answers one line read off stdin, or hands the tool call it holds to
/// the binary.
///
/// `tools` are the definitions `tools/list` answers. A message that is
/// not UTF-8 or not JSON answers -32700, one that is not a JSON object
/// (a batch included), whose `jsonrpc` is not `"2.0"`, whose `id` is not
/// a string or a number or that has no `method` answers -32600, an
/// unknown method -32601. A `tools/call` whose params are not an object,
/// whose tool is missing or not listed, or whose arguments are not an
/// object answers -32602; an argument its tool's `inputSchema` does not
/// declare, when it sets `additionalProperties: false`, answers a failed
/// tool result naming the declared ones.
pub fn read(line: &[u8], info: &ServerInfo, tools: &[Value]) -> Incoming {
    let Ok(line) = str::from_utf8(line) else {
        let message = "Parse error: the message is not UTF-8";
        return Incoming::Reply(error(Value::Null, PARSE_ERROR, message));
    };

    if line.trim().is_empty() {
        return Incoming::Ignore;
    }

    let message = match serde_json::from_str(line) {
        Ok(Value::Object(message)) => message,
        Ok(_) => {
            let message =
                "Invalid request: a message is one JSON object, batches are not supported";
            return Incoming::Reply(error(Value::Null, INVALID_REQUEST, message));
        }
        Err(err) => {
            let message = format!("Parse error: {err}");
            return Incoming::Reply(error(Value::Null, PARSE_ERROR, message));
        }
    };

    let id = match message.get("id") {
        None => None,
        Some(id @ (Value::String(_) | Value::Number(_))) => Some(id.clone()),
        Some(_) => {
            let message = "Invalid request: the id is not a string or a number";
            return Incoming::Reply(error(Value::Null, INVALID_REQUEST, message));
        }
    };

    let invalid = |message: &str| {
        let id = id.clone().unwrap_or(Value::Null);
        Incoming::Reply(error(id, INVALID_REQUEST, message))
    };

    if message.get("jsonrpc").and_then(Value::as_str) != Some("2.0") {
        return invalid("Invalid request: jsonrpc is not \"2.0\"");
    }

    // A response is invalid too: the server sends no request.
    let Some(method) = message.get("method").and_then(Value::as_str) else {
        return invalid("Invalid request: the method is missing or not a string");
    };

    let Some(id) = id else {
        return Incoming::Ignore;
    };

    let params = message.get("params");

    let result = match method {
        "initialize" => initialize(info, params),
        "ping" => json!({}),
        "tools/list" => json!({ "tools": tools }),
        "tools/call" => return call(id, params, tools),
        method => {
            let message = format!("Method not found: {method}");
            return Incoming::Reply(error(id, METHOD_NOT_FOUND, message));
        }
    };

    Incoming::Reply(json!({ "jsonrpc": "2.0", "id": id, "result": result }))
}

/// Answers one line read off stdin, running the tool call it holds with
/// `run`; `None` when nothing is to be answered.
pub fn answer(
    line: &[u8],
    info: &ServerInfo,
    tools: &[Value],
    run: impl FnOnce(&ToolCall) -> ToolResult,
) -> Option<Value> {
    match read(line, info, tools) {
        Incoming::Reply(reply) => Some(reply),
        Incoming::Call(call) => Some(call.respond(run(&call))),
        Incoming::Ignore => None,
    }
}

/// Writes one message as one line, flushed.
pub fn send(output: &mut impl Write, message: &Value) -> Result<()> {
    serde_json::to_writer(&mut *output, message).context("Cannot write a message to stdout")?;
    writeln!(output)?;
    output.flush()?;
    Ok(())
}

/// Serves the requests of stdin until EOF, answering each on stdout,
/// each tool call run with `run`.
///
/// It claims stdin first.
pub fn serve(
    info: &ServerInfo,
    tools: &[Value],
    run: impl FnMut(&ToolCall) -> ToolResult,
) -> Result<()> {
    claim_stdin();
    serve_on(&mut io::stdin().lock(), &mut io::stdout(), info, tools, run)
}

/// Reads stdin on a thread of its own, sending each line, then EOF, to
/// `events` as `wrap` turns them into the binary's event type.
///
/// It claims stdin first. The thread ends at EOF, on a read failure, or
/// once nobody receives.
pub fn spawn_stdin_reader<T: Send + 'static>(
    events: Sender<T>,
    wrap: impl Fn(Input) -> T + Send + 'static,
) -> JoinHandle<()> {
    claim_stdin();
    thread::spawn(move || read_lines(&mut io::stdin().lock(), &events, wrap))
}

/// Unwraps the result of a server, or prints its failure as text on
/// stderr and exits with status 1, or with the status of a partial
/// outcome.
///
/// stdout carries protocol messages only, so a server failing before its
/// loop (resolving its account) or in it reports here rather than
/// through the printer.
pub fn exit_on_failure<C: ErrorCode, T>(result: Result<T>) -> T {
    match result {
        Ok(value) => value,
        Err(err) => {
            if let Some(outcome) = partial_outcome_of(&err) {
                process::exit(outcome.status());
            }

            eprint!("{}", ErrorReport::<C>::from(err));
            process::exit(EXIT_FAILURE);
        }
    }
}

/// `schema` as a tool's `outputSchema`, when its root is typed `object`.
///
/// MCP takes an object root only: a reference, an `anyOf` or a `oneOf`
/// root is refused even when every alternative is an object, which the
/// registry's `non_object_roots` accepts as the shape of a printed value.
pub fn tool_output_schema(schema: &Value) -> Option<&Value> {
    (schema["type"] == "object").then_some(schema)
}

/// The message refusing an argument `key` that `owner` does not declare,
/// naming the `known` ones.
pub(crate) fn unknown_argument<'a>(
    key: &str,
    owner: &str,
    known: impl IntoIterator<Item = &'a str>,
) -> Error {
    let known: Vec<String> = known.into_iter().map(|name| format!("`{name}`")).collect();

    if known.is_empty() {
        anyhow!("Unknown argument `{key}`, {owner} takes none")
    } else {
        anyhow!(
            "Unknown argument `{key}`, {owner} takes {}",
            known.join(", ")
        )
    }
}

/// Serves the requests of `input` until EOF, answering each on `output`.
fn serve_on(
    input: &mut impl BufRead,
    output: &mut impl Write,
    info: &ServerInfo,
    tools: &[Value],
    mut run: impl FnMut(&ToolCall) -> ToolResult,
) -> Result<()> {
    let mut line = Vec::new();

    loop {
        line.clear();

        let read = input.read_until(b'\n', &mut line);
        if read.context("Cannot read a message from stdin")? == 0 {
            return Ok(());
        }

        if let Some(response) = answer(&line, info, tools, &mut run) {
            send(output, &response)?;
        }
    }
}

/// Sends each line of `input`, then EOF, to `events`.
fn read_lines<T>(input: &mut impl BufRead, events: &Sender<T>, wrap: impl Fn(Input) -> T) {
    loop {
        let mut line = Vec::new();

        match input.read_until(b'\n', &mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => {
                if events.send(wrap(Input::Line(line))).is_err() {
                    return;
                }
            }
        }
    }

    let _ = events.send(wrap(Input::Eof));
}

/// The answer to `initialize`: the client's version when the server
/// speaks it, its latest otherwise.
fn initialize(info: &ServerInfo, params: Option<&Value>) -> Value {
    let asked = params
        .and_then(|params| params.get("protocolVersion"))
        .and_then(Value::as_str);

    let version = asked
        .filter(|asked| PROTOCOL_VERSIONS.contains(asked))
        .unwrap_or(PROTOCOL_VERSIONS[0]);

    let mut result = json!({
        "protocolVersion": version,
        "capabilities": { "tools": { "listChanged": false } },
        "serverInfo": { "name": info.name, "version": info.version },
    });

    if let Some(instructions) = info.instructions {
        result["instructions"] = Value::from(instructions);
    }

    result
}

/// Checks a `tools/call` request against the listed `tools`.
fn call(id: Value, params: Option<&Value>, tools: &[Value]) -> Incoming {
    let refuse = |id, message: String| Incoming::Reply(error(id, INVALID_PARAMS, message));

    let Some(Value::Object(params)) = params else {
        return refuse(id, String::from("Invalid params: not an object"));
    };

    let Some(name) = params.get("name").and_then(Value::as_str) else {
        return refuse(id, String::from("Missing tool name"));
    };

    let Some(tool) = tools.iter().find(|tool| tool["name"] == name) else {
        return refuse(id, format!("Unknown tool: {name}"));
    };

    let arguments = match params.get("arguments") {
        None | Some(Value::Null) => Map::new(),
        Some(Value::Object(arguments)) => arguments.clone(),
        Some(_) => return refuse(id, String::from("Arguments must be an object")),
    };

    let schema = &tool["inputSchema"];

    if schema["additionalProperties"] == false {
        let properties = schema["properties"].as_object();
        let declared = |key: &String| properties.is_some_and(|known| known.contains_key(key));

        if let Some(key) = arguments.keys().find(|key| !declared(key)) {
            let known = properties.into_iter().flat_map(|known| known.keys());
            let err = unknown_argument(key, name, known.map(String::as_str));
            let result = ToolResult::error::<NoErrorCode>(err);
            return Incoming::Reply(json!({ "jsonrpc": "2.0", "id": id, "result": result }));
        }
    }

    Incoming::Call(ToolCall {
        id,
        name: name.to_owned(),
        arguments,
    })
}

/// A JSON-RPC error response.
fn error(id: Value, code: i64, message: impl Into<String>) -> Value {
    json!({ "jsonrpc": "2.0", "id": id, "error": { "code": code, "message": message.into() } })
}

/// A text content item.
fn text(text: String) -> Value {
    json!({ "type": "text", "text": text })
}

fn is_false(value: &bool) -> bool {
    !value
}

#[cfg(test)]
mod tests {
    use std::{fmt, sync::mpsc};

    use anyhow::{Error, anyhow};
    use schemars::JsonSchema;
    use serde::Serialize;
    use serde_json::{Value, json};

    use crate::{
        clap::commands::json_schema::output_schema,
        error::PartialOutcome,
        mcp::*,
        printer::{CapturePrinter, Printer},
    };

    const INFO: ServerInfo = ServerInfo {
        name: "app",
        version: "1.2.3",
        instructions: None,
    };

    fn tools() -> Vec<Value> {
        vec![
            json!({
                "name": "item_get",
                "inputSchema": {
                    "type": "object",
                    "properties": { "id": { "type": "string" }, "full": { "type": "boolean" } },
                    "required": ["id"],
                    "additionalProperties": false,
                },
            }),
            json!({
                "name": "item_list",
                "inputSchema": { "type": "object", "properties": {}, "additionalProperties": false },
            }),
            json!({ "name": "open", "inputSchema": { "type": "object" } }),
        ]
    }

    fn reply(line: &str) -> Value {
        match read(line.as_bytes(), &INFO, &tools()) {
            Incoming::Reply(reply) => reply,
            incoming => panic!("not a reply: {incoming:?}"),
        }
    }

    fn code(reply: &Value) -> i64 {
        reply["error"]["code"].as_i64().unwrap_or_default()
    }

    #[test]
    fn a_message_that_is_not_json_is_a_parse_error() {
        let reply = reply("{\"jsonrpc\": ");
        assert_eq!(code(&reply), PARSE_ERROR, "{reply}");
        assert_eq!(reply["id"], Value::Null);
        assert_eq!(reply["jsonrpc"], "2.0");
    }

    #[test]
    fn bytes_that_are_not_utf8_are_a_parse_error() {
        let Incoming::Reply(reply) = read(b"{\"id\": \"\xff\"}\n", &INFO, &[]) else {
            panic!("no reply");
        };

        assert_eq!(code(&reply), PARSE_ERROR, "{reply}");
        assert_eq!(reply["id"], Value::Null);
    }

    #[test]
    fn a_batch_or_a_scalar_is_an_invalid_request() {
        for line in [
            r#"[{"jsonrpc":"2.0","id":1,"method":"ping"}]"#,
            "3",
            "\"ping\"",
            "null",
        ] {
            let reply = reply(line);
            assert_eq!(code(&reply), INVALID_REQUEST, "{line}: {reply}");
            assert_eq!(reply["id"], Value::Null, "{line}");
        }
    }

    #[test]
    fn an_invalid_request_keeps_its_id_when_valid() {
        let wrong_version = reply(r#"{"jsonrpc":"1.0","id":7,"method":"ping"}"#);
        assert_eq!(code(&wrong_version), INVALID_REQUEST);
        assert_eq!(wrong_version["id"], 7);

        let no_version = reply(r#"{"id":"a","method":"ping"}"#);
        assert_eq!(code(&no_version), INVALID_REQUEST);
        assert_eq!(no_version["id"], "a");

        let response = reply(r#"{"jsonrpc":"2.0","id":8,"result":{}}"#);
        assert_eq!(code(&response), INVALID_REQUEST);
        assert_eq!(response["id"], 8);

        let bad_id = reply(r#"{"jsonrpc":"2.0","id":{},"method":"ping"}"#);
        assert_eq!(code(&bad_id), INVALID_REQUEST);
        assert_eq!(bad_id["id"], Value::Null);
    }

    #[test]
    fn a_notification_or_a_blank_line_gets_no_answer() {
        for line in [
            &br#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#[..],
            br#"{"jsonrpc":"2.0","method":"unknown"}"#,
            b"  \r\n",
        ] {
            assert!(matches!(read(line, &INFO, &[]), Incoming::Ignore));
        }
    }

    #[test]
    fn an_unknown_method_is_not_found() {
        let reply = reply(r#"{"jsonrpc":"2.0","id":1,"method":"resources/list"}"#);
        assert_eq!(code(&reply), METHOD_NOT_FOUND, "{reply}");
        assert_eq!(reply["id"], 1);
    }

    #[test]
    fn ping_answers_an_empty_object() {
        let reply = reply(r#"{"jsonrpc":"2.0","id":"p","method":"ping"}"#);
        assert_eq!(reply, json!({ "jsonrpc": "2.0", "id": "p", "result": {} }));
    }

    #[test]
    fn initialize_echoes_a_supported_version() {
        let line = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18"}}"#;
        let reply = reply(line);

        assert_eq!(
            reply["result"],
            json!({
                "protocolVersion": "2025-06-18",
                "capabilities": { "tools": { "listChanged": false } },
                "serverInfo": { "name": "app", "version": "1.2.3" },
            })
        );
    }

    #[test]
    fn initialize_answers_the_latest_version_to_an_unsupported_one() {
        let info = ServerInfo {
            instructions: Some("Call item_list first."),
            ..INFO
        };

        for params in [r#"{"protocolVersion":"2099-01-01"}"#, "{}", "null"] {
            let line =
                format!(r#"{{"jsonrpc":"2.0","id":1,"method":"initialize","params":{params}}}"#);
            let Incoming::Reply(reply) = read(line.as_bytes(), &info, &[]) else {
                panic!("no reply");
            };

            assert_eq!(reply["result"]["protocolVersion"], PROTOCOL_VERSIONS[0]);
            assert_eq!(reply["result"]["instructions"], "Call item_list first.");
        }
    }

    #[test]
    fn tools_list_answers_the_tools_given() {
        let reply = reply(r#"{"jsonrpc":"2.0","id":1,"method":"tools/list"}"#);
        assert_eq!(reply["result"]["tools"], Value::from(tools()));
    }

    #[test]
    fn a_call_with_bad_params_is_refused() {
        for params in [
            "",
            r#","params":3"#,
            r#","params":{}"#,
            r#","params":{"name":"item_put"}"#,
            r#","params":{"name":"item_get","arguments":[]}"#,
        ] {
            let line = format!(r#"{{"jsonrpc":"2.0","id":4,"method":"tools/call"{params}}}"#);
            let reply = reply(&line);
            assert_eq!(code(&reply), INVALID_PARAMS, "{line}: {reply}");
            assert_eq!(reply["id"], 4);
        }
    }

    #[test]
    fn a_checked_call_is_handed_over() {
        let line = r#"{"jsonrpc":"2.0","id":5,"method":"tools/call","params":{"name":"item_get","arguments":{"id":"x"}}}"#;
        let Incoming::Call(call) = read(line.as_bytes(), &INFO, &tools()) else {
            panic!("no call");
        };

        assert_eq!(call.id, 5);
        assert_eq!(call.name, "item_get");
        assert_eq!(Value::from(call.arguments.clone()), json!({ "id": "x" }));

        let line = r#"{"jsonrpc":"2.0","id":6,"method":"tools/call","params":{"name":"open","arguments":{"any":1}}}"#;
        assert!(matches!(
            read(line.as_bytes(), &INFO, &tools()),
            Incoming::Call(_)
        ));

        let line = r#"{"jsonrpc":"2.0","id":6,"method":"tools/call","params":{"name":"item_list","arguments":null}}"#;
        let Incoming::Call(call) = read(line.as_bytes(), &INFO, &tools()) else {
            panic!("no call");
        };
        assert!(call.arguments.is_empty());
    }

    #[test]
    fn an_undeclared_argument_is_a_failed_result_naming_the_declared_ones() {
        let line = r#"{"jsonrpc":"2.0","id":5,"method":"tools/call","params":{"name":"item_get","arguments":{"di":"x"}}}"#;
        let unknown = reply(line);
        let result = &unknown["result"];

        assert_eq!(unknown["id"], 5);
        assert_eq!(result["isError"], true, "{unknown}");

        let report: Value =
            serde_json::from_str(result["content"][0]["text"].as_str().unwrap()).unwrap();
        assert_eq!(
            report["error"],
            "Unknown argument `di`, item_get takes `full`, `id`"
        );

        let line = r#"{"jsonrpc":"2.0","id":5,"method":"tools/call","params":{"name":"item_list","arguments":{"x":1}}}"#;
        let none = reply(line);
        assert!(none.to_string().contains("item_list takes none"), "{none}");
    }

    #[derive(Serialize, JsonSchema)]
    struct Item {
        id: u32,
    }

    impl fmt::Display for Item {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            writeln!(f, "{}", self.id)
        }
    }

    fn printed(values: &[u32], events: &[&str], result: Result<(), Error>) -> Value {
        let mut printer = CapturePrinter::default();

        for event in events {
            printer.event(json!({ "event": event })).unwrap();
        }

        for id in values {
            printer.out(Item { id: *id }).unwrap();
        }

        serde_json::to_value(ToolResult::printed::<NoErrorCode>(printer, result)).unwrap()
    }

    #[test]
    fn a_printed_value_is_structured_after_its_events() {
        assert_eq!(
            printed(&[1], &["started"], Ok(())),
            json!({
                "content": [
                    { "type": "text", "text": r#"{"event":"started"}"# },
                    { "type": "text", "text": r#"{"id":1}"# },
                ],
                "structuredContent": { "id": 1 },
            })
        );
    }

    #[test]
    fn a_partial_outcome_keeps_its_result_as_an_error() {
        let result = printed(&[1], &[], Err(PartialOutcome::default().into()));

        assert_eq!(result["structuredContent"], json!({ "id": 1 }));
        assert_eq!(result["isError"], true);
    }

    #[test]
    fn a_failure_or_a_wrong_count_is_a_failure_object() {
        for (values, outcome) in [
            (&[][..], Ok(())),
            (&[1, 2][..], Ok(())),
            (&[1][..], Err(anyhow!("Boom"))),
        ] {
            let expected = match &outcome {
                Ok(()) => format!("The command printed {} values, one expected", values.len()),
                Err(err) => err.to_string(),
            };

            let result = printed(values, &["started"], outcome);
            assert_eq!(result["isError"], true, "{result}");
            assert!(result.get("structuredContent").is_none(), "{result}");
            assert_eq!(result["content"].as_array().unwrap().len(), 1, "{result}");

            let text = result["content"][0]["text"].as_str().unwrap();
            let report: Value = serde_json::from_str(text).unwrap();
            assert_eq!(report["error"], expected);
            assert_eq!(report["code"], Value::Null);
        }
    }

    #[test]
    fn a_value_that_is_not_an_object_is_text_alone() {
        let result = serde_json::to_value(ToolResult::success(json!([1]))).unwrap();
        assert_eq!(
            result,
            json!({ "content": [{ "type": "text", "text": "[1]" }] })
        );
    }

    #[test]
    fn an_output_schema_needs_an_object_root() {
        let object = json!({ "type": "object", "properties": {} });
        assert_eq!(tool_output_schema(&object), Some(&object));

        for schema in [
            json!({ "$ref": "#/$defs/A", "$defs": { "A": { "type": "object" } } }),
            json!({ "anyOf": [{ "type": "object" }, { "type": "object" }] }),
            json!({ "oneOf": [{ "type": "object" }] }),
            json!({ "type": "array" }),
            json!({ "type": ["object", "null"] }),
        ] {
            assert_eq!(tool_output_schema(&schema), None, "{schema}");
        }

        let schema = output_schema::<Item>();
        assert!(tool_output_schema(&schema).is_some(), "{schema}");
    }

    #[test]
    fn the_loop_answers_each_line_and_runs_each_call() {
        let input = concat!(
            r#"{"jsonrpc":"2.0","id":1,"method":"ping"}"#,
            "\n\n",
            r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
            "\n",
            r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"item_get","arguments":{"id":"x"}}}"#,
            "\n\u{0}\n",
            r#"{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"item_list"}}"#,
        );

        let mut output = Vec::new();
        let mut calls = Vec::new();

        serve_on(
            &mut input.as_bytes(),
            &mut output,
            &INFO,
            &tools(),
            |call| {
                calls.push(call.name.clone());
                ToolResult::success(json!({ "called": call.name }))
            },
        )
        .unwrap();

        let output = String::from_utf8(output).unwrap();
        let replies: Vec<Value> = output
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();

        assert_eq!(calls, ["item_get", "item_list"]);
        assert_eq!(replies.len(), 4, "{output}");
        assert_eq!(replies[0]["id"], 1);
        assert_eq!(
            replies[1]["result"]["structuredContent"]["called"],
            "item_get"
        );
        assert_eq!(code(&replies[2]), PARSE_ERROR);
        assert_eq!(replies[3]["id"], 3);
    }

    #[test]
    fn the_reader_sends_each_line_then_eof() {
        let (events, inbox) = mpsc::channel();
        read_lines(&mut &b"a\n\xff\nb"[..], &events, |input| input);
        drop(events);

        let lines: Vec<Option<Vec<u8>>> = inbox
            .iter()
            .map(|input| match input {
                Input::Line(line) => Some(line),
                Input::Eof => None,
            })
            .collect();

        assert_eq!(
            lines,
            [
                Some(b"a\n".to_vec()),
                Some(b"\xff\n".to_vec()),
                Some(b"b".to_vec()),
                None
            ]
        );
    }

    #[test]
    fn a_response_carries_the_call_id() {
        let call = ToolCall {
            id: json!("c1"),
            name: String::from("item_get"),
            arguments: Map::new(),
        };

        let response = call.respond(ToolResult::success(json!({ "id": 1 })));
        assert_eq!(response["jsonrpc"], "2.0");
        assert_eq!(response["id"], "c1");
        assert_eq!(response["result"]["structuredContent"], json!({ "id": 1 }));
    }
}
