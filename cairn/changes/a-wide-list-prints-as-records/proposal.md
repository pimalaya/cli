---
cairn: change
id: a-wide-list-prints-as-records
status: landed
created: 2026-11-11
---

# A wide list prints as records

## Why

`columns::Columns` never truncates and never asks the terminal its size, so a list whose rows carry several values of unbounded length (a Maildir id, a full address, a subject, a path) wraps and loses its alignment as soon as one value is long. The owner decided on 2026-11-11 that such lists print the way `git log` prints commits: one block per item, a title line, the item's fields, then its free text as an indented body.

## What

- `records::Records`, a list of `records::Record`: a title, state words printed after it in parentheses, `fields::Fields`, and a body indented by four spaces after a blank line, one blank line between items. A line is never wider than its own value.
- The rule the binaries follow: `Columns` when every cell but the last is short and bounded, `Records` otherwise.
- Additive, behind the `terminal` feature like its twins.
