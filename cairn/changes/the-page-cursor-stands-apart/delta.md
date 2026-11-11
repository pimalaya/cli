---
cairn: delta
change: the-page-cursor-stands-apart
---

# Delta

## ADDED Requirements

None.

## MODIFIED Requirements

### Requirement: Paging resumes
A command that can stop before the end of a listing SHALL print `nextPage`, `null` once the listing is complete, and SHALL accept that value back through `--page-token`. In text, the cursor SHALL print as a `Next page: <cursor>` line after the listing, one blank line apart when the listing printed anything.

## REMOVED Requirements

None.
