use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct RegistrationUserForm {
    pub username: String,
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub password: String,
    pub password_confirm: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct AuthorizationUserForm {
    pub username: String,
    pub password: String,
}