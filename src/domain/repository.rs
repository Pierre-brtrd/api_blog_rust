use crate::domain::{
    error::DomainError,
    post::{NewPost, Post},
};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait PostRepository {
    async fn list(&self) -> Result<Vec<Post>, anyhow::Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Post>, anyhow::Error>;
    async fn create(&self, new_post: NewPost) -> Result<Post, anyhow::Error>;
    async fn update(&self, post: Post) -> Result<Post, anyhow::Error>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
}
