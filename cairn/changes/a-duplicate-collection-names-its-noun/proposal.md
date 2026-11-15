---
cairn: change
id: a-duplicate-collection-names-its-noun
status: landed
created: 2026-11-15
---

# A duplicate collection names its noun

## Why

The last check of the CLI alignment round of 2026-11-15 (pimalaya/.github cairn/changes/cli-alignment-2026-11-15, report X3, row M4) found three answers to creating a collection that already exists: himalaya on pimdir fails coded `item-duplicate`, a collection under the item code against the noun rule of error.md; himalaya on Maildir fails uncoded; cardamum and calendula on vdir succeed. The owner ruled D17 on 2026-11-15: a create naming an existing collection fails everywhere with the collection noun, vdir included.

## What

- A create naming a collection that already exists fails coded `mailbox-duplicate`, `addressbook-duplicate` or `calendar-duplicate`, on every backend, local stores included, and a creation already queued in a pimdir store counts as existing.
- It never succeeds, fails uncoded or fails coded `item-duplicate`, which stays the one item code the domains share.
- A backend letting two collections share a display name holds no duplicate to refuse; inside a protocol namespace, a server's refusal stays a result.

Spec only: no code of this crate changes. Each binary adds the code to its enum and raises it.
