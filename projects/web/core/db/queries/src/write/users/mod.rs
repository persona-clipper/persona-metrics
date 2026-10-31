use axum::extract::State;
use axum::{Form, Json};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sqlx::PgPool;
use core_library_db_types::store::users::password::Password;
use core_library_db_types::store::users::user::CreateUserRow;
use core_web_types::modules::users::forms::{CreateUserResponse, RegistrationUserForm};

pub async fn insert_user_query(
    form: CreateUserRow,
    db: PgPool,
) -> Result<CreateUserResponse, sqlx::Error> {
    sqlx::query_as!(
        CreateUserResponse,
        r#"
        INSERT INTO users(username, display_name, email, password_hash)
        VALUES($1, $2, $3, $4)
        RETURNING id
        "#,
        form.username,
        form.display_name.unwrap_or_else(|| "User".to_string()),
        form.email,
        form.password_hash
    ).fetch_one(&db).await
}