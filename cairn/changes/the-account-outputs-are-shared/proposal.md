---
cairn: change
id: the-account-outputs-are-shared
status: landed
created: 2026-11-15
---

# The account outputs are shared

## Why

The CLI alignment round of 2026-11-15 (pimalaya/.github cairn/changes/cli-alignment-2026-11-15, task L4, findings F44 and F5, decision D5, the approved optional tidying) found the outputs of `account list` and `account check`, and the receipt of a file written, copied in each app: `AccountRow` in himalaya, cardamum and calendula; `BackendCheck` and `AccountCheckOutput` in himalaya, cardamum, calendula and carillon (as `CheckOutput`); `SavedOutput` in himalaya, cardamum and calendula. The copies drifted in their docs and text, and himalaya's `account list` lists every declared backend where the twins list the built ones and add `notBuilt`, the shape the owner kept (D5).

## What

- `account::AccountListOutput` and `account::AccountRow`, `{accounts: [{name, default, backends, notBuilt}]}`, beside `capability::AccountCapabilityOutput` behind `terminal`.
- `account::AccountCheckOutput<C>` and `account::BackendCheck<C>`, `{account, backends: [{backend, ok, code, error}]}`, generic over the binary's code enum as `error::ItemError<C>` is, their schema names kept (`AccountCheckOutput`, `BackendCheck`); `BackendCheck::new` folds a connection attempt into its row.
- `output::SavedOutput`, `{path, size}`.

The JSON is the one the apps print today, field for field; where the copies differ, himalaya's is taken: its docs, and its text (`account list` with no header row and `default` on the default account; `Saved <size> bytes to <path>`). Two departures from himalaya's text: the left-out backends follow the built ones marked ` (not built)`, which himalaya does not list yet, and the account name and the check errors are sanitized as cardamum and calendula do, an error carrying a server's text. The apps drop their copies in their own tasks.
