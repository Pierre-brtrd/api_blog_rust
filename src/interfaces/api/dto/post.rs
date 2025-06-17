use crate::interfaces::api::{
    error::ApiError,
    validation::{require_field, validate_dto},
};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct NewPost {
    #[validate(
        length(
            min = 2,
            max = 255,
            message = "Title must be between 2 and 255 characters long"
        ),
        required(message = "Title is required")
    )]
    pub title: Option<String>,
    #[validate(
        length(min = 2, message = "Content must be at least 2 characters long"),
        required(message = "Content is required")
    )]
    pub content: Option<String>,
    pub published: bool,
    #[validate(required(message = "User ID is required"))]
    pub user_id: Option<Uuid>,
}

impl NewPost {
    pub fn validate_post(&self) -> Result<(), ApiError> {
        validate_dto(self)?;
        Ok(())
    }

    pub fn validate_and_into_domain(self) -> Result<(String, String, bool, Uuid), ApiError> {
        let title = require_field(self.title, "title")?;
        let content = require_field(self.content, "content")?;
        let user_id = require_field(self.user_id, "user_id")?;
        let published = self.published;

        Ok((title, content, published, user_id))
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdatePost {
    #[validate(length(
        min = 2,
        max = 255,
        message = "Title must be between 2 and 255 characters long"
    ))]
    pub title: Option<String>,
    #[validate(length(min = 2, message = "Content must be at least 2 characters long"))]
    pub content: Option<String>,
    pub published: Option<bool>,
    pub user_id: Option<Uuid>,
}

pub struct UpdatePostPayload {
    pub title: Option<String>,
    pub content: Option<String>,
    pub published: Option<bool>,
    pub user_id: Option<Uuid>,
}

impl UpdatePost {
    pub fn validate_post(&self) -> Result<(), ApiError> {
        validate_dto(self)?;
        Ok(())
    }

    pub fn validate_and_into_domain(self) -> Result<UpdatePostPayload, ApiError> {
        validate_dto(&self)?;

        let title = self.title;
        let content = self.content;
        let published = self.published;
        let user_id = self.user_id;

        Ok(UpdatePostPayload {
            title,
            content,
            published,
            user_id,
        })
    }
}
