---
cairn: log
change: mailboxes-keep-their-own-row
landed: 2026-11-15
---

# Mailboxes keep their own row

The owner's ruling D20 of the CLI alignment round of 2026-11-15, closing X3 row M3 and the mailbox part of schema review B2. Spec only: no code of this crate changed.

## What landed

- The shared collection row `{id, name, description, color, default}` covers address books and calendars; a mailbox row is `{id, name, role, total, unread}`, as himalaya prints it.

## Capabilities moved

- [output](../spec/output.md): *Shared nouns keep one shape* modified.
