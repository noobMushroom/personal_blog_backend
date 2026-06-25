use crate::domain::error::DomainError;

pub struct UserName(String);

impl UserName {
    pub fn parse(username: &str) -> Result<Self, DomainError> {
        if username.trim().is_empty() {
            return Err(DomainError::InvalidUsername(
                "username should not be empty".into(),
            ));
        } else if username.len() < 6 {
            return Err(DomainError::InvalidUsername(
                "username should be longer than 6 characters".into(),
            ));
        } else if username.len() > 30 {
            return Err(DomainError::InvalidUsername(
                "username should not be longer than 6 characters".into(),
            ));
        } else if !username.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err(DomainError::InvalidUsername(
                "Username can only contain letters, numbers, and underscores".to_string(),
            ));
        }

        Ok(Self(username.into()))
    }

}


impl AsRef<str> for UserName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
