---
cairn: change
id: the-spinner-colours-a-terminal-only
status: landed
created: 2026-11-15
---

# The spinner colours a terminal only

## Why

Live testing on 2026-11-15 (pimalaya/.github cairn/changes/cli-alignment-2026-11-15, task L10, closing neverest's N13) found `neverest check 2>file` writing `ESC[38;5;10m✓ESC[39m` into the file, and `ESC[m✓ESC[m` under `NO_COLOR=1`. `Spinner::finish` (src/spinner.rs:104-111) wrote its glyph through crossterm's `PrintStyledContent` whether stderr was a terminal or not, while the spinner already drew no frame off a terminal. crossterm honours `NO_COLOR` by dropping the colour value only, keeping the `ESC[` and `m` around it. The kit's rule is plain text for a human (cairn/spec/terminal.md): the spinner's colour belongs to the terminal it animates, nowhere else.

## What

- `Spinner::start` decides once whether to colour: stderr a terminal and `NO_COLOR` unset or empty (no-color.org, read with `var_os`, so a value that is not UTF-8 counts as set).
- Off that, `success`, `failure` and the frames write the glyph with crossterm's plain `Print`: a stream that is not a terminal gets `<glyph> <message>` and no escape byte, a terminal under `NO_COLOR` gets no SGR sequence.
- On a terminal without `NO_COLOR`, nothing changes.

Non-breaking: no public item changed; `Spinner` gains a private field.
