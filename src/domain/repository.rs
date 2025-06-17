use crate::domain::{
    error::DomainError,
    model::{
        post::{Post, PostWithAuthor},
        user::User,
    },
};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait PostRepository {
    async fn list(&self) -> Result<Vec<PostWithAuthor>, DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<PostWithAuthor>, DomainError>;
    async fn create(&self, new_post: Post) -> Result<Post, DomainError>;
    async fn update(&self, post: Post) -> Result<Post, DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
}

#[async_trait]
pub trait UserRepository {
    async fn list(&self) -> Result<Vec<User>, DomainError>;
    async fn create(&self, user: User) -> Result<User, DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError>;
    async fn update(&self, user: User) -> Result<User, DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, DomainError>;
}
