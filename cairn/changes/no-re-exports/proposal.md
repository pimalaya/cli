---
cairn: change
id: no-re-exports
status: landed
created: 2026-11-11
---

# No re-exports

## Why

The family's Rust style bans `pub use`: an item is reached at the path of the module defining it, so a reader finds its source from its import. src/clap/commands/mod.rs re-exported `CompletionCommand`, `ManualCommand`, `JsonSchemaCommand`, `JsonSchemaRegistry` and `output_schema` from private modules. src/table.rs re-exported comfy-table whole; the plain-lists change removes it.

## What

- The `completion`, `json_schema` and `manual` modules of `clap::commands` become `pub`, and the re-exports go.
- Apps import `clap::commands::completion::CompletionCommand`, `clap::commands::manual::ManualCommand` and `clap::commands::json_schema::{JsonSchemaCommand, JsonSchemaRegistry, output_schema}`.
