---
cairn: log
change: repl-and-mcp-polish
landed: 2026-11-11
---

# REPL and MCP polish

Three fixes Ortie and Himalaya asked for once both ran on the shared REPL and MCP server.

- src/error.rs: `ErrorReport` holds its text hint in a `pub(crate)` field `log_level_hint`, `Run with --log-level to enable more verbose logs` by default, still printed only while debug logs are off and never in JSON. src/repl.rs sets it to `Restart the session with --log-level to enable more verbose logs` in `answer`, the one path every failure of a line takes (a failed command, an unsplittable or non-UTF-8 line, a refused session flag, a piped parse error). The hint is set where the report is built rather than read off `session::in_session`, which the MCP server also sets and which only exists with `repl` or `mcp`. Dropping the hint was the other option; restarting with the flag is the advice a person in the session needs, so it stays, reworded.
- src/repl.rs: the line grammar gets `.bin_name(self.bin)` beside `.name(self.bin)`. clap's `_build_subcommand` prefixes a subcommand's usage name with its parent's `bin_name` only, which `no_binary_name` leaves unset, so `help echo` and `echo --help` rendered `Usage: echo [OPTIONS] <WORD>`; they now render `Usage: app echo [OPTIONS] <WORD>`.
- Cargo.toml: `[dev-dependencies]` clap with `std` and `usage`, so the crate's own tests render usage lines as the apps do (this crate's clap leaves `usage` off, and without it the usage line is empty).
- src/mcp.rs: the private `text` becomes `pub fn text(text: impl Into<String>) -> Value`, the `{"type": "text", "text": ...}` item, linked from `ToolResult::content`.

Not added: a generic `exit_on_failure_unless(result, keep)` for the apps' MCP startup special case. Both apps keep a short match whose guard (`ConfigMissingError` and `welcome::is_wanted`) names a pimalaya-config type this crate cannot name; the helper would move the guard into a closure and hide the exit behind an inverted name, a thin wrapper for two short call sites.

Two new tests, `a_failure_hints_at_restarting_the_session_for_logs` and `a_command_help_names_the_binary_in_its_usage`, fail with the old code (`Usage: echo [OPTIONS] <WORD>` twice, the `Run with` hint) and pass. Ortie's REPL help test now asserts `Usage: ortie token [OPTIONS] <COMMAND>` and `Usage: ortie token show [OPTIONS]`.

Spec: repl ("Each line answers through its own printer" modified, "Help names the binary" added), mcp ("A text item has one builder" added).
