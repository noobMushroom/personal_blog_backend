use crate::domain::error::DomainError;

pub struct Email(String);

impl Email {
    pub fn parse(value: &str) -> Result<Self, DomainError> {
        let email = value.trim().to_lowercase();

        if !validator::ValidateEmail::validate_email(&email) {
            return Err(DomainError::InvalidEmail("Invalid email address".into()));
        }

        Ok(Self(email))
    }
}


impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_email() {
        assert!(Email::parse("user@example.com").is_ok());
    }

    #[test]
    fn missing_at_symbol() {
        assert!(Email::parse("userexample.com").is_err());
    }

    #[test]
    fn missing_domain() {
        assert!(Email::parse("user@").is_err());
    }

    #[test]
    fn empty() {
        assert!(Email::parse("").is_err());
    }

    #[test]
    fn trims_and_lowercases() {
        let email = Email::parse("  User@Example.COM  ").unwrap();
        assert_eq!(email.as_ref(), "user@example.com");
    }
}
