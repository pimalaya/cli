---
cairn: delta
change: plain-lists
---

## ADDED Requirements

### Requirement: A list prints as aligned columns
`columns::Columns` SHALL print a list as its header row then one row per item, in the order pushed, with no border and no colour. Cells SHALL be separated by two spaces and padded to the widest cell of their column, measured in terminal display columns, so a wide character counts two; the last column SHALL NOT be padded, so no line ends with a space the layout added. An empty cell SHALL print as `-`, and a line feed or carriage return inside a cell SHALL print as a space. A row shorter than the widest one SHALL be completed with empty cells. Header labels SHALL print as given, and a list with no label SHALL print no header row. Nothing SHALL be truncated and the terminal SHALL NOT be asked for its size. Every line SHALL end with a newline, so a list without rows prints its header alone.

#### Scenario: A wide last column
- GIVEN the header `ID`, `NAME`, `DESCRIPTION` and the rows `1`, `Inbox`, `Mail received` and `42`, `Sent`, empty
- WHEN the list is printed
- THEN it reads `ID  NAME   DESCRIPTION`, `1   Inbox  Mail received` and `42  Sent   -`, each line ending with a newline and none with a space

#### Scenario: Wide characters
- GIVEN a cell `日本` in a column whose other cell is `ab`
- WHEN the list is printed
- THEN `ab` is followed by four spaces before the next column, `日本` by two

### Requirement: Untrusted text is defused
`sanitize::text` SHALL replace each control character (C0, DEL, C1) and each bidi embedding, override and isolate of Unicode UAX #9 with U+FFFD, and SHALL borrow text holding none. It SHALL be available with the `terminal` feature.

#### Scenario: A reversed file name
- GIVEN `invoice\u{202e}fdp.exe`
- WHEN it is sanitized
- THEN it reads `invoice�fdp.exe`

## MODIFIED Requirements

### Requirement: Dependencies name their features
Every dependency SHALL be declared with `default-features = false` and SHALL name the features pimalaya-cli asks of it, so no consumer gets a dependency feature, colour or a terminal query among them, that the code does not use. The dependencies of the `prompt` and `spinner` features SHALL name the features their defaults enable.

## REMOVED Requirements
