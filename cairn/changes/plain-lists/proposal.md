---
cairn: change
id: plain-lists
status: landed
created: 2026-11-11
---

# Plain lists

## Why

The owner decided on 2026-11-08 that the Pimalaya CLIs print pure text: no table borders, no colour, nothing sized to the terminal. A record already prints as `fields::Fields`. Lists still went through the `table` module, a `pub use` of comfy-table whole, which each binary dressed with borders, colours and terminal-width arrangements. `table::sanitize`, which every text output of untrusted strings needs (Himalaya calls it outside tables too), sat behind that same feature.

## What

- `sanitize::text`, moved out of src/table.rs with its behaviour and tests, behind `terminal` like the printer.
- `columns::Columns`, the list twin of `Fields`, behind `terminal`: a header of labels and rows of cells printed as borderless aligned columns, two spaces apart, widths in display columns (unicode-width, no default features), an empty cell as `-`, a line break inside a cell as a space, the last column unpadded, no truncation and no terminal query, every line ending with a newline, a list without rows printing its header alone. The caller passes the header labels as they print, uppercase by convention: the module does not change them.
- Remove src/table.rs, the `table` feature and comfy-table, and the `printer::PrintTable` trait, which nothing implements and which took a terminal width.
