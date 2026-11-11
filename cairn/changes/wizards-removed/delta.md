---
cairn: delta
change: wizards-removed
---

## ADDED Requirements

## MODIFIED Requirements

### Requirement: Shared nouns keep one shape
Every binary SHALL print an account row as `{name, default, backends}`, a backend check row as `{backend, ok, code, error}`, a collection row as `{id, name, description, color, default}`, a queued action's row id as `row`, and an address as `{name, email}`.

### Requirement: Dependencies name their features
Every dependency SHALL be declared with `default-features = false` and SHALL name the features pimalaya-cli asks of it, so no consumer gets a dependency feature, colour or a terminal query among them, that the code does not use. The dependencies of the `prompt`, `table` and `spinner` features SHALL name the features their defaults enable.

## REMOVED Requirements

### Requirement: A secret is stored through stdin
Wizards leave the family (owner decision 2026-11-08): nothing stores a secret in a keyring any more.

### Requirement: The user sees where a password goes
Removed with the wizards.

### Requirement: A password is typed once
Removed with the wizards.

### Requirement: The wizard stores the tested password
Removed with the wizards.

### Requirement: The review lists each pick
Removed with the wizards.

### Requirement: OAuth hands off to Ortie
Removed with the wizards.
