---
cairn: spec
capability: output
status: current
---

# Output

The printer writes a command's result to stdout, as text or JSON, and the JSON Schema command describes what the JSON looks like.

### Requirement: Piped JSON is newline-delimited
When JSON output is enabled and stdout is not a terminal, the printer SHALL write each value as compact JSON followed by a newline, and SHALL flush after it, so a process emitting several values produces one parseable JSON value per line.

### Requirement: Schemas are stamped with the contract version
`JsonSchemaCommand` SHALL set each generated schema's `$id` to `https://pimalaya.org/schemas/<bin>/v<version>/<key>.json`, where `<version>` is the binary's major version, or `0.<minor>` before 1.0. The contract version is the binary's major version; payloads themselves SHALL carry no version field.

### Requirement: Schema roots can be checked for objects
pimalaya-cli SHALL provide a check returning the registry keys whose schema root is not a JSON object (typed `object`, a reference to an object definition, or an `anyOf` or `oneOf` whose alternatives all are), so each binary can assert that every payload is usable as structured tool output.
