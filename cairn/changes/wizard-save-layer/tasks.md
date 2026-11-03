---
cairn: tasks
change: wizard-save-layer
---

# Tasks

- [ ] Compare the nine copies and settle the one behaviour to keep (himalaya/src/wizard/configure.rs:103 as the starting point)
- [ ] Add `ExistingConfig`, `append_or_print`, `save_or_print` and `print_saved` to src/wizard, generic over the config and the generated account
- [ ] Tests: an appended account keeps the existing ones and their comments, a broken file errors instead of reading as absent, a taken name and an existing default constrain the generated account
- [ ] CHANGELOG under [Unreleased]
- [ ] Release pimalaya-cli (A3), unless shared-account-resolution already shipped a release carrying this
- [ ] Start cairn/spec with the save layer's requirements; log; land
