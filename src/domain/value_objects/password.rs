use crate::domain::constants::MIN_PASSWORD_LENGTH;
use crate::domain::errors::AppError;
use crate::domain::services::PasswordHasher;

#[derive(Debug, Clone)]
pub struct Password(String);

impl Password {
    pub fn new(value: String, hasher: &impl PasswordHasher) -> Result<Password, AppError> {
        if value.len() < MIN_PASSWORD_LENGTH {
            return Err(AppError::InvalidInput(format!("password must be at least {} characters long: {}", MIN_PASSWORD_LENGTH, value)));
        }

        let hashed_password = hasher.hash_password(&value)?;
        Ok(Password(hashed_password))
    }

    pub fn from_hash(hash: String) -> Self {
        Self(hash)
    }

    pub fn verify(&self, value: String, hasher: &impl PasswordHasher) -> Result<bool, AppError> {
        hasher.verify_password(&value, self.as_str())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
