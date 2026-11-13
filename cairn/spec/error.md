---
cairn: spec
capability: error
status: current
---

# Error

A failed command prints its error report to stdout, as text or JSON, and exits with status 1. A command that ran to its end with an outcome short of full success prints its result instead and exits 3 or above. A command line the grammar refuses exits 2.

### Requirement: Failures carry an optional stable code
The error report SHALL print a `code` field beside `error`, `sources` and `backtrace`: the code of the outermost coded error in the chain, `null` when the chain holds none. Code values SHALL be kebab-case. The code vocabulary SHALL belong to each binary; pimalaya-cli SHALL provide only the mechanism, and the codes it raises itself, `usage-invalid` alone, which the failure object's schema admits beside the binary's. A binary SHALL have one failure object, and a failure folded into a result SHALL keep its code.

### Requirement: The failure object has a schema
The error report SHALL implement `JsonSchema`, describing the failure object with `code` required, nullable and typed by the binary's code enum, from the same declaration its serialization uses.
### Requirement: A refused command line exits 2
pimalaya-cli SHALL provide `clap::parse`, which a binary calls in place of clap's `Parser::parse`. Help and version SHALL print and exit as clap does. A command line the grammar refuses (an unknown flag or command, a missing argument, a bad value) SHALL exit 2; when a `--json` word comes before any `--`, it SHALL print on stdout the failure object coded `usage-invalid`, its `error` being clap's message without its `error: ` prefix or styles, and clap's rendering on stderr otherwise. pimalaya-cli SHALL provide `error::usage_error`, building that coded failure from a clap error, for every other place a binary turns a clap error into a failure.

#### Scenario: A mistyped flag under --json
- GIVEN a binary parsing its arguments with `clap::parse`
- WHEN it runs with `--json --frobnicate`
- THEN stdout holds one failure object with `"code":"usage-invalid"` and an `error` naming `--frobnicate`, and the exit code is 2
