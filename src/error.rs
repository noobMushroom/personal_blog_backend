use actix_web::{
    HttpResponse, ResponseError,
    http::{StatusCode, header::ContentType},
};
use thiserror::Error;

use crate::domain::error::DomainError;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("Internal Server Error")]
    InternalServerError,

    #[error("{0}")]
    BadRequest(String),
}

#[derive(serde::Serialize)]
pub struct ErrorResponse<'a> {
    pub error: &'a str,
}

impl From<DomainError> for ApplicationError {
    fn from(value: DomainError) -> Self {
        match value {
            DomainError::InvalidUsername(msg)
            | DomainError::InvalidPassword(msg)
            | DomainError::InvalidEmail(msg) => ApplicationError::BadRequest(msg),
            DomainError::Internal(msg)=> { 
                tracing::error!("Internal domain error: {}", msg);
                ApplicationError::InternalServerError},
        }
    }
}

impl ResponseError for ApplicationError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            ApplicationError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ApplicationError::BadRequest(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let mut builder = HttpResponse::build(self.status_code());

        match self {
            ApplicationError::BadRequest(msg) => builder
                .insert_header(ContentType::json())
                .json(ErrorResponse { error: msg }),

            ApplicationError::InternalServerError => builder
                .insert_header(ContentType::json())
                .json(ErrorResponse {
                    error: "Internal Server Error",
                }),
        }
    }
}
