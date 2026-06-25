use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Invalid Username: {0}")]
    InvalidUsername(String),

    #[error("Invalid Password: {0}")]
    InvalidPassword(String),

    #[error("Invalid Email: {0}")]
    InvalidEmail(String),

    #[error("Internal Error: {0}")]
    Internal(String),
}

