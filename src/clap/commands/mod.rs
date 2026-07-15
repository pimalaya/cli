//! Ready-made clap commands shared by every binary.

mod completion;
mod manual;

#[doc(inline)]
pub use self::{completion::CompletionCommand, manual::ManualCommand};
