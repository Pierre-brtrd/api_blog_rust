use std::str::FromStr;

use crate::{
    application::user_service::UserService,
    domain::error::DomainError,
    infrastructure::{
        auth::{Claims, admin::AdminMiddleware, jwt::JwtMiddleware},
        persistence::sqlite::user_repo::SqliteUserRepo,
    },
    interfaces::api::{
        dto::user::{NewUser, UpdateProfile, UpdateUser, UpdateUserPayload, UserPublic},
        error::ApiError,
    },
};
use actix_web::{HttpResponse, web};
use uuid::Uuid;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/users")
            .wrap(JwtMiddleware::new())
            .wrap(AdminMiddleware::new())
            .route("", web::get().to(list_users))
            .route("", web::post().to(create_user))
            .route("/{id}", web::get().to(get_user))
            .route("/{id}", web::patch().to(update_user))
            .route("/{id}", web::delete().to(delete_user)),
    )
    .service(
        web::scope("/api/profile")
            .route("", web::get().to(get_profile))
            .route("", web::patch().to(update_profile)),
    );
}

async fn list_users(
    service: web::Data<UserService<SqliteUserRepo>>,
) -> Result<HttpResponse, ApiError> {
    let users = service.list().await.map_err(ApiError::from)?;

    Ok(HttpResponse::Ok().json(users))
}

async fn get_user(
    service: web::Data<UserService<SqliteUserRepo>>,
    id: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let id = id.into_inner();
    let id =
        Uuid::from_str(&id).map_err(|_| ApiError::BadRequest("Invalid UUID format".to_string()))?;

    match service.find_by_id(id).await {
        Ok(Some(user)) => Ok(HttpResponse::Ok().json(user)),
        Ok(None) => Err(ApiError::NotFound),
        Err(_) => Err(ApiError::InternalError),
    }
}

async fn create_user(
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

async fn update_user(
    path: web::Path<String>,
    dto: web::Json<UpdateUser>,
    service: web::Data<UserService<SqliteUserRepo>>,
) -> Result<HttpResponse, ApiError> {
    let id = Uuid::parse_str(&path.into_inner())
        .map_err(|_| ApiError::BadRequest("Invalid UUID format".to_string()))?;

    let payload = dto.into_inner().validate_and_into_domain()?;

    let updated = service.update(id, payload).await.map_err(ApiError::from)?;

    Ok(HttpResponse::Ok().json(updated))
}

async fn delete_user(
    service: web::Data<UserService<SqliteUserRepo>>,
    id: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let id = id.into_inner();
    let id =
        Uuid::from_str(&id).map_err(|_| ApiError::BadRequest("Invalid UUID format".to_string()))?;

    match service.delete(id).await {
        Ok(()) => Ok(HttpResponse::NoContent().finish()),
        Err(DomainError::NotFound) => Err(ApiError::NotFound),
        Err(_) => Err(ApiError::InternalError),
    }
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
