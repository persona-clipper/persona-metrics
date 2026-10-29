use chrono::{TimeZone, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

#[derive(Type, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[sqlx(type_name = "user_role", rename_all="lowercase")]
pub enum UserRole {
    Admin,
    User,
    Moderator,
    Guest,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub display_name: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub role: UserRole,
    pub avatar_path: Option<String>,
    pub is_active: bool,
    pub created_at: chrono::DateTime<Utc>,
}