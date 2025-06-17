use actix_web::{
    Error, FromRequest, HttpRequest, HttpResponse, ResponseError,
    dev::Payload,
    http::{StatusCode, header::AUTHORIZATION},
    web,
};
use chrono::{Duration, Utc};
use core::fmt;
use jsonwebtoken::{Header, errors::Error as JwtError};
use jsonwebtoken::{Validation, decode, encode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::future::{Ready, ready};
use uuid::Uuid;

use crate::{core::keys::Keys, domain::user::Role};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub role: Role,
}

#[derive(Debug)]
pub enum AuthError {
    MissingAuth,
    InvalidToken,
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            AuthError::MissingAuth => "Authorization header is missing",
            AuthError::InvalidToken => "Invalid or expired token",
        };
        write!(f, "{}", msg)
    }
}

impl ResponseError for AuthError {
    fn status_code(&self) -> StatusCode {
        match self {
            AuthError::MissingAuth => StatusCode::UNAUTHORIZED,
            AuthError::InvalidToken => StatusCode::FORBIDDEN,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let msg = self.to_string();
        HttpResponse::build(self.status_code()).json(json!({"error": msg}))
    }
}

impl FromRequest for Claims {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let keys_data = req
            .app_data::<web::Data<Keys>>()
            .map(|k| k.get_ref().clone());

        let token_opt = req
            .headers()
            .get(AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer ").map(str::to_owned));

        // Validate the presence of settings and header token
        let (keys, token) = match (keys_data, token_opt) {
            (Some(k), Some(t)) => (k, t),
            _ => return ready(Err(AuthError::MissingAuth.into())),
        };

        // Decode and validate the JWT token
        let res = decode::<Claims>(&token, &keys.decoding, &Validation::default())
            .map(|data| data.claims)
            .map_err(|_| AuthError::InvalidToken.into());

        ready(res)
    }
}

pub fn create_jwt_token(user_id: Uuid, role: Role, keys: &Keys) -> Result<String, JwtError> {
    let exp = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("Failed to calculate expiration time")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp,
        role: role,
    };

    encode(&Header::default(), &claims, &keys.encoding)
}
