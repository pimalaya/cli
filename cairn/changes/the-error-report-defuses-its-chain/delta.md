---
cairn: delta
change: the-error-report-defuses-its-chain
---

## ADDED Requirements

### Requirement: The text report is defused
The text form of the error report SHALL run its message and each of its sources through `sanitize::lines`, so no control or bidi character of the chain reaches the terminal while its lines stay apart. When it replaced a character, its suggestions SHALL end with `Run with --json to see the exact characters of the error`. The JSON form SHALL keep the message and the sources exact.

#### Scenario: An id carrying escape sequences
- GIVEN the error `Cannot read message nope<ESC>[31mRED<BEL>` caused by a source on two lines, `Expected =` then `at line 2<ESC>]52;c;aGk=<BEL>`
- WHEN its text report is printed
- THEN it reads `Error: Cannot read message nope�[31mRED�`, and under `Caused by:` the source on its two lines with U+FFFD in place of ESC and BEL, the suggestions naming `--json`

## MODIFIED Requirements

### Requirement: Untrusted text is defused
`sanitize::text` SHALL replace each control character (C0, DEL, C1) and each bidi embedding, override and isolate of Unicode UAX #9 with U+FFFD, and SHALL borrow text holding none. `sanitize::lines` SHALL do the same, keeping the line feed and the tab, for a text printed on several lines. Both SHALL be available with the `terminal` feature.

## REMOVED Requirements
