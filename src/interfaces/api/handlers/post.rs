use std::str::FromStr;

use crate::application::post_service::PostService;
use crate::infrastructure::auth::jwt::JwtMiddleware;
use crate::infrastructure::persistence::sqlite::post_repo::SqlitePostRepo;
use crate::infrastructure::persistence::sqlite::user_repo::SqliteUserRepo;
use crate::interfaces::api::dto::post::{NewPost, UpdatePost};
use crate::{domain::error::DomainError, interfaces::api::error::ApiError};
use actix_web::{HttpResponse, web};
use uuid::Uuid;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/posts")
            .wrap(JwtMiddleware::new())
            .route("", web::get().to(list_posts))
            .route("", web::post().to(create_post))
            .route("/{id}", web::get().to(get_post))
            .route("/{id}", web::patch().to(update_post))
            .route("/{id}", web::delete().to(delete_post)),
    );
}

async fn list_posts(
    service: web::Data<PostService<SqlitePostRepo, SqliteUserRepo>>,
) -> Result<HttpResponse, ApiError> {
    let posts = service.list().await.map_err(ApiError::from)?;

    Ok(HttpResponse::Ok().json(posts))
}

async fn get_post(
    service: web::Data<PostService<SqlitePostRepo, SqliteUserRepo>>,
    id: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let id = Uuid::from_str(&id.into_inner())
        .map_err(|_| ApiError::BadRequest("Invalid UUID format".to_string()))?;

    match service.find_by_id(id).await {
        Ok(Some(post)) => Ok(HttpResponse::Ok().json(post)),
        Ok(None) => Err(ApiError::NotFound),
        Err(_) => Err(ApiError::InternalError),
    }
}

async fn create_post(
    service: web::Data<PostService<SqlitePostRepo, SqliteUserRepo>>,
    dto: web::Json<NewPost>,
) -> Result<HttpResponse, ApiError> {
    let (title, content, published, user_id) = dto.into_inner().validate_and_into_domain()?;

    let post = service
        .create(title, content, published, user_id)
        .await
        .map_err(ApiError::from)?;

    Ok(HttpResponse::Created().json(post))
}

async fn update_post(
    service: web::Data<PostService<SqlitePostRepo, SqliteUserRepo>>,
    path: web::Path<String>,
    dto: web::Json<UpdatePost>,
) -> Result<HttpResponse, ApiError> {
    let id = Uuid::from_str(&path.into_inner())
        .map_err(|_| ApiError::BadRequest("Invalid UUID format".to_string()))?;

    let payload = dto.into_inner().validate_and_into_domain()?;

    let updated = service.update(id, payload).await.map_err(ApiError::from)?;

    Ok(HttpResponse::Ok().json(updated))
}

async fn delete_post(
    service: web::Data<PostService<SqlitePostRepo, SqliteUserRepo>>,
    id: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let id = Uuid::from_str(&id.into_inner())
        .map_err(|_| ApiError::BadRequest("Invalid UUID format".to_string()))?;

    match service.delete(id).await {
        Ok(()) => Ok(HttpResponse::NoContent().finish()),
        Err(DomainError::NotFound) => Err(ApiError::NotFound),
        Err(_) => Err(ApiError::InternalError),
    }
}
