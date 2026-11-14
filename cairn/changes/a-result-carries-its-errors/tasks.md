---
cairn: tasks
change: a-result-carries-its-errors
---

# Tasks

- [ ] src/error.rs: `ItemError<C>` with `new(id, &err)`, its `Serialize`, `JsonSchema` (every key required, `id` and `code` nullable) and one-line `Display`
- [ ] Tests: the code found under context, `sources` outermost first, the schema's required keys, the `Display` line with and without id and code
- [ ] cairn/spec/error.md and output.md: fold the delta
- [ ] CHANGELOG under Unreleased; the release left to the owner
- [ ] Log; status landed
