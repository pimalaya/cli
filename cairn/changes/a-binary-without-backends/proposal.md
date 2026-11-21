---
cairn: change
id: a-binary-without-backends
status: landed
created: 2026-11-21
---

# A binary without backends

## Why

The account group, its shared nouns and `AccountListOutput`/`AccountCheckOutput` were shaped for binaries opening protocol backends (IMAP, SMTP, JMAP, Graph, DAV, pimdir). Ortie opens none: its account list printed `backends: []` and `notBuilt: []` on every row forever, and its check called the token storage and the OAuth endpoints "backends", lumping every endpoint into one row and never saying whether the stored token was usable. Ortie also always runs on one account, so a list of one checked account carried nothing a reader needed. The owner ruled on 2026-11-21 that Ortie takes its own account treatment, with separate commands, before its 3.0.0.

## What

- commands.md, "Every binary carries an account group": a binary without backends still carries the group, and MAY add `account show`, printing every property of one account without reaching anything.
- output.md, "Shared nouns keep one shape": the account row, the checked account and the backend check row bind binaries with backends; a binary without backends prints its own shapes, named after what it lists and checks.
- terminal.md, "The accounts and their checks print alike": `AccountListOutput` and `AccountCheckOutput` are the outputs of every binary with backends.
- No code moves: the kit keeps its types, Ortie stops using them (ortie cairn/changes/archive, the account change of 2026-11-21).

## Decisions

- **An exemption, not a second shared shape**: Ortie is the one binary without backends; a shared type for one binary is a type nobody else reads.
- **The group stays**: `account list` and `account check` keep their names and their meaning (list reaches nothing, check opens what an account stands on), so a person moving from himalaya to Ortie finds the same commands.
