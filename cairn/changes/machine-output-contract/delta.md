---
cairn: delta
change: machine-output-contract
---

## ADDED Requirements

### Requirement: Piped JSON is newline-delimited
When JSON output is enabled and stdout is not a terminal, the printer SHALL write each value as compact JSON followed by a newline, and SHALL flush after it, so a process emitting several values produces one parseable JSON value per line.

### Requirement: Failures carry an optional stable code
The error report SHALL print a `code` field beside `error`, `sources` and `backtrace` when the error chain contains a coded error, the outermost one winning, and SHALL omit the field otherwise. Code values SHALL be kebab-case. The code vocabulary SHALL belong to each binary; pimalaya-cli SHALL provide only the mechanism.

### Requirement: The failure object has a schema
With the `schemars` feature, the error report SHALL derive a JSON Schema describing the failure object, its `code` typed by the binary's code enum.

### Requirement: Schemas are stamped with the contract version
`JsonSchemaCommand` SHALL set each generated schema's `$id` to `https://pimalaya.org/schemas/<bin>/v<major>/<key>.json`, where `<major>` is the binary's major version. The contract version is the binary's major version; payloads themselves SHALL carry no version field.

### Requirement: Schema roots can be checked for objects
pimalaya-cli SHALL provide a check returning the registry keys whose schema root is not a JSON object, so each binary can assert that every payload is usable as structured tool output.
