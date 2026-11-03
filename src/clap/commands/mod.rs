//! Ready-made clap commands shared by every binary.

pub(crate) mod completion;
mod json_schema;
pub(crate) mod manual;

#[doc(inline)]
pub use self::{
    completion::CompletionCommand,
    json_schema::{JsonSchemaCommand, JsonSchemaRegistry, output_schema},
    manual::ManualCommand,
};
