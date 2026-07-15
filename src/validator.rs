//! Reusable inquire input validators.

use inquire::{
    CustomUserError,
    validator::{StringValidator, Validation},
};

/// Validates that the input parses as a u16.
#[derive(Clone, Debug, Default)]
pub struct U16Validator;

impl StringValidator for U16Validator {
    fn validate(&self, input: &str) -> Result<Validation, CustomUserError> {
        match input.parse::<u16>() {
            Ok(_) => Ok(Validation::Valid),
            Err(err) => Err(Box::new(err)),
        }
    }
}

/// Validates that the input parses as a usize.
#[derive(Clone, Debug, Default)]
pub struct UsizeValidator;

impl StringValidator for UsizeValidator {
    fn validate(&self, input: &str) -> Result<Validation, CustomUserError> {
        match input.parse::<usize>() {
            Ok(_) => Ok(Validation::Valid),
            Err(err) => Err(Box::new(err)),
        }
    }
}
