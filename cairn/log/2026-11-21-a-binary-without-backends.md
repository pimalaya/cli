---
cairn: log
change: a-binary-without-backends
landed: 2026-11-21
---

# A binary without backends

The account group keeps binding every binary, but its shared shapes now bind only the binaries opening backends: a binary opening none, Ortie, prints its own account list and check, and may add `account show`. Owner ruling of 2026-11-21, ahead of Ortie 3.0.0.

## What landed

Spec only, no code: the kit keeps `AccountListOutput`, `AccountCheckOutput` and their rows for the binaries with backends.

Capabilities moved: commands ("Every binary carries an account group"), output ("Shared nouns keep one shape"), terminal ("The accounts and their checks print alike").
