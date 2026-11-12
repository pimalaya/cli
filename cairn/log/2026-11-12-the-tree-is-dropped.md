---
cairn: log
change: the-tree-is-dropped
landed: 2026-11-12
---

# The tree is dropped

Removed `tree::Trees` and `tree::Node`, unreleased, after Himalaya moved its envelope and thread lists to one line each through `columns::Columns`. The list rule now reads: `Columns` when each row reads on one line, its longest value last, `Records` when a row carries more than one value of unbounded length. The `footer!` text ends with the Pimalaya paragraph again. Capability moved: terminal (requirement "A list hangs as a tree" removed, "A wide list prints as records" modified for the rule).
