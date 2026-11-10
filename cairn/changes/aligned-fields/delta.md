---
cairn: delta
change: aligned-fields
---

## ADDED Requirements

### Requirement: A text record prints as aligned fields
`fields::Fields` SHALL print a record as one `Label: value` line per field, in the order pushed, the values starting two spaces past the colon of the longest label, as clap aligns its help. A value spanning several lines SHALL continue under its first line, an empty value SHALL leave its label alone with no trailing space, and every line SHALL end with a newline, so an empty record prints nothing. Values SHALL print as given.

#### Scenario: A token record
- GIVEN the fields `Type` = `Bearer`, `Refresh token` = `yes` and `Scopes` = `mail` then `contacts` on two lines
- WHEN the record is printed
- THEN it reads `Type:           Bearer`, `Refresh token:  yes`, `Scopes:         mail` and 16 spaces before `contacts`, each line ending with a newline
