---
cairn: tasks
change: a-tool-skips-no-positional
---

# Tasks

- [x] src/mcp/grammar.rs: `Grammar::new` refuses a command set with `allow_missing_positional`
- [x] Tests: building the grammar of `list [CALENDAR] <ID>` set with `allow_missing_positional` panics (failed before the fix: it built, and its argv `-- c x` was refused by clap with `<ID>` given twice)
- [x] CHANGELOG; fold into the spec; log; land
