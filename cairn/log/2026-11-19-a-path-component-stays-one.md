---
cairn: log
change: a-path-component-stays-one
landed: 2026-11-19
---

# A path component stays one

Owner ruling of 2026-11-19: the pimdir STORAGE §18 file name rule himalaya copied into src/shared/attachment/download.rs moves to the kit.

## What landed

- `sanitize::path_component` (src/sanitize.rs), io-pimdir's `security::path_component` rule character for character, with its own copy of the twelve STORAGE §18 bidi characters.
- `a_path_component_stays_one_component`, io-pimdir's cases plus `...` kept.

Additive. himalaya's `sanitize` also trims, strips leading dots and falls back to `attachment`: those stay its own around the shared call.

## Capabilities moved

- [terminal](../spec/terminal.md): *Untrusted text is defused* modified.
