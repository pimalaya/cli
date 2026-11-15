---
cairn: delta
change: the-family-commands-agree
---

## ADDED Requirements

### Requirement: Every binary carries an account group
(commands) Every binary SHALL carry an `account` group holding `account list`, printing the accounts its configuration declares without reaching anything, and `account check`, opening what each account stands on (each backend it declares, for a binary with backends) and printing one row each. A binary with backends SHALL also carry `account capability`. A binary that had a top-level `check` SHALL keep it as a hidden alias of `account check`.

#### Scenario: A binary moving its check
- GIVEN a binary whose check ran as `check`
- WHEN it takes the account group
- THEN `account check` runs the check, and `check` still does without being listed in the help

### Requirement: A verb keeps one meaning
(commands) `add` SHALL create an item and `create` a collection; `search` SHALL run a query and `list` walk a collection. A binary that spelled one of them otherwise SHALL keep the old verb as a hidden alias. Two pairs SHALL stay, each fitting its domain: `mailbox rename` beside `addressbook update` and `calendar update`, IMAP renaming a mailbox and nothing else where an address book or a calendar has a name, a description and a colour to change; and `raw`, the namespace command of a line protocol (IMAP, SMTP, ManageSieve) sending its commands as typed, beside `request`, the one of an HTTP API (JMAP, Graph, People) sending one request.

#### Scenario: Creating a card
- GIVEN a contacts binary that spelled the creation of a card `card create`
- WHEN it takes the family verbs
- THEN `card add` creates the card, and `card create` still does without being listed in the help

### Requirement: A code names the domain's noun
(error) A code about one item or one collection SHALL start with the noun its binary's domain gives it: `message-` and `mailbox-` for mail, `card-` and `addressbook-` for contacts, `item-` and `calendar-` for calendars, as in `<noun>-not-found`. `item-duplicate`, an item whose UID its collection already holds, SHALL be the one such code the domains share.

#### Scenario: An unknown id in two domains
- GIVEN an id no collection holds
- WHEN a mail binary reads it as a message and a contacts binary as a card
- THEN the first fails coded `message-not-found` and the second `card-not-found`

## MODIFIED Requirements

### Requirement: Collections are named and present
(output) A collection SHALL be printed even when empty, under a plural key naming its rows, never under a generic key. A read SHALL print the row its list prints, plus what only a read carries (the item's content), with the item's `id` and `etag`, `etag` `null` where the backend versions no item. A read of one file holding several items, such as an iCalendar file holding a recurring event and its overrides, MAY print their rows as a collection (`events: [...]`) beside the file's `id` and `etag`. A listing MAY carry `total`, the number of items the whole listing holds: the key is optional across the binaries, and paging SHALL NOT depend on it.

### Requirement: Paging resumes
(output) A command that can stop before the end of a listing SHALL print `nextPage`, `null` once the listing is complete, and SHALL accept that value back through `--page-token`. A ranked search such as `pimdir search` SHALL page the same way, never by a `more` flag. In text, the cursor SHALL print as a `Next page: <cursor>` line after the listing, one blank line apart when the listing printed anything.

## REMOVED Requirements
