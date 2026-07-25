//! Ready-made clap commands shared by every binary.

mod completion;
mod json_schema;
mod manual;

#[doc(inline)]
pub use self::{
    completion::CompletionCommand, json_schema::JsonSchemaCommand, manual::ManualCommand,
};
