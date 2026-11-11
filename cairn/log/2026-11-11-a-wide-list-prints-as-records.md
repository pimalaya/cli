---
cairn: log
change: a-wide-list-prints-as-records
landed: 2026-11-11
---

# A wide list prints as records

`records::Records` landed behind the `terminal` feature: one block per item, its title with state words in parentheses, its `Fields`, then its body indented by four spaces after a blank line, one blank line between items, the way `git log` prints commits. Additive. Capability moved: terminal (requirement "A wide list prints as records", with the rule choosing it over `Columns`). Himalaya and the pimdir binary adopt it for their wide lists.
