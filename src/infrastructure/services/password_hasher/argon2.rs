use argon2::{
    password_hash::{
        rand_core::OsRng, 
        PasswordHash,
        SaltString
    },
    Argon2,
};

use crate::domain::services::PasswordHasher;
use crate::domain::errors::AppError;

pub struct Argon2PasswordHasher;

impl PasswordHasher for Argon2PasswordHasher {
    fn hash_password(&self, password: &str) -> Result<String, AppError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| {
                AppError::InternalServerError(format!("failed to hash password: {}", e))
            })?
            .to_string();

        Ok(password_hash)
    }

    fn verify_password(&self, password: &str, hash: &str) -> Result<bool, AppError> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| {
                AppError::InternalServerError(format!("invalid password hash: {}", e))
            })?;

        Ok(Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok())
    }
}