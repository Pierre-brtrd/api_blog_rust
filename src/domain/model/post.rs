use crate::interfaces::api::dto::user::UserPublic;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub content: String,
    pub published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct PostWithAuthor {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub author: UserPublic,
}

impl From<PostWithAuthor> for Post {
    fn from(pwa: PostWithAuthor) -> Self {
        Self {
            id: pwa.id,
            user_id: pwa.author.id,
            title: pwa.title,
            content: pwa.content,
            published: pwa.published,
            created_at: pwa.created_at,
            updated_at: pwa.updated_at,
        }
    }
}
