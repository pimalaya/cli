---
cairn: log
change: plain-lists
landed: 2026-11-11
---

# Plain lists

`sanitize::text` (src/sanitize.rs, `terminal` feature) is `table::sanitize` moved, its body and three tests unchanged: C0, DEL, C1 and the UAX #9 embeddings, overrides and isolates become U+FFFD, clean text is borrowed.

`columns::Columns` (src/columns.rs, `terminal` feature) holds `header`, the labels, and `rows`, the rendered cells, built with `new(header)` and `push(row)`. Its `Display` turns each cell into one line (an empty cell `-`, each `\n` and `\r` a space), completes short rows with `-`, measures each column with unicode-width 0.2 (`UnicodeWidthStr::width`, no default features, so no CJK ambiguous-width mode), separates columns by two spaces, pads every column but the last, and ends each line with a newline. An empty header prints no header row, so `Columns::default()` prints nothing. Labels print as given: the caller passes them uppercase, the module does not change case, which keeps a label like `ID` or a translated one exactly as written. Five tests: a wide last column, CJK and emoji widths, empty and missing cells, a line break in a cell, a list without rows.

src/table.rs (`pub use comfy_table::*` and `sanitize`), the `table` feature, comfy-table and `printer::PrintTable` (a trait taking a terminal width, implemented nowhere in the family) are gone. Cargo.lock lost comfy-table (unicode-width 0.2.3, which it pulled, was already locked for inquire).

Spec: terminal ("A list prints as aligned columns" and "Untrusted text is defused" added, "Dependencies name their features" no longer names `table`, the intro names the text layouts).
