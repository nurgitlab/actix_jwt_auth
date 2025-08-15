use derive_more::Display;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use validator::Validate;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub message: String,
    pub user_id: i32,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Deserialize, Validate, Display)]
#[display("CreatePost: message={message}, user_id={user_id}")]
pub struct CreatePost {
    #[validate(length(
        min = 1,
        message = "Username must be at least 1 character long"
    ))]
    pub message: String,
    pub user_id: i32,
}

#[derive(Debug, Deserialize, Validate, Display)]
#[display("GetAllPosts: user_id={user_id}")]
pub struct GetAllPosts {
    pub user_id: i32,
}

#[derive(Debug, Deserialize, Validate, Display)]
#[display("GetPost: id={id}, user_id={user_id}")]
pub struct GetPost {
    pub user_id: i32,
    pub id: i32,
}

#[derive(Debug, Deserialize, Validate, Display)]
#[display("UpdatePost: id={id}, message={message}, user_id={user_id}")]
pub struct UpdatePost {
    pub id: i32,
    #[validate(length(
        min = 1,
        message = "Username must be at least 1 character long"
    ))]
    pub message: String,
    pub user_id: i32,
}

#[derive(Debug, Deserialize, Validate, Display)]
#[display("DeletePost: id={id}")]
pub struct DeletePost {
    pub id: i32,
}
