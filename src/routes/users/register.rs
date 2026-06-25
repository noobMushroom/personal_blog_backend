use actix_web::{HttpResponse, post, web};
use serde::Deserialize;

use crate::{
    cache::app_cache::AppCache, domain::user::NewUser, error::ApplicationError,
    repositories::user_repo::UserRepo,
};

#[derive(Debug, Deserialize)]
pub(crate) struct NewUserData {
    username: String,
    email: String,
    password: String,
}

#[tracing::instrument(name = "Registering new user", skip_all, fields(
        email= %new_user_data.email,
        username= %new_user_data.username
))]
#[post("signup")]
pub async fn signup(
    new_user_data: web::Json<NewUserData>,
    app_cache: web::Data<AppCache>,
    user_repo: web::Data<UserRepo>,
) -> Result<HttpResponse, ApplicationError> {
    let user_data = new_user_data.into_inner();
    let new_user = NewUser::new_user(
        user_data.email,
        user_data.password,
        user_data.username,
        &app_cache.roles,
    )?;

    user_repo.add_user(new_user).await?;

    Ok(HttpResponse::Ok().finish())
}
