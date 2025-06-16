use crate::domain::{
    error::DomainError,
    post::{NewPost, Post, PostWithAuthor},
    user::{NewUser, User},
};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait PostRepository {
    async fn list(&self) -> Result<Vec<PostWithAuthor>, anyhow::Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<PostWithAuthor>, anyhow::Error>;
    async fn create(&self, new_post: NewPost) -> Result<Post, anyhow::Error>;
    async fn update(&self, post: Post) -> Result<Post, anyhow::Error>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
}

#[async_trait]
pub trait UserRepository {
    async fn list(&self) -> Result<Vec<User>, anyhow::Error>;
    async fn create(&self, new_user: NewUser) -> Result<User, anyhow::Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, anyhow::Error>;
    async fn update(&self, user: User) -> Result<User, anyhow::Error>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, anyhow::Error>;
}
