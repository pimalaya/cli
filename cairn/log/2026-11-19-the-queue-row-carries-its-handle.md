---
cairn: log
change: the-queue-row-carries-its-handle
landed: 2026-11-19
---

# The queue row carries its handle

Owner ruling of 2026-11-19: himalaya's queue rows and receipts align on an integer `row` plus a `handle` string, as calendula and cardamum print them.

## What landed

Documentation only, pimalaya-cli defines no queue row type: the shared queue row is `{row, handle, kind, collection, linkId, itemId, producer, queuedAt, attempts, unconfirmed, error}` plus the label key, `handle` being `<store-id>.<incarnation>.<row>` (io-pimdir's `PimdirActionHandle`, pimdir STORAGE §4.5); the shared nouns name `handle`; a receipt naming its queue row prints `row` and `handle`, never in place of `id`. Additive.

## Capabilities moved

- [output](../spec/output.md): *A queue row keeps one shape*, *Shared nouns keep one shape* and *A write answers a receipt* modified.
