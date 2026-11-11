---
cairn: change
id: shared-account-resolution
status: abandoned
created: 2026-10-18
---

# Account resolution lives in pimalaya-cli

## Why

Every binary resolves the account a command runs against the same way: load the configuration, offer the wizard when there is none and a human can answer, read it again, then pick the named or the default account and name the accounts held when the name is unknown. The function is copied in six repositories (CLI_ALIGNMENT_PLAN.md, audited 2026-10-18, item H2):

- himalaya/src/cli.rs:221 `resolve_account`, the reference version
- calendula/src/cli.rs:141 `resolve_account`, with `load_config` beside it
- cardamum/src/cli.rs:174 `resolve_account`
- sirup/src/cli.rs:306 `resolve_account`
- comodoro/src/cli.rs:267 `take_account`
- ortie/src/cli.rs:149 `take_account`

The copies already disagree on what they return and on the wording of their three failures, which is how items like F2 (unnamed account errors) keep reappearing.

## What

One `resolve_account` in pimalaya-cli, behind the `terminal` and `wizard` features. What differs per binary is the configuration type, the wizard offer and the hint naming the sample configuration, so the open decision is how a binary hands those in: a trait its config implements, or closures passed at the call. That decision is the first task.

Once released, the six repositories drop their copy. Each per-repo `cli-alignment-leftovers` change names the copy it will remove.

## Abandoned

Superseded on 2026-11-11 by pimalaya-config's account-resolution change: the wizards leave the family, so resolution needs no prompt and lives in pimalaya-config (`TomlConfig::load` and `take_account` with typed errors), and the first-run welcome in pimalaya-cli's `welcome` module.
