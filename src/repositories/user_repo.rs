use sqlx::PgPool;

use crate::{
    domain::user::NewUser, error::ApplicationError, services::password_service::hash_password,
};

pub struct UserRepo {
    pool: PgPool,
}

impl UserRepo {
    pub fn new(pool: &PgPool) -> Self {
        Self { pool: pool.clone() }
    }

    #[tracing::instrument(name = "Inserting new user to the db", skip_all)]
    pub async fn add_user(&self, new_user: NewUser) -> Result<(), ApplicationError> {
        let hash_password = hash_password(&new_user.password)?;

        sqlx::query!(
            r#"
                INSERT INTO users (username, role_id, password)
                VALUES($1, $2, $3)
            "#,
            new_user.username.as_ref(),
            new_user.role_id,
            hash_password
        )
        .execute(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to insert user: {}", e);
            ApplicationError::InternalServerError
        })?;

        Ok(())
    }
}
