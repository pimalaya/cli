---
cairn: log
change: the-family-commands-agree
landed: 2026-11-15
---

# The family commands agree

Task L3 of the CLI alignment round of 2026-11-15 (findings F1, F9, F11, F12, F35; decisions D1, D7, D8, D9; point P3). Spec only. The `--version` line of the same task landed with a-session-never-nests.

## What landed

- The `account` group every binary carries, and one meaning per verb with the two pairs that stay as domain differences (`mailbox rename` beside `addressbook update` and `calendar update`, `raw` beside `request`).
- A read prints its list row plus its content, with `id` and `etag`; a file holding several items may print them as a collection; `total` is an optional key of a listing; a ranked search such as `pimdir search` pages with `nextPage` and `--page-token`, never `more`.
- A code names its domain's noun, `item-duplicate` the one shared.

## Capabilities moved

- [commands](../spec/commands.md): new, *Every binary carries an account group* and *A verb keeps one meaning*.
- [output](../spec/output.md): *Collections are named and present* and *Paging resumes* modified.
- [error](../spec/error.md): *A code names the domain's noun* added.
