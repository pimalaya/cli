---
cairn: log
change: machine-output-contract
landed: 2026-10-27
---

# One versioned machine contract for every binary

pimalaya-cli now carries what the REPL and MCP work needs from every binary, unreleased (0.3.0 pending).

Error codes moved in from the three identical copies in himalaya, calendula and cardamum, generic over the binary's vocabulary: `ErrorCode` (a bound alias), `CodedError<C>`, `code_of`, and `ErrorReport<C = NoErrorCode>`. `ErrorReport::eval` stays source-compatible for the ten binaries without codes; coded binaries call `ErrorReport::<Code>::eval_coded`. One generic `eval` was rejected because a default type parameter does not drive inference, so every uncoded caller would have needed a turbofish. Serialization and the new `JsonSchema` impl both go through one private `ErrorReportOutput`, so the failure schema cannot drift from what is printed. Its keys are unchanged (`code`, `error`, `sources`, `backtrace`).

Piped `--json` is newline-delimited: the compact branch of `StdoutPrinter` writes a newline and flushes after each value, closing the gap ortie's REPL worked around with its own newline.

`JsonSchemaCommand::execute` takes a `JsonSchemaRegistry { bin, version, schemas }` and stamps each schema's `$id` with the contract version (`v<major>`, `v0.<minor>` before 1.0). `non_object_roots` lists the keys whose root is not an object, following `$ref`, `anyOf` and `oneOf`.

`schemars` joined the `terminal` feature instead of getting a feature of its own: every binary already depends on it.

Verified: 27 unit tests pass with all features; clippy clean on all features and on `terminal` alone; `--no-default-features` builds. A check of himalaya against the local crate fails only on the `execute` signature, as expected; its own error.rs copy still compiles.

Spec started: output (Piped JSON is newline-delimited, Schemas are stamped with the contract version, Schema roots can be checked for objects); error (Failures carry an optional stable code, The failure object has a schema).
