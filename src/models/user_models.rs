use derive_more::Display;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
}

#[derive(Debug, Deserialize, Validate, Display)]
pub struct CreateUser {
    #[validate(length(
        min = 3,
        max = 25,
        message = "Username must be between 3 and 25 chars"
    ))]
    pub username: String,
}

#[derive(Debug, Deserialize, Validate, Display)]
pub struct UpdateUser {
    #[validate(length(
        min = 3,
        max = 25,
        message = "Username must be between 3 and 25 chars"
    ))]
    pub username: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UserPath {
    #[validate(range(min = 1, message = "User ID must be positive"))]
    pub user_id: i32,
}
