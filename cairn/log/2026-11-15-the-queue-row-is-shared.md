---
cairn: log
change: the-queue-row-is-shared
landed: 2026-11-15
---

# The queue row is shared

The owner's ruling D19 of the CLI alignment round of 2026-11-15, closing X3 row M9 and the queue part of schema review A4. Spec only: no code of this crate changed; the row's type in pimalaya-cli, which the ruling names, is left to a later change.

## What landed

- `pimdir queue list` prints every row, pending or parked, as `{row, kind, collection, linkId, producer, queuedAt, attempts, error}` plus `subject`, `fullName` or `summary`, each key typed with its nullability.
- `kind` is io-pimdir's action kind as the store holds it, kebab-case, an application's own `x-` kind as stored.
- Left open for the owner: whether keys beyond the shared ones (himalaya's `send` and `envelope`, calendula's `itemId`, `unconfirmed`) stay as domain extras.

## Capabilities moved

- [output](../spec/output.md): *A queue row keeps one shape* added.
