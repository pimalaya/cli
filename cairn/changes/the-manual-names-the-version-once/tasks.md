---
cairn: tasks
change: the-manual-names-the-version-once
---

# Tasks

- [x] src/clap/commands/manual.rs: drop the leading `v` of each page's long version
- [x] Cargo.toml: clap's `string` feature
- [x] Tests: the page of a tree whose long version is `v1.2.3 +imap` reads `.SH VERSION` then `v1.2.3 +imap`, never `vv1.2.3` (failed before the fix)
- [x] CHANGELOG; fold into the spec; log; land
