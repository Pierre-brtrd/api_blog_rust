use crate::core::jwt_middleware::JwtMiddleware;
use crate::infra::sqlite_post_repo::SqlitePostRepo;
use crate::{
    api::error::ApiError,
    domain::{
        error::DomainError,
        post::{NewPost, Post, UpdatePost},
        repository::PostRepository,
    },
};
use actix_web::{HttpResponse, web};
use uuid::Uuid;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/posts")
            .wrap(JwtMiddleware::new())
            .route("", web::get().to(list_posts))
            .route("", web::post().to(create_post))
            .route("/{id}", web::get().to(get_post))
            .route("/{id}", web::patch().to(update_post))
            .route("/{id}", web::delete().to(delete_post)),
    );
}

async fn list_posts(repo: web::Data<SqlitePostRepo>) -> Result<HttpResponse, ApiError> {
    let posts = repo.list().await.map_err(|_| ApiError::InternalError)?;

    Ok(HttpResponse::Ok().json(posts))
}

async fn get_post(
    repo: web::Data<SqlitePostRepo>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let id = id.into_inner();

    match repo.find_by_id(id).await {
        Ok(Some(post)) => Ok(HttpResponse::Ok().json(post)),
        Ok(None) => Err(ApiError::NotFound),
        Err(_) => Err(ApiError::InternalError),
    }
}

async fn create_post(
    repo: web::Data<SqlitePostRepo>,
    json: web::Json<NewPost>,
) -> Result<HttpResponse, ApiError> {
    let new = json.into_inner();
    new.validate_post()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    repo.create(new)
        .await
        .map(|post| HttpResponse::Created().json(post))
        .map_err(|e| ApiError::BadRequest(e.to_string()))
}

async fn update_post(
    repo: web::Data<SqlitePostRepo>,
    path: web::Path<Uuid>,
    json: web::Json<UpdatePost>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();

    let mut post_to_update = repo
        .find_by_id(id)
        .await
        .map_err(|_| ApiError::InternalError)?
        .ok_or(ApiError::NotFound)?;

    let update = json.into_inner();
    update
        .validate_post()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    if let Some(title) = &update.title {
        post_to_update.title = title.clone();
    }

    if let Some(content) = &update.content {
        post_to_update.content = content.clone();
    }

    if let Some(published) = update.published {
        post_to_update.published = published;
    }

    if let Some(user_id) = update.user_id {
        post_to_update.author.id = user_id;
    }

    let post = Post {
        id: post_to_update.id,
        user_id: post_to_update.author.id,
        title: post_to_update.title,
        content: post_to_update.content,
        published: post_to_update.published,
        created_at: post_to_update.created_at,
        updated_at: Some(chrono::Utc::now()),
    };

    let updated = repo
        .update(post)
        .await
        .map_err(|_| ApiError::InternalError)?;

    Ok(HttpResponse::Ok().json(updated))
}

async fn delete_post(
    repo: web::Data<SqlitePostRepo>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let id = id.into_inner();

    match repo.delete(id).await {
        Ok(()) => Ok(HttpResponse::NoContent().finish()),
        Err(DomainError::NotFound) => Err(ApiError::NotFound),
        Err(_) => Err(ApiError::InternalError),
    }
}
