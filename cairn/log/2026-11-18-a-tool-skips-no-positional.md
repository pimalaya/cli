---
cairn: log
change: a-tool-skips-no-positional
landed: 2026-11-18
---

# A tool skips no positional

Finding KIT-5 of the CLI family analysis of 2026-11-15, its grammar half.

## What landed

- `Grammar::new` (src/mcp/grammar.rs) panics on a command set with `allow_missing_positional`, the message naming the tool and the setting; its rustdoc gains a `# Panics` section. Checked first that the grammar misbehaves there: for `list [CALENDAR] <ID>`, the argv `-- c x` is refused by clap with `<ID>` given twice, while `-- x` alone parses.
- `a_command_allowing_a_missing_positional_is_refused`, failing before the fix (the grammar built).

No binary of the family builds such a command since calendula 14bbcc3 (ruling D28). Still open from KIT-5, in himalaya: `no-save` listed on its three composers under `--allow-write` while `send` is gated out.

## Capabilities moved

- [mcp](../spec/mcp.md): *A tool's command skips no positional* added.
