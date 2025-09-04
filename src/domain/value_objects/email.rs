use crate::domain::constants::{EMAIL_REGEX, MAX_EMAIL_LENGTH};
use crate::domain::errors::AppError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(String);

impl Email {
    pub fn new(value: String) -> Result<Self, AppError> {
        if value.len() > MAX_EMAIL_LENGTH {
            return Err(AppError::InvalidInput(format!("email too long: {}", value)));
        }

        if !EMAIL_REGEX.is_match(&value) {
            return Err(AppError::InvalidInput(format!("invalid email format: {}", value)));
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
