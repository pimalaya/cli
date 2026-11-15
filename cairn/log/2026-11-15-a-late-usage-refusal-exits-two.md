---
cairn: log
change: a-late-usage-refusal-exits-two
landed: 2026-11-15
---

# A late usage refusal exits 2

tcard's `apply - -` (both TEMPLATE and SOURCE on stdin) is refused after clap parsed the line, with `error::usage_error`, so it printed the failure object coded `usage-invalid` and exited 1, where every refusal clap makes itself exits 2. tcal's twin, himalaya's and the other apps' MCP grammar refusals, and comodoro's `json-schema` refusal take the same path.

## What landed

- `error::failure_status`: 2 when the chain carries `CommonCode::UsageInvalid`, 1 otherwise. `ErrorReport::eval_coded` and `mcp::exit_on_failure` exit with it.
- `a_refused_command_line_exits_two`, the usage error wrapped in a context as a command returns it.

## Capabilities moved

- [error](../spec/error.md): *A refused command line exits 2* covers a refusal made after clap.
