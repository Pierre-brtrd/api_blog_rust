use crate::{
    domain::{
        error::DomainError,
        model::post::{Post, PostWithAuthor},
        repository::PostRepository,
    },
    interfaces::api::dto::user::UserPublic,
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
    async fn list(&self) -> Result<Vec<PostWithAuthor>, DomainError> {
        let rows = sqlx::query!(
            r#"
                SELECT 
                p.id as "post_id: Uuid",
                p.title,
                p.content,
                p.published as "published: bool",
                p.created_at as "created_at: DateTime<Utc>",
                p.updated_at as "updated_at: DateTime<Utc>",

                u.id as "user_id: Uuid",
                u.username,
                u.email,
                u.created_at as "user_created_at: DateTime<Utc>"
                FROM posts p
                JOIN users u ON p.user_id = u.id
                ORDER BY p.created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let posts = rows
            .into_iter()
            .map(|row| PostWithAuthor {
                id: row.post_id.expect("Post ID is required"),
                title: row.title.expect("Title is required"),
                content: row.content.expect("Content is required"),
                published: row.published.expect("Published status is required"),
                created_at: row.created_at.expect("Created at is required"),
                updated_at: row.updated_at,
                author: UserPublic {
                    id: row.user_id,
                    username: row.username.clone(),
                    email: row.email.clone(),
                    created_at: row.user_created_at,
                },
            })
            .collect();

        Ok(posts)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<PostWithAuthor>, DomainError> {
        let row = sqlx::query!(
            r#"
            SELECT 
            p.id as "post_id: Uuid", 
            p.title, content, 
            p.published as "published: bool", 
            p.created_at as "created_at: DateTime<Utc>", 
            p.updated_at as "updated_at: DateTime<Utc>",
            u.id as "user_id: Uuid",
            u.username,
            u.email,
            u.created_at as "user_created_at: DateTime<Utc>"
            FROM posts p
            JOIN users u ON p.user_id = u.id
            WHERE p.id = ?
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        let post_with_author = row.map(|row| PostWithAuthor {
            id: row.post_id,
            title: row.title,
            content: row.content,
            published: row.published,
            created_at: row.created_at,
            updated_at: row.updated_at,
            author: UserPublic {
                id: row.user_id,
                username: row.username.clone(),
                email: row.email.clone(),
                created_at: row.user_created_at,
            },
        });

        Ok(post_with_author)
    }

    async fn create(&self, new_post: Post) -> Result<Post, DomainError> {
        sqlx::query_as!(
            Post,
            r#"
            INSERT INTO posts (id, user_id, title, content, published, created_at)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
            new_post.id,
            new_post.user_id,
            new_post.title,
            new_post.content,
            new_post.published,
            new_post.created_at,
        )
        .execute(&self.pool)
        .await?;

        Ok(new_post)
    }

    async fn update(&self, post: Post) -> Result<Post, DomainError> {
        let now = Utc::now();

        sqlx::query!(
            r#"
            UPDATE posts SET
                user_id = ?,
                title = ?,
                content = ?,
                published = ?,
                updated_at = ?
            WHERE id = ?
            "#,
            post.user_id,
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
