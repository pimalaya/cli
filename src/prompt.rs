//! Interactive prompt helpers wrapping the inquire crate.
//!
//! Thin typed wrappers for prompting free text, booleans and a choice
//! from a list, each returning a [`PromptResult`].

use core::fmt;

use inquire::{Confirm, InquireError, Select, Text};
use thiserror::Error;

/// Error raised when a prompt fails or is cancelled.
#[derive(Debug, Error)]
pub enum PromptError {
    /// Prompting free text failed.
    #[error("cannot prompt text")]
    Text(#[source] InquireError),
    /// Prompting a yes/no confirmation failed.
    #[error("cannot prompt boolean")]
    Bool(#[source] InquireError),
    /// Prompting a choice from a list failed.
    #[error("cannot prompt item from list")]
    Item(#[source] InquireError),
}

/// Result of a prompt helper.
pub type PromptResult<T> = Result<T, PromptError>;

/// Prompts for free text, with an optional default.
pub fn text<T: AsRef<str>>(prompt: T, default: Option<T>) -> PromptResult<String> {
    let mut prompt = Text::new(prompt.as_ref());

    if let Some(default) = default.as_ref() {
        prompt = prompt.with_default(default.as_ref())
    }

    prompt.prompt().map_err(PromptError::Text)
}

/// Prompts for free text, returning `None` when skipped.
pub fn some_text<T: AsRef<str>>(prompt: T, default: Option<T>) -> PromptResult<Option<String>> {
    let mut prompt = Text::new(prompt.as_ref());

    if let Some(default) = default.as_ref() {
        prompt = prompt.with_default(default.as_ref())
    }

    prompt.prompt_skippable().map_err(PromptError::Text)
}

/// Prompts for a yes/no confirmation, with a default.
pub fn bool(prompt: impl AsRef<str>, default: bool) -> PromptResult<bool> {
    Confirm::new(prompt.as_ref())
        .with_default(default)
        .prompt()
        .map_err(PromptError::Bool)
}

/// Prompts for one item chosen from a list, with an optional default
/// selection.
pub fn item<T: fmt::Display + Eq>(
    prompt: impl AsRef<str>,
    items: impl IntoIterator<Item = T>,
    default: Option<T>,
) -> PromptResult<T> {
    let items: Vec<_> = items.into_iter().collect();

    let default = if let Some(default) = default.as_ref() {
        items
            .iter()
            .enumerate()
            .find_map(|(i, item)| if item == default { Some(i) } else { None })
    } else {
        None
    };

    let mut prompt = Select::new(prompt.as_ref(), items).without_filtering();

    if let Some(default) = default.as_ref() {
        prompt = prompt.with_starting_cursor(*default);
    }

    prompt.prompt().map_err(PromptError::Item)
}
