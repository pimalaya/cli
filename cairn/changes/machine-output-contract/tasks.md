---
cairn: tasks
change: machine-output-contract
---

# Tasks

- [x] Settle the `eval` shape for binaries without codes: `eval` unchanged on the default `ErrorReport`, `eval_coded` for coded binaries
- [x] Move `CodedError`, `code_of` and the coded report from himalaya/src/error.rs into src/error.rs, generic over the code enum; keep the three existing tests, generalised
- [x] `JsonSchema` on the report, describing `code` as the binary's enum (`schemars` under `terminal`)
- [x] `StdoutPrinter`: newline and flush after each compact JSON value (not unit-tested: the printer writes to the real stdout)
- [x] `JsonSchemaRegistry` passed to `JsonSchemaCommand::execute`, stamping `$id` with the contract version; tests on the stamp and the `0.<minor>` rule
- [x] `JsonSchemaRegistry::non_object_roots` plus a test over `$ref`, `anyOf` and `oneOf` roots
- [x] CHANGELOG under [Unreleased] (Added, Changed)
- [x] Fold the delta into cairn/spec/output.md and cairn/spec/error.md; log; land
- [ ] Release 0.3.0, on the user's go
- [ ] Adoption in each binary, one change per repository (tracked there)
