---
cairn: change
id: the-queue-row-carries-its-handle
status: landed
created: 2026-11-19
---

# The queue row carries its handle

## Why

calendula and cardamum print, beside every queue row's integer `row`, its `handle`, the row under the store's identity (`<store-id>.<incarnation>.<row>`, io-pimdir's `PimdirActionHandle`, pimdir STORAGE §4.5), in `pimdir queue list`, `pimdir status` and calendula's intent receipts: a bare row id restored from a copy of the store can name another row, a handle cannot. himalaya is being aligned on the same pair (owner ruling of 2026-11-19), but the family contract (output.md, *A queue row keeps one shape*) lists ten keys without `handle`, and its receipt keys say a queued write carries no queue id, which calendula's intent receipts already contradict.

## What

Documentation only, pimalaya-cli defines no queue row type: the shared row gains `handle` (eleven keys), the shared nouns name it, and the receipt keys admit `row` and `handle` on a receipt that names its queue row. Additive: no key is removed or re-spelled.
