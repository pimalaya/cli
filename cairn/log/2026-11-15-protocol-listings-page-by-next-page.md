---
cairn: log
change: protocol-listings-page-by-next-page
landed: 2026-11-15
---

# Protocol listings page by nextPage

The owner's ruling D18 of the CLI alignment round of 2026-11-15, closing X3 row M1 and the paging part of schema review A6. Spec only: no code of this crate changed.

## What landed

- A protocol listing that can stop early prints `nextPage`, the provider's cursor as a string as the provider gives it (`nextPageToken`, `@odata.nextLink`), `null` once none, taken back as is through `--page-token`; JMAP prints the value the binary resumes from.
- The cursor key is lifted into `nextPage`, never printed beside it; the rest of the payload stays raw, sync tokens included.

## Capabilities moved

- [output](../spec/output.md): *Paging resumes* and *Protocol commands are raw-faithful* modified.
