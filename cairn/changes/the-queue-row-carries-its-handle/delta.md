---
cairn: delta
change: the-queue-row-carries-its-handle
---

## MODIFIED Requirements

### Requirement: A queue row keeps one shape
`pimdir queue list` SHALL print each row it lists, pending or parked, as `{row, handle, kind, collection, linkId, itemId, producer, queuedAt, attempts, unconfirmed, error}` plus its domain's label key. `handle` SHALL be a string, the same row under the store's identity, `<store-id>.<incarnation>.<row>` (io-pimdir's `PimdirActionHandle`, pimdir STORAGE §4.5, the identity read from `PimdirReader::change_cursor`), which `pimdir queue show` and `pimdir queue cancel` take in place of `row` and check against the store, so a row id never names a row of a copy restored since. (The other keys are unchanged.)

#### Scenario: A pending creation and a parked update
- GIVEN a contacts binary whose pimdir queue holds a pending `add` of the card `Jane Doe` and a parked `update` of another card
- WHEN `pimdir queue list` runs with `--json`
- THEN both rows carry the eleven shared keys, `handle` reading `<store-id>.<incarnation>.<row>`, and `fullName`, the `add` with its `linkId`, `itemId` `null` and `error` `null`, the `update` with `linkId` `null`, the card it updates as `itemId` and its `error` text

### Requirement: Shared nouns keep one shape
A queued action's row id SHALL print as `row` and its handle as `handle`. (The other nouns are unchanged.)

### Requirement: A write answers a receipt
The receipt keys SHALL include `row` and `handle`, the queue row a write appended and that row under the store's identity, as `pimdir queue list` prints them, on a receipt that names its row, such as a pimdir intent's, never in place of `id`; `queued` loses its "no queue id". (The other keys are unchanged.)

## REMOVED Requirements
