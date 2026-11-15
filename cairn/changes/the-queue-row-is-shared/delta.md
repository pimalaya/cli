---
cairn: delta
change: the-queue-row-is-shared
---

## ADDED Requirements

### Requirement: A queue row keeps one shape
`pimdir queue list` SHALL print each row it lists, pending or parked, as `{row, kind, collection, linkId, itemId, producer, queuedAt, attempts, unconfirmed, error}` plus its domain's label key: `subject` for mail, `fullName` for contacts, `summary` for calendars. Every key SHALL print, `null` or not. `row` SHALL be an integer, the queue row id `pimdir queue show` and `pimdir queue cancel` take. `kind` SHALL be a string, the row's action as the store holds it (pimdir STORAGE §15.3), read from io-pimdir (`PimdirAction::kind` for a pending row, `PimdirParkedAction::action` for a parked one): the store's own `add`, `set-flags`, `remove`, `move`, `copy`, `update` and `set-performer`, the intents of `io_pimdir::intent` (`submit`, `calendar-reply`, `calendar-cancel`, `collection-create`, `collection-rename`, `collection-delete`), all kebab-case, or an application's own (`x-`) as stored; it SHALL NOT be re-spelled. `collection` SHALL be a string, the id of the collection the row is queued on, as the binary's collection listing prints it. `linkId` SHALL be a string or `null`: the link id of the item the row creates, the `id` its write's receipt printed, `null` when the row creates no item or the store gives it none. `itemId` SHALL be a string or `null`: the id of the stored item the row addresses (an update, a flag change, a move, a copy, a removal, an intent on an item), as the binary's item listing prints it, `null` when the row addresses no stored item, a creation among them. `producer` SHALL be a string, the process that staged the row. `queuedAt` SHALL be a string, the instant the store stamped the row, RFC 3339 in UTC with `Z`. `attempts` SHALL be an integer from 0, how many times the sync engine tried the row. `unconfirmed` SHALL be a boolean, `true` when the row's outcome is unknown (pimdir STORAGE §15.2), its `error` then beginning `unconfirmed: `. `error` SHALL be a string or `null`: `null` while the row is pending, the store's text once it is parked, a text beginning `unconfirmed: ` marking a row whose outcome is unknown (pimdir STORAGE §15.2). The label SHALL be a string or `null`, read from the body the row pins or the item it addresses, `null` when there is none to read.

#### Scenario: A pending creation and a parked update
- GIVEN a contacts binary whose pimdir queue holds a pending `add` of the card `Jane Doe` and a parked `update` of another card
- WHEN `pimdir queue list` runs with `--json`
- THEN both rows carry the ten shared keys and `fullName`, the `add` with its `linkId`, `itemId` `null` and `error` `null`, the `update` with `linkId` `null`, the card it updates as `itemId` and its `error` text

## MODIFIED Requirements

## REMOVED Requirements
