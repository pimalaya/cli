---
cairn: log
change: a-list-hangs-as-a-tree
landed: 2026-11-12
---

# A list hangs as a tree

Added `tree::Trees` and `tree::Node`: roots one blank line apart, each child on a `├─ ` or `└─ ` branch with one line of detail under its title and its own children below, the way `tree` prints a directory. The binaries print a list as `Trees` when its items hang from a parent, `Records` for other wide items, `Columns` for short rows. Capability moved: terminal (requirement "A list hangs as a tree" added, "A wide list prints as records" modified for the rule).
