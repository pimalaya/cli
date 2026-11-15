---
cairn: change
id: the-queue-row-is-shared
status: landed
created: 2026-11-15
---

# The queue row is shared

## Why

Report X3 of the CLI alignment round of 2026-11-15 (pimalaya/.github cairn/changes/cli-alignment-2026-11-15, row M9, and the queue part of schema review A4) found `pimdir queue list` printing rows in three shapes: himalaya `{row, producer, queuedAt, send, envelope}` with no `kind`, `collection` or `linkId`; cardamum `{row, kind, collection, linkId, producer, queuedAt, fullName}`; calendula the same with `itemId` and `summary` for `fullName`. The parked rows of all three are `{row, queuedAt, producer, (collection), kind, attempts, unconfirmed, error}`. The owner ruled D19 on 2026-11-15: one shared row, `{row, kind, collection, linkId, producer, queuedAt, attempts, error}`, plus each domain's label key (`subject`, `fullName`, `summary`).

## What

- Every row `pimdir queue list` prints, pending or parked, carries the eight shared keys and its domain's label, each typed and its nullability stated, grounded in io-pimdir's `PimdirPendingAction` and `PimdirParkedAction`, which carry all of them but the label.
- `kind` is the store's action kind, never re-spelled: io-pimdir's `PimdirAction::kind` and the intents of `io_pimdir::intent`, kebab-case, or an application's own `x-` kind as stored.

The ruling also puts the row's type in pimalaya-cli, as the account rows are (`account::AccountRow`). That type is code and is left to a later change: this one is spec only.

## Open

Ruled by the owner the same day (D21 of the plan): `itemId` and `unconfirmed` join the shared row, which every binary already printed in part; himalaya drops its `send` (always `kind` being `submit`) and `envelope` (the label it duplicates). The spec names the shared row and says nothing of other extras.
