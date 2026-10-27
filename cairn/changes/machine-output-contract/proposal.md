---
cairn: change
id: machine-output-contract
status: landed
created: 2026-10-27
---

# One versioned machine contract for every binary

## Why

The NLnet work plans a REPL and an MCP server on top of the CLIs. Both are machine callers, and both need the same thing the business page already sells to integrators: a stable, versioned contract of JSON payloads and error codes. Today each piece of that contract exists, but in the wrong place or not quite right:

- Error codes are copied three times. himalaya, calendula and cardamum each carry an identical `ErrorCode`, `CodedError`, `code_of`, `CodedReport` and `eval` in src/error.rs (calendula and cardamum differ only in their variants and doc wording). A fourth binary would copy them again, and the report shape they print has no JSON Schema, so a caller cannot learn the error object the way it learns the data objects.
- Piped `--json` is not line-delimited. `StdoutPrinter::out` writes compact JSON with no trailing newline and no flush when stdout is not a terminal, so two outputs of one process (a REPL session, a streaming command like `comodoro watch`) mash together. Ortie's REPL works around it with its own newline between commands.
- Schemas carry no version. `JsonSchemaCommand` writes each schema as the binary built it, so nothing in a generated file says which contract it describes, and an MCP server or a cached schema set cannot tell a himalaya 2 payload from a himalaya 3 one.
- Nothing checks that a payload root is an object. MCP's `structuredContent` must be a JSON object; a `serde(transparent)` newtype over a `Vec` would make a command unusable as an MCP tool, and no test would notice.

Per-payload stamping (a `version` field in every object) was considered and rejected: it repeats on every line what the binary version already says, and MCP reports the server version once at initialization. The contract version is the binary's major version, and it lives in the schemas.

## What

1. src/error.rs gains the coded-error layer, generic over the binary's code enum: `CodedError<C>`, `code_of::<C>`, and an `ErrorReport` that prints `code` beside `error`, `sources` and `backtrace` when the chain carries one. `ErrorReport<C = NoErrorCode>` keeps `ErrorReport::eval` unchanged for the binaries without codes, and adds `ErrorReport::<Code>::eval_coded` for the others (one generic `eval` would have broken the ten uncoded callers: a default type parameter does not drive inference). The vocabulary stays in each binary: pimalaya-cli owns the mechanism and the rule that values are kebab-case, never the list.
2. The report implements `JsonSchema` (`schemars` joins the `terminal` feature rather than a feature of its own, since every binary already depends on it) `JsonSchema`, so each binary registers a `<bin>-error` key describing its failure object, codes included.
3. `StdoutPrinter` ends every compact JSON value with a newline and flushes, making piped `--json` newline-delimited JSON. The pretty terminal branch already ends with a newline.
4. `JsonSchemaCommand` stamps each schema with `$id` = `https://pimalaya.org/schemas/<bin>/v<major>/<key>.json`, the binary passing its name and `CARGO_PKG_VERSION`. The URL need not resolve yet; hosting the schema sets is a later website change. Before 1.0 the contract version is `0.<minor>`, since semver breaks on the minor there.
5. `JsonSchemaCommand::execute` takes a `JsonSchemaRegistry { bin, version, schemas }`, whose `non_object_roots` returns the keys whose schema root is not an object, for each binary's existing registry test to assert empty.

pimalaya-cli is 0.x and item 4 changes a public signature, so this ships as 0.3.0.

## Scope / non-goals

- The camelCase key switch stays per binary (himalaya, ortie, comodoro owe it at their next major). It should land in the same major as their adoption of this contract, so the first stamped `v<major>` is already camelCase.
- The REPL and the MCP server themselves are separate changes; they consume this contract.
- Per-binary adoption (dropping the three error.rs copies, registering `<bin>-error`, passing the version) is a follow-up change in each repository.
- Ortie's REPL spec writes errors to stderr when piped, against the family rule that errors go to stdout under the exit-code contract. That is ortie's to fix when it adopts this change; noted here so it is not lost.
