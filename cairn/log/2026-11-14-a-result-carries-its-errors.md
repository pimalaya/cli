---
cairn: log
change: a-result-carries-its-errors
landed: 2026-11-14
---

# A result carries its errors

src/error.rs: `ItemError<C = NoErrorCode>`, a plain struct with public `id: Option<String>`, `code: Option<C>`, `error: String` and `sources: Vec<String>`, deriving `Serialize` and `JsonSchema` from that one declaration (every key required under the serialize contract, `id` and `code` nullable). `ItemError::new(id: Option<&str>, err: &Error)` takes the code through `code_of`, so only the binary's vocabulary codes an item (no `CommonCode`), and `sources` from the chain below the outermost message, as the failure object does. Its `Display` writes `<id>  <code>  <error>`, the code as its serialized string, `-` for an absent id or code, the id and message passed through `sanitize::text` so a server's newline or escape cannot break the line. No new exit status: a binary returns `PartialOutcome::default()` once a result with errors is printed.

Tests: the code and `sources` of a coded error under two contexts, the schema's required and nullable keys and its code enum, the `Display` line with and without id and code, a newline in the message kept on one line.

Spec: error ("An item error is the failure object of one item" added, intro names the item error), output ("Shared nouns keep one shape", "A partial outcome prints its result" and "Events precede the result" with its listing scenario).

MCP is unchanged: `mcp::ToolResult::printed` already keeps a partial outcome's result as `structuredContent` beside `isError: true`. Himalaya adopts `errors` first (its change of the same name), calendula and cardamum when they print several items that can fail one by one. The release is left to the owner.
