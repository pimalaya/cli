---
cairn: log
change: the-tree-gets-its-pages
landed: 2026-11-12
---

# The tree gets its pages, the tool its types

`manual` now builds the tree without clap's `help` subcommands and walks it depth first: every visible command gets a page named and titled after its dashed path, its synopsis spelling the whole command and its source the root's name and version, so each SEE ALSO entry names a page the run writes (Himalaya: 278 pages, all 277 references resolving). The `manual` and `json-schema` help name commands generically, no longer citing `neverest-sync.events`. `mcp::grammar::Grammar` takes a lone value for an array of one, makes an argument taking one value a required scalar (an array of one still accepted), types integer value parsers `integer`, gains `describe` to replace a command-line help, and writes a backquoted long flag of a property as the property's name. Found by the Himalaya live test of 2026-11-12 (F32, F34). Capabilities moved: terminal (requirement "Every command has a manual page" added), mcp ("A tool's input is its command's grammar" modified).
