---
cairn: delta
change: protocol-listings-page-by-next-page
---

## ADDED Requirements

## MODIFIED Requirements

### Requirement: Paging resumes
A command that can stop before the end of a listing SHALL print `nextPage`, `null` once the listing is complete, and SHALL accept that value back through `--page-token`. A ranked search such as `pimdir search` SHALL page the same way, never by a `more` flag. Every listing of a protocol namespace that can stop before the end SHALL page the same way: `nextPage` SHALL be the provider's cursor as a string, as the provider gives it (Google's `nextPageToken`, Microsoft Graph's `@odata.nextLink`), `null` once the provider gives none, and `--page-token` SHALL take it back as is; a protocol giving no cursor (JMAP pages by position) SHALL print, as a string, the value the binary resumes from. The provider's cursor key SHALL be lifted into `nextPage`, not printed beside it; the rest of the payload SHALL stay raw, a sync token such as `nextSyncToken` or `@odata.deltaLink` included. In text, the cursor SHALL print as a `Next page: <cursor>` line after the listing, one blank line apart when the listing printed anything.

### Requirement: Protocol commands are raw-faithful
A command of a protocol namespace whose payload is JSON SHALL print the server's payload as received, under a designed collection key when it lists, without re-serializing it through a type that adds or drops fields. The provider's paging cursor alone SHALL move, lifted into `nextPage` (*Paging resumes*).

## REMOVED Requirements
