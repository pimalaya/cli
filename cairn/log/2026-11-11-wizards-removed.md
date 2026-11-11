---
cairn: log
change: wizards-removed
landed: 2026-11-11
---

# Wizards removed

The owner decided on 2026-11-08 that the Pimalaya CLIs drop their configuration wizards. src/wizard/ (keyring, ortie, review and the IMAP, SMTP, JMAP, CalDAV and CardDAV wizards), examples/imap_wizard.rs with its `[[example]]`, and the `wizard`, `imap`, `smtp`, `jmap`, `caldav` and `carddav` features are gone; `default` is now `prompt`, `terminal`, `build`, `spinner` (and `table` until plain-lists removed it the same day).

src/prompt.rs keeps `bool`, `text`, `some_text` and `item`, which himalaya (pimdir queue cancel), cardamum and calendula (composers), unfog (gtd clarify), comodoro, io-pimdir, tcard and tcal call outside their wizards. `u16`, `usize`, `secret`, `some_secret`, `password` and `items` went with the `PromptError` variants only they raised (`U16`, `Usize`, `Secret`, `Password`, `Items`): a grep of the src/ of himalaya, cardamum, calendula, neverest, unfog, carillon, sirup, himalaya-tui and ortie found them called from wizards only (`items` in neverest/src/wizard/settings.rs). src/validator.rs, used by the integer prompts only, and the secrecy dependency went too (Cargo.lock lost secrecy and zeroize); thiserror stays for `PromptError`. The src/lib.rs header, README and CONTRIBUTING no longer describe a wizard.

Spec: wizard (cairn/spec/wizard.md deleted, its 6 requirements removed); output ("Shared nouns keep one shape" no longer names the wizard's output); terminal ("Dependencies name their features" no longer names `wizard`). The wizard-save-layer change is abandoned.

Verified with the other changes of the day: fmt, clippy `-D warnings` per feature, tests and doc pass.
