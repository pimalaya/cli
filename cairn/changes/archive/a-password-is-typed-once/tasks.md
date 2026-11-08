---
cairn: tasks
change: a-password-is-typed-once
---

# Tasks

- [x] `KeyringProvider::store`: secret on stdin per provider, `security -i` line escaped, stderr text on failure
- [x] `keyring::prompt_password`: endpoint lines with the off-domain mention, one masked prompt
- [x] `keyring::prompt_store`: installed providers, store then read back, drop a failing provider, custom and raw entries
- [x] `review` module: `ReviewRow`, `ReviewAction`, `prompt`
- [x] `ortie` module: `sign_in`, `OrtieSignInError`, Ortie's stdout sent to stderr on Unix and Windows
- [x] Tests: off-domain rule, `security -i` escaping and line-break refusal, store and configure argv shapes
- [x] fmt, clippy with all features, tests, no-default-features and wizard-only builds, missing docs
- [x] CHANGELOG under [Unreleased]
- [x] Fold delta.md into cairn/spec/wizard.md; log; status `landed`; archive
- [ ] Verify against the real programs (never run on this machine): the four store commands, `security -i` quoting and exit status on macOS, the Windows stderr duplicate
- [x] Left to each CLI's own change: himalaya, cardamum, calendula, neverest adopt the new prompts (2026-11-08)
