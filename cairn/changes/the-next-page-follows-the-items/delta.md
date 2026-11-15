---
cairn: delta
change: the-next-page-follows-the-items
---

## ADDED Requirements

## MODIFIED Requirements

### Requirement: A list prints under the fields placing it
`listing::Listing` SHALL print a list as its header fields as `fields::Fields` prints them, one blank line, then its items as `columns::Columns` prints them with no header row. The header's values SHALL start two spaces past the colon of the longest label; every page of a listing carries the same labels, so its values align from page to page. `Listing::new` SHALL head it with `Account:`, the account name sanitized and the backend in parentheses when given; `Listing::head` SHALL add a field, its value sanitized. `Listing::page` SHALL add `Page:` to the header and, while a page follows, set the flag reaching it: `--page <n>` for a page by number or the whole listing, `--page-token <token>` for a page reached by a token, the token sanitized. The page SHALL read `<page>, <noun> <first>` or `<page>, <nouns> <first> to <last>` when numbered, `all, ...` when unpaged, `<count> <noun(s)>` when reached by a token, `empty` when it holds nothing, and end with `, the last` on a numbered or token page with items that no page follows. The flag SHALL print as a `Next page: <flag> <value>` line after the listing's items and before its item errors, one blank line apart when anything printed before it. The line reads as `output::Paginated`'s `Next page: <cursor>`, the flag standing before the value because a listing frame pages by `--page` or by `--page-token` and the reader needs to know which, where a `Paginated` cursor always goes back through `--page-token`. Nothing else SHALL print under the items. With no item, the `empty` line SHALL print in their place when set, and the header SHALL stand alone otherwise, the next page line still following it. A listing with no header field SHALL print its items alone.

#### Scenario: The second page of envelopes
- GIVEN a listing of the account `work` on `imap`, the field `Mailbox` = `INBOX`, the numbered page 2 of size 2 holding two envelopes, page 3 following
- WHEN it is printed
- THEN it reads `Account:  work (imap)`, `Mailbox:  INBOX`, `Page:     2, envelopes 3 to 4`, a blank line, one line per envelope, a blank line, then `Next page: --page 3`

### Requirement: Item errors print last
`error::ItemErrors` SHALL print the item errors of a command as one line per error, in the order pushed: `Error:`, the item's id, the code and the message, aligned among themselves as `columns::Columns` aligns cells, an absent id or code written `-`, the id and message sanitized. A command SHALL print them after its output and one blank line (`ItemErrors::write_after`), and SHALL print nothing, blank line included, when there are none. `listing::Listing` SHALL print its errors that way after its items and its next page line, or alone when it prints nothing else: the next page line belongs to the listing it continues, and the errors stay the last lines of every output, a listing's included.

#### Scenario: Two errors under a listing
- GIVEN a listing holding one envelope, an error coded `message-unreadable` with no id and an uncoded error of the item `42`
- WHEN it is printed
- THEN the envelope line is followed by a blank line, `Error:  -   message-unreadable  Cannot decode it` and `Error:  42  -                   Gone`

#### Scenario: An error under a page that continues
- GIVEN a listing holding one envelope on the numbered page 1 of size 1, page 2 following, and an uncoded error of the item `42`
- WHEN it is printed
- THEN the envelope line is followed by a blank line, `Next page: --page 2`, a blank line and `Error:  42  -  Gone`

### Requirement: An account's capabilities print one line each
`capability::AccountCapabilityOutput` SHALL be the output of every binary's `account capability`: `{account, capabilities: [{name, backend, level, detail}]}`, one row per capability of the unified API, `backend` the backend it resolves to or `null` when none is allowed, `level` one of `full`, `partial`, `server` and `none`, `detail` what the level leaves out or `null`. Its text SHALL be a `listing::Listing` headed by the account, one line per capability: the name, the backend, the level and the detail. A capability's `name` SHALL be the unified command it stands for, `<noun>.<verb>` in the binary's own nouns (`message.add`, `card.add`, `item.add`, `mailbox.create`). This vocabulary is not the pimdir store's: a sync engine reporting what a source declares to a store (neverest's `check`) SHALL print the store's names as pimdir STORAGE §15 spells them (`mail.message.add`, `collection.rename`), the names a store records and a queue refusal names, and a binary SHALL NOT translate one into the other in its output.

#### Scenario: A capability with no transport
- GIVEN the account `work` serving `message.move` through `imap` in full and `message.send` through no backend
- WHEN the output is printed
- THEN it reads `Account:  work`, a blank line, `message.move  imap  full  -` and `message.send  -     none  No transport`, and the JSON row of `message.send` has `"backend": null` and `"level": "none"`

### Requirement: Paging resumes
A command that can stop before the end of a listing SHALL print `nextPage`, `null` once the listing is complete, and SHALL accept that value back through `--page-token`. A ranked search such as `pimdir search` SHALL page the same way, never by a `more` flag. Every listing of a protocol namespace that can stop before the end SHALL page the same way: `nextPage` SHALL be the provider's cursor as a string, as the provider gives it (Google's `nextPageToken`, Microsoft Graph's `@odata.nextLink`), `null` once the provider gives none, and `--page-token` SHALL take it back as is; a protocol giving no cursor (JMAP pages by position) SHALL print, as a string, the value the binary resumes from. The provider's cursor key SHALL be lifted into `nextPage`, not printed beside it; the rest of the payload SHALL stay raw, a sync token such as `nextSyncToken` or `@odata.deltaLink` included. In text, the cursor SHALL print as a `Next page: <cursor>` line after the listing's items and before its item errors, one blank line apart when anything printed before it; a listing framed by `listing::Listing` writes the flag reaching the page before the cursor (`Next page: --page 3`, cairn/spec/terminal.md).

## REMOVED Requirements
