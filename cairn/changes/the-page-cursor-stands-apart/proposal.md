---
cairn: change
id: the-page-cursor-stands-apart
status: landed
created: 2026-11-11
---

# The page cursor stands apart

## Why

`output::Paginated` prints its `Next page:` line right under the listing. Under `records::Records` it reads as one more line of the last item's body, and under `columns::Columns` as one more row.

## What

The text cursor prints after one blank line when the listing printed anything, alone otherwise. JSON is unchanged.
