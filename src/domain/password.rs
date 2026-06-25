use crate::domain::error::DomainError;
use secrecy::{SecretBox, SecretString};

pub struct Password(SecretString);

impl Password {
    pub fn parse(password: &str) -> Result<Self, DomainError> {
        let password = password.trim();
        let special_characters = ['!', '@', '#', '$', '%', '^', '&', '*'];

        if password.len() < 8 {
            return Err(DomainError::InvalidPassword(
                "Password must be at least 8 characters".into(),
            ));
        }

        if password.len() > 64 {
            return Err(DomainError::InvalidPassword(
                "Password cannot exceed 64 characters".into(),
            ));
        }

        if !password.chars().any(|v| v.is_lowercase()) {
            return Err(DomainError::InvalidPassword(
                "Password must contain at least one lowercase letter".into(),
            ));
        }

        if !password.chars().any(|v| v.is_uppercase()) {
            return Err(DomainError::InvalidPassword(
                "Password must contain at least one uppercase letter".into(),
            ));
        }

        if !password.chars().any(|v| v.is_numeric()) {
            return Err(DomainError::InvalidPassword(
                "Password must contain at least one number".into(),
            ));
        }

        if !password.chars().any(|v| special_characters.contains(&v)) {
            return Err(DomainError::InvalidPassword(
                "Password must contain at least one special character (!, @, #, $, %, ^, &, *)".into(),
            ));
        }

        Ok(Self(SecretString::new(password.into())))
    }
}


impl AsRef<SecretBox<str>> for Password {
    fn as_ref(&self) -> &SecretBox<str> {
        &self.0
    }
}
