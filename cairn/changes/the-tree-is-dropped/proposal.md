---
cairn: change
id: the-tree-is-dropped
status: landed
created: 2026-11-12
---

# The tree is dropped

## Why

The owner found the tree layout messy once Himalaya printed real mailboxes with it, and decided on 2026-11-12 that Himalaya lists envelopes and threads one line each, the way `mscan` does, through `columns::Columns`. No binary uses `tree::Trees` any more, and it never shipped in a release.

## What

- `tree::Trees` and `tree::Node` are removed, with the rule naming them.
- The rule the binaries follow: `Columns` when each row reads on one line, its longest value last, `Records` when a row carries more than one value of unbounded length.
- The `footer!` text ends with the Pimalaya paragraph again, under the bug tracker and sponsoring links, its last sentence closing with a full stop.
