---
cairn: log
change: aligned-fields
landed: 2026-11-11
---

# Aligned fields

`fields::Fields` (src/fields.rs, `terminal` feature) holds `rows`, a list of a label and its rendered value, appended with `push`. Its `Display` measures the longest label in characters and prints each row as `Label:` padded so the values start two spaces past that label's colon, the layout of clap's help. A value's further lines are indented to the value column, an empty line inside it stays empty, an empty value prints its label alone, and every line ends with a newline. `values_align_past_the_longest_label` and `an_empty_record_prints_nothing` cover it. Ortie's `token inspect` is the first consumer.

Capabilities moved: terminal (cairn/spec/terminal.md: "A text record prints as aligned fields" added).
