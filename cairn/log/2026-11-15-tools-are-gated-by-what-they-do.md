---
cairn: log
change: tools-are-gated-by-what-they-do
landed: 2026-11-15
---

# Tools are gated by what they do

Task L2 of the CLI alignment round of 2026-11-15 (finding F2, decision D2). Spec only.

## What landed

- Without a gate flag a server lists only the tools that read; `--allow-write` adds the tools changing the account's data, `--allow-send` those acting beyond the account; a binary may keep a narrower gate of its own (ortie's `--allow-revoke`).
- `readOnlyHint` is true exactly when the tool as listed changes no data, `openWorldHint` exactly when it reaches other people, never for reaching the network; no gate moves a hint by itself.

## Capabilities moved

- [mcp](../spec/mcp.md): *Gates list what a tool may do* and *Annotations describe the tool as listed* added.
