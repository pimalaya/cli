---
cairn: change
id: the-tree-gets-its-pages
status: landed
created: 2026-11-12
---

# The tree gets its pages, the tool its types

## Why

A live test of Himalaya (2026-11-12, findings F32 and F34) met two shared gaps:

1. `manual` wrote one page for the root and one per first-level subcommand, none for a leaf (`Cannot find command himalaya-envelope-list`). The pages were rendered from detached subcommands, so their `.TH` read the short name (`envelope`), and their SEE ALSO read `envelope\-search(1)` and `envelope\-help(1)`, pages that do not exist. The `json-schema` help cited `neverest-sync.events` as its example, another program's command, in every binary.
2. `mcp::grammar::Grammar` typed every value a string and every repeatable argument an array. Himalaya's server instructions show `query` as a string, which the tool refused (`must be an array of strings`); an argument the binary takes one value of was an array of one; `--page-size` was a string; and a help written for the command line (`all of them when none is given`, `With --send`) reached the tool unchanged.

## What

- `manual` builds the tree first, without clap's `help` subcommands, then walks it depth first: every visible command gets a page named and titled after its dashed path (`himalaya-envelope-list`), its synopsis spelling the whole command, its `.TH` source the root's name and version. Each page SEE ALSO lists then exists.
- The `manual` and `json-schema` help name commands generically, with no program's command as example.
- `Grammar`: a lone value where an array is expected stands for an array of one; `one` makes an argument a required scalar (an array of one is still accepted); an integer value parser is typed `integer`; `describe` replaces an argument's description; a description naming a property's long flag between backquotes names the property instead.
- Not done: rewriting prose generically (`See --help`, `the standard input`): no rule tells a command-line-only sentence from a true one, so a binary reaches for `describe` or rewords its help.
