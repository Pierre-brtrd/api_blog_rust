use crate::{
    api::error::ApiError,
    domain::{
        error::DomainError,
        repository::UserRepository,
        user::{NewUser, User},
    },
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::SqlitePool;
use uuid::Uuid;

use argon2::password_hash::{Error, PasswordHash, SaltString, rand_core::RngCore};
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

pub fn hash_password(password: &str) -> Result<String, Error> {
    // Generate a random salt
    let mut rng = ChaCha20Rng::from_entropy();
    let mut salt_bytes = [0u8; 32];
    rng.fill_bytes(&mut salt_bytes);

    let salt = SaltString::generate(&mut rng);

    // hash the password
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    Ok(password_hash)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, Error> {
    let parsed_hash = PasswordHash::new(hash)?;
    let argon2 = Argon2::default();

    Ok(argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

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
    async fn list(&self) -> Result<Vec<User>, anyhow::Error> {
        let rows = sqlx::query_as!(
            User,
            r#"
            SELECT id as "id: Uuid", username, password_hash, email, created_at as "created_at: DateTime<Utc>", updated_at as "updated_at: DateTime<Utc>"
            FROM users
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    async fn create(&self, new_user: NewUser) -> Result<User, anyhow::Error> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let password = new_user
            .password
            .as_deref()
            .ok_or_else(|| DomainError::EmptyContent)?;

        let hashed = hash_password(password)
            .map_err(|e| DomainError::PasswordHashingError(e.to_string()))?;

        let user = NewUser {
            username: new_user.username,
            password: Some(hashed),
            email: new_user.email,
        };

        let res = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, username, password_hash, email, created_at)
            VALUES (?, ?, ?, ?, ?)
            RETURNING id as "id: Uuid", username, email, password_hash, created_at as "created_at: DateTime<Utc>", updated_at as "updated_at: DateTime<Utc>"
            "#,
            id,
            user.username,
            user.password,
            user.email,
            now
        )
        .fetch_one(&self.pool)
        .await;

        match res {
            Ok(user) => Ok(user),
            Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
                Err(ApiError::BadRequest("Email already exists".to_string()).into())
            }
            Err(e) => Err(e.into()),
        }
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, anyhow::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id as "id: Uuid", username, password_hash, email, created_at as "created_at: DateTime<Utc>", updated_at as "updated_at: DateTime<Utc>"
            FROM users
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, anyhow::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id as "id: Uuid", username, password_hash, email, created_at as "created_at: DateTime<Utc>", updated_at as "updated_at: DateTime<Utc>"
            FROM users
            WHERE username = ?
            "#,
            username
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn update(&self, user: User) -> Result<User, anyhow::Error> {
        let now = Utc::now();
        let res = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET username = ?, password_hash = ?, email = ?, updated_at = ?
            WHERE id = ?
            RETURNING id as "id: Uuid", username, password_hash, email, created_at as "created_at: DateTime<Utc>", updated_at as "updated_at: DateTime<Utc>"
            "#,
            user.username,
            user.password_hash,
            user.email,
            now,
            user.id
        )
        .fetch_one(&self.pool)
        .await;

        match res {
            Ok(user) => Ok(user),
            Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
                Err(ApiError::BadRequest("Email already exists".to_string()).into())
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
