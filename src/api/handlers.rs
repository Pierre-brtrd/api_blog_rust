use crate::{
    api::error::ApiError,
    core::server::AppState,
    domain::{
        error::DomainError,
        post::{NewPost, Post, UpdatePost},
        repository::{PostRepository, UserRepository},
        user::{NewUser, UpdateUser},
        validation::{PasswordRequirements, validate_password},
    },
    infra::{
        sqlite_post_repo::SqlitePostRepo,
        sqlite_user_repo::{SqliteUserRepo, hash_password},
    },
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
    )
    .service(
        web::scope("/users")
            .route("", web::get().to(list_users::<SqliteUserRepo>))
            .route("", web::post().to(create_user::<SqliteUserRepo>))
            .route("/{id}", web::get().to(get_user::<SqliteUserRepo>))
            .route("/{id}", web::patch().to(update_user::<SqliteUserRepo>))
            .route("/{id}", web::delete().to(delete_user::<SqliteUserRepo>)),
    );
}

async fn list_posts<R: PostRepository>(
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let posts = state
        .post_repo
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

    match state.post_repo.find_by_id(id).await {
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
    new.validate_post()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    state
        .post_repo
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

    let mut post_to_update = state
        .post_repo
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

    let updated = state
        .post_repo
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

    match state.post_repo.delete(id).await {
        Ok(()) => Ok(HttpResponse::NoContent().finish()),
        Err(DomainError::NotFound) => Err(ApiError::NotFound),
        Err(_) => Err(ApiError::InternalError),
    }
}

async fn list_users<R: UserRepository>(
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let users = state
        .user_repo
        .list()
        .await
        .map_err(|_| ApiError::InternalError)?;

    Ok(HttpResponse::Ok().json(users))
}

async fn get_user<R: UserRepository>(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let id = id.into_inner();

    match state.user_repo.find_by_id(id).await {
        Ok(Some(user)) => Ok(HttpResponse::Ok().json(user)),
        Ok(None) => Err(ApiError::NotFound),
        Err(_) => Err(ApiError::InternalError),
    }
}

async fn create_user<R: UserRepository>(
    state: web::Data<AppState>,
    json: web::Json<NewUser>,
) -> Result<HttpResponse, ApiError> {
    let new = json.into_inner();

    new.validate_user()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    state
        .user_repo
        .create(new)
        .await
        .map(|user| HttpResponse::Created().json(user))
        .map_err(|e| {
            ApiError::BadRequest(DomainError::PasswordHashingError(e.to_string()).to_string())
        })
}

async fn update_user<R: UserRepository>(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    json: web::Json<UpdateUser>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();

    let mut user = state
        .user_repo
        .find_by_id(id)
        .await
        .map_err(|_| ApiError::InternalError)?
        .ok_or(ApiError::NotFound)?;

    let update = json.into_inner();

    update
        .validate_user()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    if let Some(username) = &update.username {
        user.username = username.clone();
    }

    if let Some(email) = &update.email {
        user.email = email.clone();
    }

    if let Some(password) = &update.password {
        validate_password(password, &PasswordRequirements::default())
            .map_err(|e| ApiError::BadRequest(e.to_string()))?;

        let hashed = hash_password(password).map_err(|e| ApiError::BadRequest(e.to_string()))?;

        user.password_hash = hashed;
    }

    let updated = state
        .user_repo
        .update(user)
        .await
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    Ok(HttpResponse::Ok().json(updated))
}

async fn delete_user<R: UserRepository>(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let id = id.into_inner();

    match state.user_repo.delete(id).await {
        Ok(()) => Ok(HttpResponse::NoContent().finish()),
        Err(DomainError::NotFound) => Err(ApiError::NotFound),
        Err(_) => Err(ApiError::InternalError),
    }
}
