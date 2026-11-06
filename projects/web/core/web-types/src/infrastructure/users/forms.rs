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

/// Метод авторизации пользователя должна возвращать данную структуру
/// замена `AuthorizationUserResponse`
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SessionUserInfo {
    pub id: i32,
    pub username: String,
    pub avatar_path: Option<String>,

}