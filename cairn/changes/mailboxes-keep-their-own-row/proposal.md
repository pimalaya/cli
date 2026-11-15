---
cairn: change
id: mailboxes-keep-their-own-row
status: landed
created: 2026-11-15
---

# Mailboxes keep their own row

## Why

Report X3 of the CLI alignment round of 2026-11-15 (pimalaya/.github cairn/changes/cli-alignment-2026-11-15, row M3, and the mailbox part of schema review B2) found himalaya's `mailbox list` rows `{id, name, role, total, unread}` against output.md's shared collection row `{id, name, description, color, default}`. A mailbox has a special-use role and message counts where an address book or a calendar has a description, a colour and a default. The owner ruled D20 on 2026-11-15: the collection row covers calendars and address books, mailboxes keep `{id, name, role, total, unread}`.

## What

- *Shared nouns keep one shape* names the address book or calendar row `{id, name, description, color, default}` and the mailbox row `{id, name, role, total, unread}`.

Spec only: no code of this crate changes, and himalaya already prints the mailbox row.
