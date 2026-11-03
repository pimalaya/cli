---
cairn: spec
capability: error
status: current
---

# Error

A failed command prints its error report to stdout, as text or JSON, and exits with status 1. A command that ran to its end with an outcome short of full success prints its result instead and exits 2 or above.

### Requirement: Failures carry an optional stable code
The error report SHALL print a `code` field beside `error`, `sources` and `backtrace`: the code of the outermost coded error in the chain, `null` when the chain holds none. Code values SHALL be kebab-case. The code vocabulary SHALL belong to each binary; pimalaya-cli SHALL provide only the mechanism. A binary SHALL have one failure object, and a failure folded into a result SHALL keep its code.

### Requirement: The failure object has a schema
The error report SHALL implement `JsonSchema`, describing the failure object with `code` required, nullable and typed by the binary's code enum, from the same declaration its serialization uses.
