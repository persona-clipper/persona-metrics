use std::fmt::format;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use sqlx::PgPool;
use core_library_db_types::store::users::{User, UserRole};

pub async fn get_user_by_id(
    id: i32,
    pool: PgPool,
) -> impl IntoResponse{
    let response = sqlx::query_as!(
        User,
        r#"
        SELECT
            id,
            display_name,
            username,
            email,
            password_hash,
            role as "role: UserRole",
            avatar_path,
            is_active,
            created_at
        FROM users WHERE id = $1
        "#,
        id
    ).fetch_one(&pool).await;

    match response {
        Ok(user) => {
            (StatusCode::OK, Json(user)).into_response()
        },
        Err(sqlx::Error::RowNotFound) => {
            (StatusCode::NOT_FOUND, "User not found").into_response()
        },
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}