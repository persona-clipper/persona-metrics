use std::string::ToString;
use axum::http::StatusCode;
use chrono::{TimeZone, Utc};
use compact_str::CompactString;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use crate::infrastructure::model::ToHttpCode;

#[derive(Type, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[sqlx(type_name = "user_role", rename_all="lowercase")]
pub enum UserRole {
    Admin,
    User,
    Moderator,
    Guest,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct GetUserRow {
    pub id: i64,
    pub display_name: Option<String>,
    pub username: CompactString,
    pub email: Option<String>,
    pub password_hash: String,
    pub role: UserRole,
    pub avatar_path: Option<String>,
    pub is_active: bool,
    pub created_at: chrono::DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserRow {
    pub password_hash: String,
    pub username: String,
    pub display_name: Option<String>,
    pub email: Option<String>,
}

pub struct User(String);

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub enum UserError {
    UserNotEnoughLength,
    UserInvalidCharactersInUsername,
    UserFailAuth,
    UserNotFound,
}

impl ToHttpCode for UserError {
    fn http_code(&self) -> StatusCode {
       match &self {
           UserError::UserNotEnoughLength => StatusCode::BAD_REQUEST,
           UserError::UserInvalidCharactersInUsername => StatusCode::BAD_REQUEST,
           UserError::UserFailAuth => StatusCode::UNAUTHORIZED,
           UserError::UserNotFound => StatusCode::NOT_FOUND,
       }
    }
}

impl User {
    pub fn new(username: &str) -> Result<Self, UserError> {
        if username.len() < 8 {
            return Err(UserError::UserNotEnoughLength);
        }
        if !username.chars().any(|c| { c.is_alphanumeric() }) {
            return Err(UserError::UserInvalidCharactersInUsername);
        }

        Ok(Self(username.to_string()))
    }
}