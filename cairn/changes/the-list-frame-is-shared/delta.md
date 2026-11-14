---
cairn: delta
change: the-list-frame-is-shared
---

## ADDED Requirements

### Requirement: A list prints under the fields placing it
`listing::Listing` SHALL print a list as its header fields, one blank line, then its items as `columns::Columns` prints them with no header row. A header field SHALL print as `Label:` padded so the values start in one column, two spaces past the colon of the longest label and never before the column `Next page:` sets, so values align on every page. `Listing::new` SHALL head it with `Account:`, the account name sanitized and the backend in parentheses when given; `Listing::head` SHALL add a field, its value sanitized. `Listing::page` SHALL add `Page:` and, while a page follows, `Next page:` with the flag reaching it: `--page <n>` for a page by number or the whole listing, `--page-token <token>` for a page reached by a token. The page SHALL read `<page>, <noun> <first>` or `<page>, <nouns> <first> to <last>` when numbered, `all, ...` when unpaged, `<count> <noun(s)>` when reached by a token, `empty` when it holds nothing, and end with `, the last` on a numbered or token page with items that no page follows. Nothing SHALL print under the items but the item errors. With no item, the `empty` line SHALL print in their place when set, and the header SHALL stand alone otherwise. A listing with no header field SHALL print its items alone.

#### Scenario: The second page of envelopes
- GIVEN a listing of the account `work` on `imap`, the field `Mailbox` = `INBOX`, the numbered page 2 of size 2 holding two envelopes, page 3 following
- WHEN it is printed
- THEN it reads `Account:    work (imap)`, `Mailbox:    INBOX`, `Page:       2, envelopes 3 to 4`, `Next page:  --page 3`, a blank line, then one line per envelope

### Requirement: Item errors print last
`error::ItemErrors` SHALL print the item errors of a command as one line per error, in the order pushed: `Error:`, the item's id, the code and the message, aligned among themselves as `columns::Columns` aligns cells, an absent id or code written `-`, the id and message sanitized. A command SHALL print them after its output and one blank line (`ItemErrors::write_after`), and SHALL print nothing, blank line included, when there are none. `listing::Listing` SHALL print its errors that way after its items, or alone when it prints nothing else.

#### Scenario: Two errors under a listing
- GIVEN a listing holding one envelope, an error coded `message-unreadable` with no id and an uncoded error of the item `42`
- WHEN it is printed
- THEN the envelope line is followed by a blank line, `Error:  -   message-unreadable  Cannot decode it` and `Error:  42  -                   Gone`

### Requirement: An account's capabilities print one line each
`capability::AccountCapabilityOutput` SHALL be the output of every binary's `account capability`: `{account, capabilities: [{name, backend, level, detail}]}`, one row per capability of the unified API, `backend` the backend it resolves to or `null` when none is allowed, `level` one of `full`, `partial`, `server` and `none`, `detail` what the level leaves out or `null`. Its text SHALL be a `listing::Listing` headed by the account, one line per capability: the name, the backend, the level and the detail.

#### Scenario: A capability with no transport
- GIVEN the account `work` serving `message.move` through `imap` in full and `message.send` through no backend
- WHEN the output is printed
- THEN it reads `Account:    work`, a blank line, `message.move  imap  full  -` and `message.send  -     none  No transport`, and the JSON row of `message.send` has `"backend": null` and `"level": "none"`

## MODIFIED Requirements

## REMOVED Requirements
