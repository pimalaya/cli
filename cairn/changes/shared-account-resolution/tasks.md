---
cairn: tasks
change: shared-account-resolution
---

# Tasks

- [ ] Decide trait or closure for the per-binary parts (config type, wizard offer, sample hint)
- [ ] Add `resolve_account` from himalaya/src/cli.rs:221, keeping its three named failures: the path read, the accounts held, the two ways to pick a default
- [ ] Keep the offer guarded on a terminal stdin and a non-JSON printer, and the configuration read again after the wizard ran
- [ ] Tests: unknown name lists the accounts held, no default names `-a` and `default = true`, no configuration names the path looked for
- [ ] CHANGELOG under [Unreleased]
- [ ] Release pimalaya-cli (A3), unless wizard-save-layer already shipped a release carrying this
- [ ] Fold into cairn/spec; log; land
