use crate::{
    application::user_service::UserService,
    domain::error::DomainError,
    infrastructure::persistence::sqlite::user_repo::SqliteUserRepo,
    interfaces::api::{
        dto::user::{LoginUser, RawLoginRequest},
        error::ApiError,
    },
};
use actix_web::{HttpResponse, web};
use serde_json::json;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/login").route("", web::post().to(login)));
}

pub async fn login(
    raw: web::Json<RawLoginRequest>,
    service: web::Data<UserService<SqliteUserRepo>>,
) -> Result<HttpResponse, ApiError> {
    raw.validate_login()?;

    let LoginUser { username, password } = raw.into_inner().try_into()?;

    let token = service
        .login(&username, &password)
        .await
        .map_err(|e: DomainError| ApiError::from(e))?;

    Ok(HttpResponse::Ok().json(json!({ "token": token })))
}
