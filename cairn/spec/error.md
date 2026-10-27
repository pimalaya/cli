---
cairn: spec
capability: error
status: current
---

# Error

A failed command prints its error report to stdout, as text or JSON, and exits with status 1.

### Requirement: Failures carry an optional stable code
The error report SHALL print a `code` field beside `error`, `sources` and `backtrace` when the error chain contains a coded error of the binary's vocabulary, the outermost one winning, and SHALL omit the field otherwise. Code values SHALL be kebab-case. The code vocabulary SHALL belong to each binary; pimalaya-cli SHALL provide only the mechanism.

### Requirement: The failure object has a schema
The error report SHALL implement `JsonSchema`, describing the failure object with `code` optional and typed by the binary's code enum, from the same declaration its serialization uses.
