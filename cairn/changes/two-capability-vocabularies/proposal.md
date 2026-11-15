---
cairn: change
id: two-capability-vocabularies
status: landed
created: 2026-11-15
---

# Two capability vocabularies

## Why

The schema review of 2026-11-14 (finding A1) and the CLI alignment round of 2026-11-15 found capability names in two grammars: himalaya, cardamum and calendula print their unified commands (`message.add`, `card.add`), neverest's `check` prints the pimdir store's declarations (`mail.message.add`). They name two different things: what a frontend's commands can do on an account, and what a sync source declares to the store, the names the store records and every queue refusal prints. The owner ruled D22 on 2026-11-15: document the two vocabularies, rename nothing.

## What

"An account's capabilities print one line each" states that `name` is the unified command in the binary's nouns, that a sync engine prints the store's names, and that neither is translated into the other.
