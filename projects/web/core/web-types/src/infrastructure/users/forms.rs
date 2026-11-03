use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct RegistrationUserForm {
    pub username: String,
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub password: String,
    pub password_confirm: String,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct AuthorizationUserForm {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserResponse { pub id: i32 }

#[derive(Serialize, Deserialize)]
pub struct AuthorizationUserResponse {
    pub id: i32,
    pub avatar_path: String,
}