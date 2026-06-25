use uuid::Uuid;

use crate::{
    cache::app_cache::RolesCache,
    domain::{email::Email, error::DomainError, password::Password, username::UserName},
};

pub struct NewUser {
    pub username: UserName,
    pub email: Email,
    pub role_id: Uuid,
    pub password: Password,
}

impl NewUser {
    pub fn new_user(
        email: String,
        password: String,
        username: String,
        roles: &RolesCache,
    ) -> Result<Self, DomainError> {
        let email = Email::parse(&email)?;
        let password = Password::parse(&password)?;
        let username = UserName::parse(&username)?;
        let role_id = roles
            .get_role_by_name("user")
            .ok_or(DomainError::Internal("Role id not found".into()))?;

        Ok(Self {
            username,
            email,
            password,
            role_id,
        })
    }
}
