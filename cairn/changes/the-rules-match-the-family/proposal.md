---
cairn: change
id: the-rules-match-the-family
status: landed
created: 2026-11-15
---

# The rules match the family

## Why

The CLI alignment round of 2026-11-15 (pimalaya/.github cairn/changes/cli-alignment-2026-11-15, tasks L11 and L12) found three rules of this spec the binaries cannot or do not follow, each for a good reason:

- commands.md asks every binary with backends for `account capability`, whose output lists the capabilities of the unified API. carillon and neverest have backends but no unified API, so the command would have nothing to report.
- output.md says vCard and iCalendar text is a string when UTF-8 and base64 otherwise, with no way for a script to tell which. cardamum and calendula now print an `encoding` key beside it.
- output.md says a write prints `id` `null` when queued. himalaya's `message add`, cardamum's `card add` and calendula's creations print the link id the store gives the queued item, which is real, not a placeholder. The owner ruled D15 on 2026-11-15: "yes, should be same behaviour everywhere".

## What

- `account capability` belongs to a binary with a unified API.
- vCard and iCalendar text travels with `encoding`, `utf-8` or `base64`.
- A queued write prints the id the store gives the item (its link id), `null` only when the backend gives none, never a placeholder.

Spec only: no code of this crate changes.
