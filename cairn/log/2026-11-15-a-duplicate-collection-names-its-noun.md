---
cairn: log
change: a-duplicate-collection-names-its-noun
landed: 2026-11-15
---

# A duplicate collection names its noun

The owner's ruling D17 of the CLI alignment round of 2026-11-15, closing X3 row M4. Spec only: no code of this crate changed.

## What landed

- A create naming a collection that already exists fails coded `mailbox-duplicate`, `addressbook-duplicate` or `calendar-duplicate` on every backend, Maildir, mbox, vdir and pimdir (a queued creation included) as much as a server; never a success, an uncoded failure or `item-duplicate`, which stays the one shared item code.
- A backend keying collections by generated ids, letting two share a display name, holds no duplicate; a protocol namespace keeps a server's refusal as its result.

## Capabilities moved

- [error](../spec/error.md): *A code names the domain's noun* modified, the scenario *A collection created twice* added.
