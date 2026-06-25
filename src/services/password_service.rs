use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use secrecy::ExposeSecret;

use crate::{domain::password::Password, error::ApplicationError};

pub fn hash_password(password: &Password) -> Result<String, ApplicationError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_ref().expose_secret().as_bytes(), &salt)
        .map(|v| v.to_string())
        .map_err(|e| {
            tracing::error!("Error hashing the password: {}", e);
            ApplicationError::InternalServerError
        })
}

pub fn verify_password(hash: &str, password: &Password) -> Result<bool, ApplicationError> {
    let hashed_password = PasswordHash::new(hash).map_err(|e| {
        tracing::error!("Error hashing the password: {}", e);
        ApplicationError::InternalServerError
    })?;

    Ok(Argon2::default()
        .verify_password(
            password.as_ref().expose_secret().as_bytes(),
            &hashed_password,
        )
        .is_ok())
}
