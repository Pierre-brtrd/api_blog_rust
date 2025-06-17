use crate::domain::{
    error::DomainError,
    model::user::{Role, User},
    repository::UserRepository,
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Clone)]
pub struct SqliteUserRepo {
    pool: SqlitePool,
}

impl SqliteUserRepo {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for SqliteUserRepo {
    async fn list(&self) -> Result<Vec<User>, DomainError> {
        let rows = sqlx::query_as!(
            User,
            r#"
            SELECT id as "id: Uuid", username, role as "role: Role", password_hash, email, created_at as "created_at: DateTime<Utc>", updated_at as "updated_at: DateTime<Utc>"
            FROM users
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    async fn create(&self, user: User) -> Result<User, DomainError> {
        let res = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, username, role, password_hash, email, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            RETURNING id as "id: Uuid", username, email, role as "role: Role", password_hash, created_at as "created_at: DateTime<Utc>", updated_at as "updated_at: DateTime<Utc>"
            "#,
            user.id,
            user.username,
            user.role,
            user.password_hash,
            user.email,
            user.created_at,
            user.updated_at,
        )
        .fetch_one(&self.pool)
        .await;

        match res {
            Ok(user) => Ok(user),
            Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
                Err(DomainError::DuplicateEmail)
            }
            Err(e) => Err(e.into()),
        }
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id as "id: Uuid", username, role as "role: Role",password_hash, email, created_at as "created_at: DateTime<Utc>", updated_at as "updated_at: DateTime<Utc>"
            FROM users
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, DomainError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id as "id: Uuid", username, role as "role: Role", password_hash, email, created_at as "created_at: DateTime<Utc>", updated_at as "updated_at: DateTime<Utc>"
            FROM users
            WHERE username = ?
            "#,
            username
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn update(&self, user: User) -> Result<User, DomainError> {
        let now = Utc::now();
        let res = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET username = ?, password_hash = ?, email = ?, updated_at = ?, role = ?
            WHERE id = ?
            RETURNING id as "id: Uuid", username, role as "role: Role", password_hash, email, created_at as "created_at: DateTime<Utc>", updated_at as "updated_at: DateTime<Utc>"
            "#,
            user.username,
            user.password_hash,
            user.email,
            now,
            user.role,
            user.id
        )
        .fetch_one(&self.pool)
        .await;

        match res {
            Ok(user) => Ok(user),
            Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
                Err(DomainError::DuplicateEmail)
            }
            Err(e) => Err(e.into()),
        }
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        let result = sqlx::query!("DELETE FROM users WHERE id = ?", id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            Err(DomainError::NotFound)
        } else {
            Ok(())
        }
    }
}
