---
cairn: change
id: a-path-component-stays-one
status: landed
created: 2026-11-19
---

# A path component stays one

## Why

himalaya copied io-pimdir's `security::path_component` rule (pimdir STORAGE §18) into its attachment download (src/shared/attachment/download.rs, `sanitize` and its own `BIDI` table) because io-pimdir is an optional dependency there. Every binary writing a file named by a server or a message needs the same rule, and `sanitize::text` does not fit: it writes U+FFFD and misses U+061C, U+200E and U+200F. Owner ruling of 2026-11-19: one kit function, so apps share it.

## What

`sanitize::path_component(name) -> String`, the rule of io-pimdir's `security::path_component` character for character: `/`, `\`, NUL, control and the twelve bidi formatting characters of STORAGE §18 become `_`; an empty name reads `_`, `.` and `..` read `_` and `__`. Tested against io-pimdir's own cases. Additive.
