---
cairn: tasks
change: the-spinner-colours-a-terminal-only
---

# Tasks

- [x] src/spinner.rs: colour only on a terminal without a non-empty `NO_COLOR`, the glyph plain otherwise, in `finish` and the render loop
- [x] Tests: the test binary runs a spinner again with stderr a pipe, `NO_COLOR` unset and `1`, and reads `✓ Checked` and `✗ Not sent` with no escape byte (failed before the fix with the bytes live testing saw); the empty `NO_COLOR` keeping the colour
- [x] CHANGELOG; fold into the spec; log; land
