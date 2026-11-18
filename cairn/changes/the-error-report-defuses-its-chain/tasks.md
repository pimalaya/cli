---
cairn: tasks
change: the-error-report-defuses-its-chain
---

# Tasks

- [x] src/sanitize.rs: `lines`, sharing the replacement with `text`
- [x] src/error.rs: the text report sanitizes the message and each source, and names `--json` when it replaced a character
- [x] Tests: a chain holding a CSI, an OSC 52 and BEL bytes across two lines prints no ESC or BEL, keeps its line feed and names `--json` (failed before the fix with the raw bytes); `lines` keeps the line feed and the tab
- [x] CHANGELOG; fold into the spec; log; land
