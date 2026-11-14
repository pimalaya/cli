---
cairn: change
id: the-list-frame-is-shared
status: landed
created: 2026-11-14
---

# The list frame is shared

## Why

Himalaya, Calendula and Cardamum each carry a copy of the same list frame: the account and what is listed above the items, the page and the flag reaching the next one. The copies have drifted: Himalaya prints the page above the items, Calendula and Cardamum below them, and the item errors of a result (a-result-carries-its-errors) were about to land in three more places. Their `account capability` outputs drifted the same way: Himalaya lists one row per capability with the backend serving it, Calendula one block per configured backend, Cardamum the resolved backend alone.

Owner decisions of 2026-11-14: lists print a header only, the paging in it, nothing under the items; a command's item errors print last, after one blank line, one `Error:` line each, aligned among themselves, nothing at all when there are none; `account capability` takes Himalaya's flat shape everywhere, its type moved here.

## What

- `listing::Listing` and `listing::ListingPage`, lifted from Himalaya (src/listing.rs) with the backend named by a string: the header fields, the items as `Columns` with no header row, the line printed when there are none, and the item errors last.
- `error::ItemErrors`, the trailing `Error:` lines of any command, which `Listing` prints and a command printing another shape writes with `ItemErrors::write_after`.
- `capability::AccountCapabilityOutput`, `AccountCapability` and `CapabilityLevel`: `{account, capabilities: [{name, backend, level, detail}]}`, printed as a `Listing`.

Himalaya, Calendula and Cardamum adopt them in their own changes.
