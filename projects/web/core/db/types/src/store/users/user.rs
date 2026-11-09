use std::string::ToString;
use axum::http::StatusCode;
use chrono::{TimeZone, Utc};
use compact_str::CompactString;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use crate::infrastructure::model::{ToHttpCode, ToHumanMessage};

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

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateOrganizationForm {
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
}

// #[derive(Serialize, Deserialize)]
// pub struct OrganizationsUserListItem {
//     pub name: String,
//     pub description: String,
//     pub server_count: i32,
//     pub user_role: String,
// }
//
pub struct User(String);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum UserError {
    UserUsernameNotEnoughLength { username: String },
    UserInvalidCharactersInUsername { username: String },
    UserFailAuth { reason: String },
    UserNotFound,
}

impl ToHttpCode for UserError {
    fn http_code(&self) -> StatusCode {
       match &self {
           UserError::UserUsernameNotEnoughLength { .. } => StatusCode::BAD_REQUEST,
           UserError::UserInvalidCharactersInUsername { .. } => StatusCode::BAD_REQUEST,
           UserError::UserFailAuth { .. } => StatusCode::UNAUTHORIZED,
           UserError::UserNotFound => StatusCode::NOT_FOUND,
       }
    }
}

impl ToHumanMessage for UserError {
    fn message(&self) -> String {
        match &self {
            UserError::UserUsernameNotEnoughLength { username} => {
                tracing::warn!("Username `{}` is too short (minimum 8 characters)", username);
                "Username is too short (min. 8 characters)".into()
            },
            UserError::UserInvalidCharactersInUsername { username } => {
                tracing::warn!("The username `{}` contains invalid characters", username);
                "The username contains invalid characters".into()
            }
            UserError::UserNotFound => {
                tracing::warn!("");
                "User not found".into()
            },
            UserError::UserFailAuth { reason }=> {
                tracing::warn!("Failed to auth user: {}", reason);
                "Authorization error. Most likely, the password was incorrect.".into()
            }
        }
    }
}

impl User {
    pub fn new(username: &str) -> Result<Self, UserError> {
        if username.len() < 8 {
            return Err(UserError::UserUsernameNotEnoughLength { username: username.into() });
        }
        if !username.chars().any(|c| { c.is_alphanumeric() }) {
            return Err(UserError::UserInvalidCharactersInUsername { username: username.into() });
        }

        Ok(Self(username.to_string()))
    }
}