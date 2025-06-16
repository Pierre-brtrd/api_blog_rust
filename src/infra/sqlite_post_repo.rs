use crate::domain::{
    error::DomainError,
    post::{NewPost, Post},
    repository::PostRepository,
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Clone)]
pub struct SqlitePostRepo {
    pool: SqlitePool,
}

impl SqlitePostRepo {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PostRepository for SqlitePostRepo {
    async fn list(&self) -> Result<Vec<Post>, anyhow::Error> {
        let rows = sqlx::query_as!(
            Post,
            r#"
            SELECT id as "id: Uuid", title, content, published as "published: bool", created_at as "created_at: DateTime<Utc>", updated_at as "updated_at: DateTime<Utc>"
            FROM posts
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Post>, anyhow::Error> {
        let row = sqlx::query_as!(
            Post,
            r#"
            SELECT id as "id: Uuid", title, content, published as "published: bool", created_at as "created_at: DateTime<Utc>", updated_at as "updated_at: DateTime<Utc>"
            FROM posts
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row)
    }

    async fn create(&self, new_post: NewPost) -> Result<Post, anyhow::Error> {
        // Verifications
        if new_post.title.trim().is_empty() {
            return Err(DomainError::EmptyContent.into());
        }

        if new_post.title.trim().len() < 3 {
            return Err(DomainError::InvalidMinLentgthTitle.into());
        }

        if new_post.title.trim().len() > 255 {
            return Err(DomainError::InvalidMaxLentgthTitle.into());
        }

        if new_post.content.trim().is_empty() {
            return Err(DomainError::EmptyContent.into());
        }

        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query_as!(
            Post,
            r#"
            INSERT INTO posts (id, title, content, published, created_at)
            VALUES (?, ?, ?, ?, ?)
            "#,
            id,
            new_post.title,
            new_post.content,
            new_post.published,
            now
        )
        .execute(&self.pool)
        .await?;

        Ok(Post {
            id,
            title: new_post.title,
            content: new_post.content,
            published: new_post.published,
            created_at: now,
            updated_at: None,
        })
    }

    async fn update(&self, post: Post) -> Result<Post, anyhow::Error> {
        let now = Utc::now();

        sqlx::query!(
            r#"
            UPDATE posts SET
                title = ?,
                content = ?,
                published = ?,
                updated_at = ?
            WHERE id = ?
            "#,
            post.title,
            post.content,
            post.published,
            now,
            post.id
        )
        .execute(&self.pool)
        .await?;

        Ok(Post {
            updated_at: Some(now),
            ..post
        })
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        let result = sqlx::query!("DELETE FROM posts WHERE id = ?", id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            Err(DomainError::NotFound)
        } else {
            Ok(())
        }
    }
}
