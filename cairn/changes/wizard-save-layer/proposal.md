---
cairn: change
id: wizard-save-layer
status: active
created: 2026-10-18
---

# The wizard save layer lives in pimalaya-cli

## Why

Every binary of the family ends its wizard the same way: read what an existing configuration file constrains, then append the generated account to it or print it, and report where it landed. That layer has been copied into nine repositories since August 2026 and the copies drift (CLI_ALIGNMENT_PLAN.md, audited 2026-10-18, item H1):

- calendula/src/wizard/configure.rs:91 (`ExistingConfig`), :220 (`append_or_print`), :246 (`print_saved`)
- cardamum/src/wizard/configure.rs:91, :220, :246
- carillon/src/wizard/configure.rs:104, :236, :264
- comodoro/src/cli/configure.rs:198, :331, :360
- himalaya/src/wizard/configure.rs:103, :223, :278
- himalaya-tui/src/wizard/configure.rs:115, :254 (no `append_or_print`)
- neverest/src/cli/configure.rs:100, :253, :279
- ortie/src/wizard.rs:156, :284, :315
- sirup/src/wizard/configure.rs:116, :247, :276

## What

`ExistingConfig`, `append_or_print` and `print_saved` (with the `save_or_print` branch they sit beside) move into src/wizard, generic over the binary's configuration and generated-account types, behind the `wizard` feature. Each binary keeps only what is its own: the config type, the account renderer, and the "what to run next" hint.

Once released, every repository above drops its copy. Each per-repo `cli-alignment-leftovers` change names the copy it will remove.
