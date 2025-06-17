use chrono::Utc;
use uuid::Uuid;

use crate::{
    domain::{
        error::DomainError,
        model::post::{Post, PostWithAuthor},
        repository::{PostRepository, UserRepository},
    },
    interfaces::api::dto::post::UpdatePostPayload,
};

#[derive(Clone)]
pub struct PostService<R, UR> {
    repo: R,
    user_repo: UR,
}

impl<R, UR> PostService<R, UR>
where
    R: PostRepository + Send + Sync,
    UR: UserRepository + Send + Sync,
{
    pub fn new(repo: R, user_repo: UR) -> Self {
        Self { repo, user_repo }
    }

    pub async fn list(&self) -> Result<Vec<PostWithAuthor>, DomainError> {
        self.repo.list().await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<PostWithAuthor>, DomainError> {
        self.repo.find_by_id(id).await
    }

    pub async fn create(
        &self,
        title: String,
        content: String,
        published: bool,
        user_id: Uuid,
    ) -> Result<Post, DomainError> {
        let post = Post {
            id: Uuid::new_v4(),
            title,
            content,
            published,
            user_id,
            created_at: Utc::now(),
            updated_at: None,
        };

        self.repo.create(post.clone()).await?;

        Ok(post)
    }

    pub async fn update(
        &self,
        post_id: Uuid,
        payload: UpdatePostPayload,
    ) -> Result<Post, DomainError> {
        let mut post: Post = self
            .repo
            .find_by_id(post_id)
            .await?
            .ok_or(DomainError::NotFound)?
            .into();

        if let Some(title) = payload.title {
            post.title = title;
        }

        if let Some(content) = payload.content {
            post.content = content;
        }

        if let Some(published) = payload.published {
            post.published = published;
        }

        if let Some(user_id) = payload.user_id {
            self.user_repo
                .find_by_id(user_id)
                .await?
                .ok_or(DomainError::NotFound)?;
            post.user_id = user_id;
        }

        post.updated_at = Some(Utc::now());

        let updated = self.repo.update(post).await?;

        Ok(updated)
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        self.repo.delete(id).await
    }
}
