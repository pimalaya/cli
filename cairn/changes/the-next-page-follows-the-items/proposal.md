---
cairn: change
id: the-next-page-follows-the-items
status: landed
created: 2026-11-15
---

# The next page follows the items

## Why

Report X3 of the CLI alignment round of 2026-11-15 (pimalaya/.github cairn/changes/cli-alignment-2026-11-15, row L14) found the spec contradicting itself on where `Next page:` prints: terminal.md had `listing::Listing` put it in the header, above the items (the owner decision of 2026-11-14 in the-list-frame-is-shared), where output.md's *Paging resumes* and `output::Paginated` print it after the listing. The owner ruled D27 on 2026-11-15: text prints the next-page line after the listing everywhere.

## What

- `Listing::page` keeps `Page:` in the header and stores the flag reaching the next page in the new field `Listing::next_page`; `Listing` prints it as `Next page: <flag> <value>` after the items, one blank line apart, the label and its single space as `Paginated` prints them. The flag stays (`--page 3`, `--page-token abc`) because a listing frame pages by two flags and the reader needs to know which; a `Paginated` cursor always goes back through `--page-token` and prints bare.
- Order: items, next page line, item errors. The line belongs to the listing it continues, and item errors stay the last lines of every output (*Item errors print last*).
- The header prints as `fields::Fields` prints it: values two spaces past the colon of the longest label. The floor at the width of `Next page` goes: it kept values in one column between a page with a next one and the last page, and the header now carries the same labels on every page. `Account:    work` becomes `Account:  work`.
- terminal.md and output.md state the position in the same words.

Every binary printing a `Listing` changes its text output on its next pimalaya-cli bump. Adding the public field `next_page` breaks code building a `Listing` by struct literal without `..Default::default()`; no binary of the family does.
