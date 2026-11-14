---
cairn: tasks
change: a-result-carries-its-errors
---

# Tasks

- [x] src/error.rs: `ItemError<C>` with `new(id, &err)`, its `Serialize`, `JsonSchema` (every key required, `id` and `code` nullable) and one-line `Display`
- [x] Tests: the code found under context, `sources` outermost first, the schema's required keys, the `Display` line with and without id and code
- [x] cairn/spec/error.md and output.md: fold the delta
- [x] CHANGELOG under Unreleased; the release left to the owner
- [x] Log; status landed
