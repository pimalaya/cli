---
cairn: delta
change: no-re-exports
---

## ADDED Requirements

### Requirement: Items are reached where they are defined
pimalaya-cli SHALL NOT re-export an item, its own or a dependency's: a binary SHALL import each item at the path of the module defining it.

#### Scenario: The completion command
- GIVEN a binary adding the shared completion command to its parser
- WHEN it imports the command
- THEN the path is `pimalaya_cli::clap::commands::completion::CompletionCommand`, and `pimalaya_cli::clap::commands::CompletionCommand` does not resolve

## MODIFIED Requirements

## REMOVED Requirements
