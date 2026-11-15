---
cairn: change
id: tools-are-gated-by-what-they-do
status: landed
created: 2026-11-15
---

# Tools are gated by what they do

## Why

The CLI alignment round of 2026-11-15 (pimalaya/.github cairn/changes/cli-alignment-2026-11-15, task L2, finding F2, decision D2) found the MCP gates of the family apart: himalaya and calendula gate behind `--allow-write` and `--allow-send`, cardamum behind `--allow-write`, ortie behind its lone `--allow-revoke` while it lists three tools that write (`auth_get`, `auth_resume`, `token_refresh`) with no gate. `openWorldHint` meant "sends beyond the account" in himalaya and calendula, "reaches the network" in ortie, and was never set in cardamum. The owner ruled for the two gates everywhere and one meaning of `openWorldHint`, written here.

## What

Spec only, cairn/spec/mcp.md: without a gate flag only the tools that read are listed; `--allow-write` lists the tools that change the account's data, `--allow-send` the ones acting beyond the account; a binary may keep its own gate for a narrower destructive class (ortie's `--allow-revoke`); `readOnlyHint` and `openWorldHint` have one meaning each and no gate moves them by itself. Each binary aligns in its own task (ortie's O2).
