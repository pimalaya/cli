use core::fmt;

use inquire::{Confirm, InquireError, Password, PasswordDisplayMode, Select, Text};
use secrecy::SecretString;
use thiserror::Error;

use super::validator::{U16Validator, UsizeValidator};

#[derive(Debug, Error)]
pub enum PromptError {
    #[error("cannot prompt unsigned integer (u16)")]
    U16(#[source] InquireError),
    #[error("cannot prompt unsigned integer (usize)")]
    Usize(#[source] InquireError),
    #[error("cannot prompt secret")]
    Secret(#[source] InquireError),
    #[error("cannot prompt password")]
    Password(#[source] InquireError),
    #[error("cannot prompt text")]
    Text(#[source] InquireError),
    #[error("cannot prompt boolean")]
    Bool(#[source] InquireError),
    #[error("cannot prompt item from list")]
    Item(#[source] InquireError),
}

pub type PromptResult<T> = Result<T, PromptError>;

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

pub fn secret(prompt: impl AsRef<str>) -> PromptResult<String> {
    Password::new(prompt.as_ref())
        .with_display_mode(PasswordDisplayMode::Masked)
        .without_confirmation()
        .prompt()
        .map_err(PromptError::Secret)
}

pub fn some_secret(prompt: impl AsRef<str>) -> PromptResult<Option<SecretString>> {
    Password::new(prompt.as_ref())
        .with_display_mode(PasswordDisplayMode::Masked)
        .without_confirmation()
        .prompt_skippable()
        .map(|secret| secret.map(Into::into))
        .map_err(PromptError::Secret)
}

pub fn password(prompt: impl AsRef<str>) -> PromptResult<SecretString> {
    Password::new(prompt.as_ref())
        .with_display_mode(PasswordDisplayMode::Masked)
        .with_custom_confirmation_message("Confirm password")
        .prompt()
        .map(Into::into)
        .map_err(PromptError::Password)
}

pub fn text<T: AsRef<str>>(prompt: T, default: Option<T>) -> PromptResult<String> {
    let mut prompt = Text::new(prompt.as_ref());

    if let Some(default) = default.as_ref() {
        prompt = prompt.with_default(default.as_ref())
    }

    prompt.prompt().map_err(PromptError::Text)
}

pub fn some_text<T: AsRef<str>>(prompt: T, default: Option<T>) -> PromptResult<Option<String>> {
    let mut prompt = Text::new(prompt.as_ref());

    if let Some(default) = default.as_ref() {
        prompt = prompt.with_default(default.as_ref())
    }

    prompt.prompt_skippable().map_err(PromptError::Text)
}

pub fn bool(prompt: impl AsRef<str>, default: bool) -> PromptResult<bool> {
    Confirm::new(prompt.as_ref())
        .with_default(default)
        .prompt()
        .map_err(PromptError::Bool)
}

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
