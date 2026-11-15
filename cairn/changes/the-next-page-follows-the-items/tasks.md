---
cairn: tasks
change: the-next-page-follows-the-items
---

# Tasks

- [x] src/listing.rs: `Listing::next_page`, printed after the items and before the item errors; the header through `fields::Fields`, `LABEL_WIDTH` dropped (D27, X3 L14)
- [x] src/listing.rs, src/capability.rs: tests on the new layout, failing on the old one
- [x] cairn/spec/terminal.md: *A list prints under the fields placing it*, *Item errors print last*, the capability scenario; cairn/spec/output.md: *Paging resumes* states the position in the same words
- [x] CHANGELOG.md: one [Unreleased] line
- [x] Fold into the spec; log; land
