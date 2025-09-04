use crate::domain::errors::AppError;

pub trait PasswordHasher {
    fn hash_password(&self, password: &str) -> Result<String, AppError>;
    fn verify_password(&self, password: &str, hash: &str) -> Result<bool, AppError>;
}