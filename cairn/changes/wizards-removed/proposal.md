---
cairn: change
id: wizards-removed
status: landed
created: 2026-11-11
---

# Wizards removed

## Why

The owner decided on 2026-11-08 that the Pimalaya CLIs drop their configuration wizards: a first run prints a welcome naming the missing configuration and the commands creating it from the shipped sample, as Ortie 3.0 does. pimalaya-cli carried the shared wizard steps (the keyring store, the OAuth hand-off to Ortie, the review) and one wizard per protocol, behind the `wizard`, `imap`, `smtp`, `jmap`, `caldav` and `carddav` features, plus prompt helpers only those wizards called.

## What

- Delete src/wizard/ and examples/imap_wizard.rs with its `[[example]]`, and the `wizard`, `imap`, `smtp`, `jmap`, `caldav` and `carddav` features, out of `default` too.
- In src/prompt.rs, remove `u16`, `usize`, `secret`, `some_secret`, `password` and `items`, and the `PromptError` variants only they raised. `bool`, `text`, `some_text` and `item` stay: himalaya, cardamum, calendula, unfog, comodoro, io-pimdir, tcard and tcal call them outside their wizards.
- Remove src/validator.rs, used by the removed integer prompts only, and the secrecy dependency.
- Drop the wizard spec (cairn/spec/wizard.md), the wizard's output from the shared nouns of the output spec, and the `wizard` feature from the terminal spec's dependency rule.
- Update the src/lib.rs header and the README.
