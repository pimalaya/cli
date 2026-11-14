---
cairn: log
change: the-list-frame-is-shared
landed: 2026-11-14
---

# The list frame is shared

Added src/listing.rs (`Listing`, `ListingPage`), lifted from Himalaya's src/listing.rs with the backend named by a string, the token page always available, and the header column at least as wide as `Next page:` but widening for a longer label such as `Addressbook:`. Added `error::ItemErrors`: the trailing `Error:` lines of any command, aligned through `Columns`, printed after one blank line and not at all when empty; `ItemError`'s one-line `Display` now reads the same cells. Added src/capability.rs (`AccountCapabilityOutput`, `AccountCapability`, `CapabilityLevel`), Himalaya's flat `account capability` shape with the level typed.

Owner decisions of 2026-11-14: lists print a header only, the paging in it; item errors print last, not in the header; `account capability` takes Himalaya's shape across the family. Himalaya, Calendula and Cardamum adopt the three in their own changes.

Spec: terminal ("A list prints under the fields placing it", "Item errors print last", "An account's capabilities print one line each" ADDED).

Verified: `cargo test --all-features` (108 unit tests, 6 new), clippy `-D warnings` on `--all-features`, `--no-default-features`, `--no-default-features --features terminal` and `--features build`, `cargo doc` with `-D missing_docs` and broken intra-doc links denied.
