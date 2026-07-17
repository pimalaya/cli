//! Interactive prompt helpers wrapping the inquire crate.
//!
//! Thin typed wrappers for prompting integers, secrets, passwords,
//! free text, booleans and a choice from a list, each returning a
//! [`PromptResult`].

use core::fmt;

use inquire::{Confirm, InquireError, MultiSelect, Password, PasswordDisplayMode, Select, Text};
use secrecy::SecretString;
use thiserror::Error;

use crate::validator::{U16Validator, UsizeValidator};

/// Error raised when a prompt fails or is cancelled.
#[derive(Debug, Error)]
pub enum PromptError {
    /// Prompting a u16 integer failed.
    #[error("cannot prompt unsigned integer (u16)")]
    U16(#[source] InquireError),
    /// Prompting a usize integer failed.
    #[error("cannot prompt unsigned integer (usize)")]
    Usize(#[source] InquireError),
    /// Prompting a masked secret failed.
    #[error("cannot prompt secret")]
    Secret(#[source] InquireError),
    /// Prompting a confirmed password failed.
    #[error("cannot prompt password")]
    Password(#[source] InquireError),
    /// Prompting free text failed.
    #[error("cannot prompt text")]
    Text(#[source] InquireError),
    /// Prompting a yes/no confirmation failed.
    #[error("cannot prompt boolean")]
    Bool(#[source] InquireError),
    /// Prompting a choice from a list failed.
    #[error("cannot prompt item from list")]
    Item(#[source] InquireError),
    /// Prompting several choices from a list failed.
    #[error("cannot prompt items from list")]
    Items(#[source] InquireError),
}

/// Result of a prompt helper.
pub type PromptResult<T> = Result<T, PromptError>;

/// Prompts for a u16 integer, with an optional default.
pub fn u16(prompt: impl AsRef<str>, default: Option<u16>) -> PromptResult<u16> {
    let prompt = Text::new(prompt.as_ref()).with_validator(U16Validator);

    let number = if let Some(default) = default {
        prompt.with_default(&default.to_string()).prompt()
    } else {
        prompt.prompt()
    };

    match number {
        Ok(number) => Ok(number.parse().unwrap()),
        Err(err) => Err(PromptError::U16(err)),
    }
}

/// Prompts for a usize integer, with an optional default.
pub fn usize(prompt: impl AsRef<str>, default: Option<usize>) -> PromptResult<usize> {
    let prompt = Text::new(prompt.as_ref()).with_validator(UsizeValidator);

    let number = if let Some(default) = default {
        prompt.with_default(&default.to_string()).prompt()
    } else {
        prompt.prompt()
    };

    match number {
        Ok(number) => Ok(number.parse().unwrap()),
        Err(err) => Err(PromptError::Usize(err)),
    }
}

/// Prompts for a masked secret without confirmation.
pub fn secret(prompt: impl AsRef<str>) -> PromptResult<String> {
    Password::new(prompt.as_ref())
        .with_display_mode(PasswordDisplayMode::Masked)
        .without_confirmation()
        .prompt()
        .map_err(PromptError::Secret)
}

/// Prompts for a masked secret, returning `None` when skipped.
pub fn some_secret(prompt: impl AsRef<str>) -> PromptResult<Option<SecretString>> {
    Password::new(prompt.as_ref())
        .with_display_mode(PasswordDisplayMode::Masked)
        .without_confirmation()
        .prompt_skippable()
        .map(|secret| secret.map(Into::into))
        .map_err(PromptError::Secret)
}

/// Prompts for a masked password with a confirmation prompt.
pub fn password(prompt: impl AsRef<str>, confirm: impl AsRef<str>) -> PromptResult<SecretString> {
    Password::new(prompt.as_ref())
        .with_display_mode(PasswordDisplayMode::Masked)
        .with_custom_confirmation_message(confirm.as_ref())
        .prompt()
        .map(Into::into)
        .map_err(PromptError::Password)
}

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

    let mut prompt = Select::new(prompt.as_ref(), items);

    if let Some(default) = default.as_ref() {
        prompt = prompt.with_starting_cursor(*default);
    }

    prompt.prompt().map_err(PromptError::Item)
}

/// Prompts for several items chosen from a list (a multi-select),
/// with the given item indices selected by default.
pub fn items<T: fmt::Display + Eq>(
    prompt: impl AsRef<str>,
    items: impl IntoIterator<Item = T>,
    default: impl IntoIterator<Item = usize>,
) -> PromptResult<Vec<T>> {
    let items: Vec<_> = items.into_iter().collect();
    let default: Vec<usize> = default.into_iter().collect();

    MultiSelect::new(prompt.as_ref(), items)
        .with_default(&default)
        .prompt()
        .map_err(PromptError::Items)
}
