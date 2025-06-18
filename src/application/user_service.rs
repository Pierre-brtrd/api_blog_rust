use chrono::Utc;
use uuid::Uuid;

use crate::{
    domain::{
        error::DomainError,
        model::user::{Role, User},
        repository::UserRepository,
    },
    infrastructure::{
        auth::{
            create_jwt_token,
            password::{hash_password, verify_password},
        },
        security::keys::Keys,
    },
    interfaces::api::dto::user::UpdateUserPayload,
};

#[derive(Clone)]
pub struct UserService<R> {
    repo: R,
    keys: Keys,
}

impl<R> UserService<R>
where
    R: UserRepository + Send + Sync,
{
    pub fn new(repo: R, keys: Keys) -> Self {
        UserService { repo, keys }
    }

    pub async fn list(&self) -> Result<Vec<User>, DomainError> {
        self.repo.list().await
    }

    pub async fn find_by_id(&self, id: uuid::Uuid) -> Result<Option<User>, DomainError> {
        self.repo.find_by_id(id).await
    }

    pub async fn login(&self, username: &str, password: &str) -> Result<String, DomainError> {
        let user: User = self
            .repo
            .find_by_username(username)
            .await?
            .ok_or(DomainError::Unauthorized("Invalid credentials".to_string()))?;

        let valid = verify_password(password, &user.password_hash)
            .map_err(|_| DomainError::Unauthorized("Invalid credentials".to_string()))?;

        if !valid {
            return Err(DomainError::Unauthorized("Invalid credentials".to_string()));
        }

        let token = create_jwt_token(user.id, user.role.clone(), &self.keys)
            .map_err(|_| DomainError::InternalError)?;

        Ok(token)
    }

    pub async fn create_user(
        &self,
        username: String,
        password: String,
        email: String,
    ) -> Result<User, DomainError> {
        let hashed_password = hash_password(&password).map_err(|_| DomainError::InternalError)?;

        let user = User {
            id: uuid::Uuid::new_v4(),
            username,
            password_hash: hashed_password,
            email,
            role: Role::User,
            created_at: Utc::now(),
            updated_at: None,
        };

        self.repo.create(user.clone()).await?;

        Ok(user)
    }

    pub async fn delete(&self, id: uuid::Uuid) -> Result<(), DomainError> {
        self.repo.delete(id).await
    }

    pub async fn update(
        &self,
        user_id: Uuid,
        payload: UpdateUserPayload,
    ) -> Result<User, DomainError> {
        let mut user = self
            .find_by_id(user_id)
            .await?
            .ok_or(DomainError::NotFound)?;

        if let Some(u) = payload.username {
            user.username = u;
        }

        if let Some(e) = payload.email {
            user.email = e;
        }

        if let Some(raw_pwd) = payload.password {
            user.password_hash = hash_password(&raw_pwd).map_err(|_| DomainError::InternalError)?;
        }

        if let Some(r) = payload.role {
            user.role = r;
        }

        user.updated_at = Some(Utc::now());

        let updated = self.repo.update(user).await?;

        Ok(updated)
    }
}
