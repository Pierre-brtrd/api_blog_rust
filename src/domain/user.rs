use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::{
    api::error::ApiError,
    domain::validation::{PasswordRequirements, validate_password},
};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum Role {
    User,
    Admin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub role: Role,
}

#[derive(Debug, Serialize)]
pub struct UserPublic {
    pub id: Uuid,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct NewUser {
    #[validate(
        length(
            min = 3,
            max = 50,
            message = "Username must be between 3 and 50 characters"
        ),
        required(message = "Username obligatoire")
    )]
    pub username: Option<String>,
    #[validate(required(message = "Password obligatoire"))]
    pub password: Option<String>,
    #[validate(
        email(message = "Email Invalide"),
        required(message = "Email obligatoire")
    )]
    pub email: Option<String>,
}

impl NewUser {
    pub fn validate_user(&self) -> Result<(), ApiError> {
        let password = self
            .password
            .as_deref()
            .ok_or_else(|| ApiError::BadRequest("Password is required".to_string()))?;

        validate_password(password, &PasswordRequirements::default())
            .map_err(|e| ApiError::BadRequest(e.to_string()))?;

        self.validate()
            .map_err(|e| ApiError::BadRequest(e.to_string()))?;

        Ok(())
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUser {
    #[validate(length(
        min = 3,
        max = 50,
        message = "Username must be between 3 and 50 characters"
    ))]
    pub username: Option<String>,
    pub password: Option<String>,
    #[validate(email(message = "Email Invalide"))]
    pub email: Option<String>,
}

impl UpdateUser {
    pub fn validate_user(&self) -> Result<(), ApiError> {
        if let Some(password) = &self.password {
            validate_password(password, &PasswordRequirements::default())
                .map_err(|e| ApiError::BadRequest(e.to_string()))?;
        }

        self.validate()
            .map_err(|e| ApiError::BadRequest(e.to_string()))?;

        Ok(())
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct RawLoginRequest {
    #[validate(required(message = "Username obligatoire"))]
    pub username: Option<String>,
    #[validate(required(message = "Password obligatoire"))]
    pub password: Option<String>,
}

impl RawLoginRequest {
    pub fn validate_login(&self) -> Result<(), ApiError> {
        self.validate()
            .map_err(|e| ApiError::BadRequest(e.to_string()))?;
        Ok(())
    }
}

pub struct LoginUser {
    pub username: String,
    pub password: String,
}

impl TryFrom<RawLoginRequest> for LoginUser {
    type Error = ApiError;

    fn try_from(raw: RawLoginRequest) -> Result<Self, ApiError> {
        Ok(LoginUser {
            username: raw.username.unwrap(),
            password: raw.password.unwrap(),
        })
    }
}
