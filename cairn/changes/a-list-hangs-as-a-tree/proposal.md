---
cairn: change
id: a-list-hangs-as-a-tree
status: landed
created: 2026-11-12
---

# A list hangs as a tree

## Why

The owner decided on 2026-11-12 that Himalaya prints the envelopes of a mailbox as a tree hanging from the mailbox, subject on the branch and the date, id, state and address on the line under it, and a thread as a tree of its replies. `records::Records` spends seven lines per envelope and cannot show a reply under its parent; the old IMAP thread view drew its own tree by hand. One shared shape serves both.

## What

- `tree::Trees`, a list of roots, each a `tree::Node`: a title on its branch (`├─ `, `└─ `), one line of detail under it, its children hanging below, one blank line between roots. The connectors are the ones Himalaya's body-structure tree already draws.
- The rule the binaries follow: `Trees` when the items hang from a parent (a mailbox, the message they reply to), `Records` for other items carrying more than one value of unbounded length, `Columns` when every cell but the last is short and bounded.
- Additive, behind the `terminal` feature like its twins, in the unreleased 0.4.1.
