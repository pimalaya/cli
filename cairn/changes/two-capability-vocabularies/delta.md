---
cairn: delta
change: two-capability-vocabularies
---

## ADDED Requirements

## MODIFIED Requirements

### Requirement: An account's capabilities print one line each
`capability::AccountCapabilityOutput` SHALL be the output of every binary's `account capability`: `{account, capabilities: [{name, backend, level, detail}]}`, one row per capability of the unified API, `backend` the backend it resolves to or `null` when none is allowed, `level` one of `full`, `partial`, `server` and `none`, `detail` what the level leaves out or `null`. Its text SHALL be a `listing::Listing` headed by the account, one line per capability: the name, the backend, the level and the detail. A capability's `name` SHALL be the unified command it stands for, `<noun>.<verb>` in the binary's own nouns (`message.add`, `card.add`, `item.add`, `mailbox.create`). This vocabulary is not the pimdir store's: a sync engine reporting what a source declares to a store (neverest's `check`) SHALL print the store's names as pimdir STORAGE §15 spells them (`mail.message.add`, `collection.rename`), the names a store records and a queue refusal names, and a binary SHALL NOT translate one into the other in its output.

#### Scenario: A capability with no transport
- GIVEN the account `work` serving `message.move` through `imap` in full and `message.send` through no backend
- WHEN the output is printed
- THEN it reads `Account:    work`, a blank line, `message.move  imap  full  -` and `message.send  -     none  No transport`, and the JSON row of `message.send` has `"backend": null` and `"level": "none"`

## REMOVED Requirements
