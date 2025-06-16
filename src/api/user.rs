use crate::{
    api::error::ApiError,
    core::server::AppState,
    domain::{
        error::DomainError,
        repository::UserRepository,
        user::{NewUser, UpdateUser},
        validation::{PasswordRequirements, validate_password},
    },
    infra::sqlite_user_repo::{SqliteUserRepo, hash_password},
};
use actix_web::{HttpResponse, web};
use uuid::Uuid;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::get().to(list_users::<SqliteUserRepo>))
            .route("", web::post().to(create_user::<SqliteUserRepo>))
            .route("/{id}", web::get().to(get_user::<SqliteUserRepo>))
            .route("/{id}", web::patch().to(update_user::<SqliteUserRepo>))
            .route("/{id}", web::delete().to(delete_user::<SqliteUserRepo>)),
    );
}

async fn list_users<UserRepository>(state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let users = state
        .user_repo
        .list()
        .await
        .map_err(|_| ApiError::InternalError)?;

    Ok(HttpResponse::Ok().json(users))
}

async fn get_user<UserRepository>(
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

async fn create_user<UserRepository>(
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

async fn update_user<UserRepository>(
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

async fn delete_user<UserRepository>(
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
