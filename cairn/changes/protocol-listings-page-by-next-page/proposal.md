---
cairn: change
id: protocol-listings-page-by-next-page
status: landed
created: 2026-11-15
---

# Protocol listings page by nextPage

## Why

Report X3 of the CLI alignment round of 2026-11-15 (pimalaya/.github cairn/changes/cli-alignment-2026-11-15, row M1, and the paging part of schema review A6) found protocol listings paging two ways: himalaya's Gmail, Graph and JMAP listings print `nextPage` and take `--page-token`, where cardamum's `gpeople` and `msgraph` listings and calendula's `msgraph` listings print the provider's own key (`nextPageToken`, `@odata.nextLink`) and take no flag to resume. output.md said both a raw payload and `nextPage`, without saying how they meet. The owner ruled D18 on 2026-11-15: every protocol listing prints `nextPage`, the provider's cursor as a string, and takes `--page-token`, the rest of the payload staying raw.

## What

- A protocol listing that can stop before the end prints `nextPage`, the provider's cursor as given, `null` once there is none, and takes it back through `--page-token` as is; a protocol giving no cursor (JMAP) prints the value the binary resumes from.
- The provider's cursor key is lifted into `nextPage`, not printed beside it; the rest of the payload stays raw, sync tokens (`nextSyncToken`, `@odata.deltaLink`) included.
- *Protocol commands are raw-faithful* names that one move.

A listing the binary walks to its end (calendula's `gcal` listings follow every page) cannot stop early and prints no cursor. Spec only: no code of this crate changes.
