---
cairn: change
id: first-run-welcome
status: landed
created: 2026-11-11
---

# First-run welcome

## Why

With the wizards gone (owner decision 2026-11-08), a person running a Pimalaya CLI without a configuration needs another way in. Ortie 3.0 set the convention: a welcome naming the missing file, the commands creating it from the sample configuration a release or a package ships next to the binary (or downloading it when none ships), then the first command to run. Its code (ortie/src/cli.rs `welcome`, `shipped_sample`, `find_sample`; ortie/src/auth/get.rs `shell_quote`, `invocation`) is Ortie-specific only in the binary name and a few sentences, so every CLI of the family can share it.

## What

Behind `terminal`, where the printer lives:

- `shell::quote`, Ortie's `shell_quote`: POSIX single quotes with `'\''` for an embedded quote, double quotes on Windows.
- `shell::invocation`, Ortie's `invocation` with the binary name as a parameter: the binary, one quoted `-c` per configuration path, a quoted `-a` when an account is named.
- `welcome::shipped_sample`, Ortie's `shipped_sample` and `find_sample` with the binary name as a parameter: share/doc/<bin>/config.sample.toml under the prefix holding the binary's directory, or under that directory, the binary canonicalized on Unix.
- `welcome::Welcome`, whose `Display` writes Ortie's welcome with the app's intro line, sample URL, next-step sentence and first command as fields; byte-identical to Ortie's for Ortie's inputs.
- `welcome::is_wanted`: a person meets the welcome when the printer is not JSON and stderr is a terminal.
- Ortie's tests ported: the welcome commands run as printed (POSIX, and its Windows twin), the shipped sample is found installed or extracted, quoting and the invocation.
