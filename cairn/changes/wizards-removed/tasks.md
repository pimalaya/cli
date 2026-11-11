---
cairn: tasks
change: wizards-removed
---

- [x] Delete src/wizard/, examples/imap_wizard.rs and its `[[example]]`
- [x] Drop the `wizard`, `imap`, `smtp`, `jmap`, `caldav` and `carddav` features, out of `default` too
- [x] Remove the prompt functions only wizards call, checked against the src/ of himalaya, cardamum, calendula, neverest, unfog, carillon, sirup, himalaya-tui and ortie
- [x] Remove src/validator.rs and the secrecy dependency
- [x] src/lib.rs header, README, CHANGELOG
- [x] fmt, clippy `-D warnings` per feature, tests, doc
- [x] Delta folded (wizard spec deleted, output and terminal specs), log written
