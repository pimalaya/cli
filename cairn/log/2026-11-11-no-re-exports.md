---
cairn: log
change: no-re-exports
landed: 2026-11-11
---

# No re-exports

src/clap/commands/mod.rs is a pure aggregator of `pub mod completion`, `pub mod json_schema` and `pub mod manual`; the `pub use` of `CompletionCommand`, `ManualCommand`, `JsonSchemaCommand`, `JsonSchemaRegistry` and `output_schema` is gone. Each module gained a `//!` line. `completion::serialize_shell`, `pub` but unreachable while the module was crate-private, became private rather than leak into the public API. The crate's own tests and doc links take the module paths, and the completion, manual and error tests import through `crate::` instead of `super::`.

Binaries now import `clap::commands::completion::CompletionCommand`, `clap::commands::manual::ManualCommand` and `clap::commands::json_schema::{JsonSchemaCommand, JsonSchemaRegistry, output_schema}`. The other re-export, src/table.rs's `pub use comfy_table::*`, went with plain-lists the same day.

Spec: terminal ("Items are reached where they are defined" added).
