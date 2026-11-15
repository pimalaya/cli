---
cairn: log
change: the-spinner-colours-a-terminal-only
landed: 2026-11-15
---

# The spinner colours a terminal only

Task L10 of the CLI alignment round of 2026-11-15, from live testing (neverest's N13: `neverest check 2>file` wrote colour sequences into the file, `NO_COLOR=1` changing nothing).

## What landed

- `Spinner::start` decides once whether to colour: stderr a terminal and `NO_COLOR` unset or empty. Otherwise `success`, `failure` and the render loop write the glyph with crossterm's plain `Print`, since crossterm under `NO_COLOR` still writes `ESC[m` around a styled glyph.
- A stream that is not a terminal now gets `✓ <message>` or `✗ <message>` and no escape byte, where it got `ESC[38;5;10m✓ESC[39m` (and `ESC[m✓ESC[m` under `NO_COLOR=1`). A terminal without `NO_COLOR` is unchanged.
- `a_stream_that_is_no_terminal_gets_plain_lines` runs the test binary again with stderr a pipe, `NO_COLOR` unset and `1`; it failed before the fix with the bytes live testing saw. `an_empty_no_color_keeps_the_colour` holds the empty-value rule.
- Checked by hand with examples/spinner.rs: `2>file` holds no escape byte; on a pseudo-terminal (`script`), 53 colour sequences without `NO_COLOR`, none under `NO_COLOR=1`.

## Capabilities moved

- [terminal](../spec/terminal.md): *The spinner colours a terminal only* added; the opening paragraph names it.
