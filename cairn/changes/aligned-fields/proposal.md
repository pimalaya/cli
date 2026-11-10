---
cairn: change
id: aligned-fields
status: landed
created: 2026-11-11
---

# Aligned fields

## Why

Each binary lays its one-record text output out by hand: Ortie's `token inspect` writes `Token type: bearer`, `With refresh token: true` with no alignment, and every other CLI of the family invents its own. The owner asked on 2026-11-10 which standard exists for plain-text data; none does formally, and the two conventions closest to the family are the `Label: value` header lines of RFC 5322 and the aligned two-column layout of clap's help. They asked on 2026-11-11 for a shared helper.

## What

- `fields::Fields`, behind the `terminal` feature: rows of a label and a rendered value, `push` to append one, and a `Display` printing aligned `Label: value` lines, a multi-line value continuing under its first line, every line ending with a newline.
- Values print as given, as table cells do; the caller sanitizes an untrusted one.
