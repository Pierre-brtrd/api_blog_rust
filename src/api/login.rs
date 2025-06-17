use crate::{
    api::error::ApiError,
    core::{auth::create_jwt_token, keys::Keys},
    domain::{
        repository::UserRepository,
        user::{LoginUser, RawLoginRequest},
    },
    infra::sqlite_user_repo::{SqliteUserRepo, verify_password},
};
use actix_web::{HttpResponse, web};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/login").route("", web::post().to(login)));
}

async fn login(
    raw: web::Json<RawLoginRequest>,
    repo: web::Data<SqliteUserRepo>,
    keys: web::Data<Keys>,
) -> Result<HttpResponse, ApiError> {
    raw.validate_login()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let login: LoginUser = raw.into_inner().try_into()?;

    let user = repo
        .find_by_username(&login.username)
        .await
        .map_err(|_| ApiError::InternalError)?
        .ok_or(ApiError::Unauthorized("Invalid credentials".to_string()))?;

    let is_valid = verify_password(&login.password, &user.password_hash)
        .map_err(|_| ApiError::InternalError)?;
    if !is_valid {
        return Err(ApiError::Unauthorized("Invalid credentials".to_string()));
    }

    let token = create_jwt_token(user.id, user.role, &keys).map_err(|_| ApiError::InternalError)?;

    Ok(HttpResponse::Ok().json(serde_json::json!({ "token": token })))
}
