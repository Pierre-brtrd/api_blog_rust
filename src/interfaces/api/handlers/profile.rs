use crate::{
    application::user_service::UserService,
    infrastructure::{auth::Claims, persistence::sqlite::user_repo::SqliteUserRepo},
    interfaces::api::{
        dto::user::{UpdateProfile, UpdateUserPayload, UserPublic},
        error::ApiError,
    },
};
use actix_web::{HttpResponse, web};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/profile")
            .route("", web::get().to(get_profile))
            .route("", web::patch().to(update_profile)),
    );
}

async fn get_profile(
    claims: Claims,
    service: web::Data<UserService<SqliteUserRepo>>,
) -> Result<HttpResponse, ApiError> {
    let id = claims.user_id()?;

    let user = service.find_by_id(id).await?.ok_or(ApiError::NotFound)?;

    Ok(HttpResponse::Ok().json(UserPublic::from(user)))
}

async fn update_profile(
    claims: Claims,
    dto: web::Json<UpdateProfile>,
    service: web::Data<UserService<SqliteUserRepo>>,
) -> Result<HttpResponse, ApiError> {
    let id = claims.user_id()?;

    let payload: UpdateUserPayload = dto.into_inner().validate_and_into_domain()?;

    let updated = service.update(id, payload).await.map_err(ApiError::from)?;

    Ok(HttpResponse::Ok().json(updated))
}
