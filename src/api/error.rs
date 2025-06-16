use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::Serialize;
use thiserror::Error;

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
