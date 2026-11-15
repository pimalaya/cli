---
cairn: log
change: the-next-page-follows-the-items
landed: 2026-11-15
---

# The next page follows the items

The owner's ruling D27 of the CLI alignment round of 2026-11-15, closing X3 row L14: text prints the next-page line after the listing everywhere.

## What landed

- `listing::Listing` prints `Next page: <flag> <value>` after its items and before its item errors, one blank line apart, from the new field `Listing::next_page` that `Listing::page` sets; `Page:` stays in the header. The flag stays, where `output::Paginated` prints the bare cursor: a listing frame pages by `--page` or `--page-token`, a `Paginated` cursor only by `--page-token`.
- The header prints as `fields::Fields`, values two spaces past the colon of the longest label; the floor at the width of `Next page` is gone (`Account:    work` now `Account:  work`).
- terminal.md and output.md state the position in the same words; the order is items, next page, item errors, the errors staying the last lines of every output.

## Capabilities moved

- [terminal](../spec/terminal.md): *A list prints under the fields placing it*, *Item errors print last* and *An account's capabilities print one line each* modified.
- [output](../spec/output.md): *Paging resumes* modified.
