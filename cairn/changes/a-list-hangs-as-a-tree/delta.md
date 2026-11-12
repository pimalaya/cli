---
cairn: delta
change: a-list-hangs-as-a-tree
---

# Delta

## ADDED Requirements

### Requirement: A list hangs as a tree
`tree::Trees` SHALL print its roots in the order pushed, one blank line between them. A root SHALL print its title alone on a line, then its detail, when it has one, after `│  ` when it has children and after three spaces otherwise. Each child SHALL print its title after its parent's prefix and `├─ `, or `└─ ` when it is the last child, an untitled child leaving no space after its branch; the prefix of its own lines SHALL be its parent's prefix followed by `│  `, or three spaces under a last child. A child's detail, when it has one, SHALL print after that prefix and `│  ` when the child has children, else right under its title. Every line SHALL end with a newline, so an empty list prints nothing. Titles and details SHALL print as given, each on one line.

#### Scenario: Envelopes under their mailbox
- GIVEN the root `INBOX` with no detail and two children, `Hi` with the detail `42 (unread)  Alice <a@example.org>` and `Re: Hi` with the detail `41  Carol <c@example.org>`
- WHEN the list is printed
- THEN it reads `INBOX`, `├─ Hi`, `│  42 (unread)  Alice <a@example.org>`, `└─ Re: Hi` and `   41  Carol <c@example.org>`, each line ending with a newline

#### Scenario: A reply with replies of its own
- GIVEN the root `Lunch` with the detail `1  Me` and two children, the first with the detail `2  Alice` and one child of its own with the detail `7  Carol`
- WHEN the list is printed
- THEN `1  Me` prints after `│  `, `2  Alice` after `│  │  `, its child's title after `│  └─ ` and `7  Carol` after `│     `

## MODIFIED Requirements

### Requirement: A wide list prints as records
`records::Records` SHALL print a list as one block per item, in the order pushed, one blank line between blocks. A block SHALL start with the item's title alone on a line, followed by its state words in parentheses, comma-separated, when it has any; then its fields as `fields::Fields` prints them; then, when the item has a body, a blank line and each line of the body indented by four spaces, an empty body line staying empty. Every line SHALL end with a newline, so an empty list prints nothing. Titles, state words, fields and bodies SHALL print as given. A binary SHALL print a list as `tree::Trees` when its items hang from a parent (a mailbox, the message they reply to), as `Records` when its rows otherwise carry more than one value of unbounded length, and as `columns::Columns` when every cell but the last is short and bounded.

#### Scenario: Two envelopes
- GIVEN the item `42` in the state `unread`, `flagged`, with the fields `From` = `Alice <a@example.org>` and `Date`, and the body `Hi`, then the item `41` in no state with the body `Re: Hi`
- WHEN the list is printed
- THEN it reads `42 (unread, flagged)`, its two fields, a blank line, `    Hi`, a blank line, then `41`, its fields, a blank line and `    Re: Hi`, each line ending with a newline

## REMOVED Requirements

None.
