---
cairn: tasks
change: a-refused-call-names-its-argument
---

# Tasks

- [x] src/mcp/grammar.rs: `argv` refuses a missing required argument by its property name
- [x] src/mcp/grammar.rs: `Grammar::usage_error`, naming the property of a missing argument or a refused value
- [x] Cargo.toml: clap's `error-context` under `mcp`
- [x] Tests: a missing argument from `argv` and from clap, a refused value by its parser and by its possible values, a conflict keeping clap's message, each a failed tool result coded `usage-invalid`; the argv tests giving the required arguments
- [x] CHANGELOG; fold into the spec; log; land
