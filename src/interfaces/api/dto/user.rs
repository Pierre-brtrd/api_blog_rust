use crate::{
    domain::model::user::{Role, User},
    interfaces::api::{
        error::ApiError,
        validation::{require_field, require_password, validate_dto},
    },
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize)]
pub struct UserPublic {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserPublic {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: user.created_at,
        }
    }
}

#[derive(Debug, Deserialize, Validate, Clone)]
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
    #[validate(required(message = "Confirm Password obligatoire"))]
    pub confirm_password: Option<String>,
    #[validate(
        email(message = "Email Invalide"),
        required(message = "Email obligatoire")
    )]
    pub email: Option<String>,
}

impl NewUser {
    pub fn validate_user(&self) -> Result<(), ApiError> {
        validate_dto(self)?;

        if self.password != self.confirm_password {
            return Err(ApiError::BadRequest("Passwords do not match".to_string()));
        }

        let _pwd = require_password(self.password.clone())?;

        Ok(())
    }

    pub fn validate_and_into_domain(self) -> Result<(String, String, String), ApiError> {
        validate_dto(&self)?;

        let username = require_field(self.username, "username")?;
        let email = require_field(self.email, "email")?;

        if self.password != self.confirm_password {
            return Err(ApiError::BadRequest("Passwords must not match".to_string()));
        }

        let password = require_password(self.password)?;

        Ok((username, password, email))
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
    pub role: Option<Role>,
}

pub struct UpdateUserPayload {
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub role: Option<Role>,
}

impl UpdateUser {
    pub fn validate_user(&self) -> Result<(), ApiError> {
        validate_dto(self)?;

        if self.password.is_some() {
            let _pwd = require_password(self.password.clone())?;
        }

        Ok(())
    }

    pub fn validate_and_into_domain(self) -> Result<UpdateUserPayload, ApiError> {
        validate_dto(&self)?;

        let password = if self.password.is_some() {
            Some(require_password(self.password.clone())?)
        } else {
            None
        };

        Ok(UpdateUserPayload {
            username: self.username,
            password,
            email: self.email,
            role: self.role,
        })
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProfile {
    #[validate(length(
        min = 3,
        max = 255,
        message = "Username must be between 3 and 255 characters"
    ))]
    pub username: Option<String>,
    #[validate(email(message = "Email Invalide"))]
    pub email: Option<String>,
    pub plain_password: Option<String>,
    pub confirm_password: Option<String>,
}

impl UpdateProfile {
    pub fn validate_user(&self) -> Result<(), ApiError> {
        validate_dto(self)?;

        if self.plain_password.is_some() || self.confirm_password.is_some() {
            if self.plain_password != self.confirm_password {
                return Err(ApiError::BadRequest("Passwords do not match".to_string()));
            }
            let _pwd = require_password(self.plain_password.clone())?;
        }

        Ok(())
    }

    pub fn validate_and_into_domain(self) -> Result<UpdateUserPayload, ApiError> {
        validate_dto(&self)?;

        let password = if self.plain_password.is_some() {
            if self.plain_password != self.confirm_password {
                return Err(ApiError::BadRequest("Passwords do not match".to_string()));
            }

            Some(require_password(self.plain_password)?)
        } else {
            None
        };

        Ok(UpdateUserPayload {
            username: self.username,
            password,
            email: self.email,
            role: None,
        })
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
        validate_dto(self)?;
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
        Ok(Self {
            username: raw.username.unwrap(),
            password: raw.password.unwrap(),
        })
    }
}
