---
cairn: log
change: the-rules-match-the-family
landed: 2026-11-15
---

# The rules match the family

Tasks L11 and L12 of the CLI alignment round of 2026-11-15, and the owner's ruling D15.

## What landed

- `account capability` belongs to a binary with a unified API; one whose backends serve none (carillon, neverest) carries none.
- vCard and iCalendar content prints beside `encoding`, `utf-8` or `base64`, as cardamum and calendula print it.
- A queued write prints the id the store gives the item, its link id, never a placeholder; `null` only when the backend gives none.

Spec only: no code of this crate changed.

## Capabilities moved

- [commands](../spec/commands.md): *Every binary carries an account group*, the `account capability` sentence.
- [output](../spec/output.md): *A write answers a receipt*, the `id` key; *Bytes are never raw under JSON*, the `encoding` key.
