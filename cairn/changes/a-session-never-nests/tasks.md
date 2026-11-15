---
cairn: tasks
change: a-session-never-nests
---

# Tasks

- [x] src/repl.rs: refuse a `repl` or `mcp` line, coded `usage-invalid`, before `execute`
- [x] src/repl.rs: code the session-flag, open-quote and non-UTF-8 refusals `usage-invalid`
- [x] src/repl.rs: the version the command tree carries, propagated to every command
- [x] Tests: the nested session by name, alias, prefix and with `--json`; its help still printed; the codes of the other refusals; the version a command enum carries in a text and a JSON session, and none without one
- [x] CHANGELOG; fold into the spec; log; land
