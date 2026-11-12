---
cairn: delta
change: the-tree-is-dropped
---

# Delta

## ADDED Requirements

None.

## MODIFIED Requirements

### Requirement: A wide list prints as records
`records::Records` SHALL print a list as one block per item, in the order pushed, one blank line between blocks. A block SHALL start with the item's title alone on a line, followed by its state words in parentheses, comma-separated, when it has any; then its fields as `fields::Fields` prints them; then, when the item has a body, a blank line and each line of the body indented by four spaces, an empty body line staying empty. Every line SHALL end with a newline, so an empty list prints nothing. Titles, state words, fields and bodies SHALL print as given. A binary SHALL print a list as `columns::Columns` when each row reads on one line, its longest value last, and as `Records` when a row carries more than one value of unbounded length.

#### Scenario: Two envelopes
- GIVEN the item `42` in the state `unread`, `flagged`, with the fields `From` = `Alice <a@example.org>` and `Date`, and the body `Hi`, then the item `41` in no state with the body `Re: Hi`
- WHEN the list is printed
- THEN it reads `42 (unread, flagged)`, its two fields, a blank line, `    Hi`, a blank line, then `41`, its fields, a blank line and `    Re: Hi`, each line ending with a newline

## REMOVED Requirements

### Requirement: A list hangs as a tree
Removed: no binary prints a tree any more, Himalaya listing envelopes and threads one line each through `columns::Columns`.
