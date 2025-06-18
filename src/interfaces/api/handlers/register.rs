use actix_web::{HttpResponse, web};

use crate::{
    application::user_service::UserService,
    infrastructure::persistence::sqlite::user_repo::SqliteUserRepo,
    interfaces::api::{
        dto::user::{NewUser, UserPublic},
        error::ApiError,
    },
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/register").route("", web::post().to(register)));
}

async fn register(
    dto: web::Json<NewUser>,
    service: web::Data<UserService<SqliteUserRepo>>,
) -> Result<HttpResponse, ApiError> {
    let (username, password, email) = dto.into_inner().validate_and_into_domain()?;

    let user = service
        .create_user(username, password, email)
        .await
        .map_err(ApiError::from)?;

    let public = UserPublic::from(user);

    Ok(HttpResponse::Created().json(public))
}
