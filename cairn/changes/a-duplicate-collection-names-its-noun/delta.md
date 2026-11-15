---
cairn: delta
change: a-duplicate-collection-names-its-noun
---

## ADDED Requirements

## MODIFIED Requirements

### Requirement: A code names the domain's noun
A code about one item or one collection SHALL start with the noun its binary's domain gives it: `message-` and `mailbox-` for mail, `card-` and `addressbook-` for contacts, `item-` and `calendar-` for calendars, as in `<noun>-not-found`. `item-duplicate`, an item whose UID its collection already holds, SHALL be the one such code the domains share. A create naming a collection that already exists SHALL fail coded with the collection noun, `mailbox-duplicate`, `addressbook-duplicate` or `calendar-duplicate`, on every backend: a local store (Maildir, mbox, vdir, pimdir, a creation already queued there included) as much as a server answering that the collection exists. It SHALL NOT succeed, fail uncoded or fail coded `item-duplicate`. A collection is named by what its backend keys it by, so a backend letting two collections share a display name holds no duplicate to refuse. Inside a protocol namespace, a server's refusal stays a result (cairn/spec/output.md, *A protocol refusal is a result*).

#### Scenario: An unknown id in two domains
- GIVEN an id no collection holds
- WHEN a mail binary reads it as a message and a contacts binary as a card
- THEN the first fails coded `message-not-found` and the second `card-not-found`

#### Scenario: A collection created twice
- GIVEN a mailbox `Archive` in a pimdir store and an address book `work` in a vdir, both existing
- WHEN a mail binary creates the mailbox `Archive` and a contacts binary the address book `work`
- THEN the first fails coded `mailbox-duplicate` and the second `addressbook-duplicate`, each exiting 1, and neither store changes

## REMOVED Requirements
