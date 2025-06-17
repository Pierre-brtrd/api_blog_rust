use crate::{
    domain::validation::{PasswordRequirements, validate_password},
    interfaces::api::error::ApiError,
};
use validator::Validate;

pub fn validate_dto<T: Validate>(dto: &T) -> Result<(), ApiError> {
    dto.validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))
}

pub fn require_field<T>(opt: Option<T>, name: &str) -> Result<T, ApiError> {
    opt.ok_or_else(|| ApiError::BadRequest(format!(r#"{} is required"#, name)))
}

pub fn require_password(opt: Option<String>) -> Result<String, ApiError> {
    let pwd = require_field(opt, "password")?;
    validate_password(&pwd, &PasswordRequirements::default())
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    Ok(pwd)
}
