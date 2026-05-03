//! Re-export of the `comfy-table` tabular output crate.
//!
//! Consumers of `pimalaya-cli` enable the `table` feature and reach
//! the underlying types via [`crate::table`] instead of pulling
//! `comfy-table` in directly.

pub use comfy_table::*;
