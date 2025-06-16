use crate::{
    api::error::ApiError,
    core::server::AppState,
    domain::{
        error::DomainError,
        post::{NewPost, UpdatePost},
        repository::PostRepository,
    },
    infra::sqlite_post_repo::SqlitePostRepo,
};
use actix_web::{HttpResponse, web};
use uuid::Uuid;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/posts")
            .route("", web::get().to(list_posts::<SqlitePostRepo>))
            .route("", web::post().to(create_post::<SqlitePostRepo>))
            .route("/{id}", web::get().to(get_post::<SqlitePostRepo>))
            .route("/{id}", web::patch().to(update_post::<SqlitePostRepo>))
            .route("/{id}", web::delete().to(delete_post::<SqlitePostRepo>)),
    );
}

async fn list_posts<R: PostRepository>(
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let posts = state
        .repo
        .list()
        .await
        .map_err(|_| ApiError::InternalError)?;

    Ok(HttpResponse::Ok().json(posts))
}

async fn get_post<R: PostRepository>(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let id = id.into_inner();

    match state.repo.find_by_id(id).await {
        Ok(Some(post)) => Ok(HttpResponse::Ok().json(post)),
        Ok(None) => Err(ApiError::NotFound),
        Err(_) => Err(ApiError::InternalError),
    }
}

async fn create_post<R: PostRepository>(
    state: web::Data<AppState>,
    json: web::Json<NewPost>,
) -> Result<HttpResponse, ApiError> {
    let new = json.into_inner();
    state
        .repo
        .create(new)
        .await
        .map(|post| HttpResponse::Created().json(post))
        .map_err(|e| ApiError::BadRequest(e.to_string()))
}

async fn update_post<R: PostRepository>(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    json: web::Json<UpdatePost>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();

    let mut post = state
        .repo
        .find_by_id(id)
        .await
        .map_err(|_| ApiError::InternalError)?
        .ok_or(ApiError::NotFound)?;

    if let Some(title) = &json.title {
        post.title = title.clone();
    }

    if let Some(content) = &json.content {
        post.content = content.clone();
    }

    if let Some(published) = json.published {
        post.published = published;
    }

    let updated = state
        .repo
        .update(post)
        .await
        .map_err(|_| ApiError::InternalError)?;

    Ok(HttpResponse::Ok().json(updated))
}

async fn delete_post<R: PostRepository>(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let id = id.into_inner();

    match state.repo.delete(id).await {
        Ok(()) => Ok(HttpResponse::NoContent().finish()),
        Err(DomainError::NotFound) => Err(ApiError::NotFound),
        Err(_) => Err(ApiError::InternalError),
    }
}
