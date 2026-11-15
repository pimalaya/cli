---
cairn: log
change: the-account-outputs-are-shared
landed: 2026-11-15
---

# The account outputs are shared

Task L4 of the CLI alignment round of 2026-11-15 (findings F44 and F5, decision D5).

## What landed

- `account::AccountListOutput` and `account::AccountRow` (`{accounts: [{name, default, backends, notBuilt}]}`), `account::AccountCheckOutput<C>` and `account::BackendCheck<C>` (`{account, backends: [{backend, ok, code, error}]}`) with `BackendCheck::new`, behind `terminal`; `output::SavedOutput` (`{path, size}`).
- JSON identical to the apps' copies field for field. Docs and text taken from himalaya's where the copies differ, plus the ` (not built)` marker of the twins and their sanitizing of the account name and the check errors.
- Schema names kept for the generic types (`AccountCheckOutput`, `BackendCheck`), so the apps' schemas keep their `$defs` keys.

## Capabilities moved

- [terminal](../spec/terminal.md): *The accounts and their checks print alike* added.
- [output](../spec/output.md): *A file written is named with its size* added; *Shared nouns keep one shape* modified (`notBuilt`, `{path, size}`).
