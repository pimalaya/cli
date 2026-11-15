---
cairn: change
id: the-family-commands-agree
status: landed
created: 2026-11-15
---

# The family commands agree

## Why

The CLI alignment round of 2026-11-15 (pimalaya/.github cairn/changes/cli-alignment-2026-11-15, task L3) found shared commands and keys apart across the binaries, and the owner ruled on each: the `account` group every binary carries (F1, D1), the shape of a read (F9, D7), `total` on a listing (F11, D8), `pimdir search` paging by `more` against "Paging resumes" (F12, D9), the noun of the error codes (F35), and the verb pairs that are domain differences rather than drift (P3). The family contract is this spec, so the rulings are written here before each binary aligns in its own tasks.

## What

Spec only:

- cairn/spec/commands.md, a new capability: the `account` group (`account list`, `account check`, `account capability` where a binary has backends, a former top-level `check` kept hidden); one meaning per verb (`add` an item, `create` a collection, `search` a query), and the two pairs that stay (`mailbox rename` beside `addressbook update` and `calendar update`; `raw` for line protocols beside `request` for HTTP APIs).
- cairn/spec/output.md: a read prints its list row plus its content, with `id` and `etag`, a file holding several items (calendula's `events: [...]`) staying legal; `total` an optional key of a listing; a ranked search such as `pimdir search` pages with `nextPage` and `--page-token`, never `more`.
- cairn/spec/error.md: a code names the domain's noun (`message-`, `mailbox-`, `card-`, `addressbook-`, `item-`, `calendar-`), `item-duplicate` the one shared code.
