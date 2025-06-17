use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::Serialize;
use thiserror::Error;

use crate::domain::error::DomainError;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Ressource not found")]
    NotFound,
    #[error("{0}")]
    BadRequest(String),
    #[error("Internal server error")]
    InternalError,
    #[error("{0}")]
    Unauthorized(String),
}

impl From<DomainError> for ApiError {
    fn from(err: DomainError) -> Self {
        match err {
            DomainError::NotFound => ApiError::NotFound,
            DomainError::InternalError => ApiError::InternalError,
            DomainError::Unauthorized(msg) => ApiError::Unauthorized(msg),
            DomainError::DatabaseError(_) => ApiError::InternalError,
            DomainError::DuplicateEmail => ApiError::BadRequest("Email already exists".to_string()),
            DomainError::EmptyContent => {
                ApiError::BadRequest("Content cannot be empty".to_string())
            }
            DomainError::InvalidMaxLentgthTitle => {
                ApiError::BadRequest("Title exceeds maximum length".to_string())
            }
            DomainError::InvalidMinLentgthTitle => {
                ApiError::BadRequest("Title is too short".to_string())
            }
            DomainError::InvalidUserId => ApiError::BadRequest("Invalid user ID".to_string()),
            DomainError::PasswordHashingError(_) => ApiError::InternalError,
        }
    }
}

#[derive(Serialize)]
struct ApiErrorResponse {
    error: String,
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match *self {
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let body = ApiErrorResponse {
            error: self.to_string(),
        };

        HttpResponse::build(self.status_code()).json(body)
    }
}
