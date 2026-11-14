---
cairn: change
id: a-result-carries-its-errors
status: active
created: 2026-11-14
---

# A result carries its errors

## Why

A command acting on several items has two ways to end today: the failure object, exit 1, when nothing could be done, or its result, exit 0 or 3. When some items fail and the others do not, the binaries have no shape for it: Himalaya drops the failed items from a listing with a warning on stderr ([himalaya#801], [#641], [#649], [#707]), skips an unknown id of a write with a warning, or stops a write half done on its first failed item. The only rule pimalaya-cli gives, "A failed item SHALL be an event field, never a failure object" (spec output), fits a command that streams events, not a listing or a write printing one result.

JMAP shows the shape every backend can fill: the data, and beside it what failed, per item, typed (RFC 8620 §5.1 `notFound`, §5.3 `notUpdated`, `notCreated` and `notDestroyed`). The Himalaya change of the same name (pimalaya/himalaya, cairn/changes/a-result-carries-its-errors) checked each backend against its protocol, and every one can report a failed item by its id, with a precision ranging from JMAP's typed reasons to IMAP's "asked for, not answered". The shape belongs here, so the family's binaries share it the way they share the failure object.

## What

- `error::ItemError<C>`, the error of one item: `{id, code, error, sources}`. It is the failure object without `backtrace`, plus `id`: the item's id as the command prints ids, `null` when the answer named none. `code` is the binary's own code or `null`, `error` the message, `sources` the chain below it, outermost first, as the failure object reads them. It is built from an `anyhow::Error` and an id: `ItemError::new(id, &err)` finds the code anywhere in the chain (`code_of`), as `ErrorReport::eval_coded` does.
- `ItemError` implements `JsonSchema` from the same declaration its serialization uses: every key required, `id`, `code` nullable, `code` typed by the binary's code enum.
- Its `Display` is one line, `<id>  <code>  <error>`, an absent id or code written `-`, so a binary places it in its own text layout.
- The output contract:
  - A command acting on several items SHALL print the failures of single items as an `errors` list of item errors in its result, `[]` when none failed. A command streaming events MAY report them as event fields instead. Neither is ever a failure object.
  - A result whose `errors` is not empty SHALL exit 3, through the existing `PartialOutcome`.
  - A failure leaving nothing attempted stays the failure object, exit 1.
  - The item error joins the shared nouns.
- MCP: `mcp::ToolResult::printed` already keeps the result of a partial outcome as `structuredContent` beside `isError: true`, so an agent reads both the items and their `errors`. Nothing changes there.
- Calendula and cardamum adopt the list when they print several items that can fail one by one. Himalaya adopts it first.

## Scope / non-goals

- Each binary's code vocabulary stays its own. pimalaya-cli adds no code: an item error's `code` is the binary's, `null` when it has none to give.
- The text layout of the errors is the binary's: pimalaya-cli gives the line, the binary places it.

## Decisions

- **`errors`, not `failures`.** The key the maintainer chose, 2026-11-14.
- **No `backtrace` in an item error.** It belongs to the command's failure, not to an item, and would be `null` in every entry.
- **Exit 3 stays one status.** A result with errors exits 3, like a failed `check` and a refused protocol completion. A script tells the two outcomes apart by the root of the printed object, `code` for the failure object, the command's keys for a result.

[himalaya#801]: https://github.com/pimalaya/himalaya/issues/801
[#641]: https://github.com/pimalaya/himalaya/issues/641
[#649]: https://github.com/pimalaya/himalaya/issues/649
[#707]: https://github.com/pimalaya/himalaya/issues/707
