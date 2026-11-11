---
cairn: change
id: one-config-path-per-flag
status: landed
created: 2026-11-11
---

# One path per `-c`

## Why

`clap::args::ConfigFlags` split each `-c` value on `:`, like `$PATH`, so a Windows path (`C:\config.toml`) or a Unix path holding a colon broke into two paths. Its help still offered the wizard for a missing file. Ortie 3.0 settled the flag as `ConfigPathsArg`: one path per occurrence, repeat the flag to merge files. The other binaries carry their own copy of the splitting flag.

## What

- `ConfigFlags` takes one path per `-c`, the flag repeatable: no `value_delimiter`, no environment variable. Its help is Ortie's, with no wizard in it. The type and its `paths` field keep their names, so a binary swaps its own flag for it.
- A test that `-c` keeps `:` and `;` inside one path.
