---
cairn: delta
change: the-spinner-colours-a-terminal-only
---

## ADDED Requirements

### Requirement: The spinner colours a terminal only
`spinner::Spinner` SHALL draw its frames only when stderr is a terminal, and SHALL colour its frames and its `✓` and `✗` glyphs only when stderr is a terminal and `NO_COLOR` is unset or empty, as no-color.org asks. On a stream that is not a terminal, `success` and `failure` SHALL write their line as plain text, `<glyph> <message>`, with no escape byte, whatever `NO_COLOR` holds. On a terminal under a non-empty `NO_COLOR`, the spinner SHALL write no SGR sequence, the cursor control drawing its frames aside.

#### Scenario: Stderr redirected to a file
- GIVEN a binary run with `2>file`, `NO_COLOR` unset or set to `1`
- WHEN one spinner succeeds with `Checked` and another fails with `Not sent`
- THEN the file holds `✓ Checked` then `✗ Not sent`, each line ending with a newline, and no escape byte

#### Scenario: NO_COLOR on a terminal
- GIVEN stderr on a terminal and `NO_COLOR=1`
- WHEN a spinner runs then succeeds
- THEN stderr holds its frames and its `✓` line with no SGR sequence

## MODIFIED Requirements

## REMOVED Requirements
