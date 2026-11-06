use std::error::Error;
use std::fmt::format;
use std::io::read_to_string;
use argon2::{Argon2, PasswordHash};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use password_hash::PasswordVerifier;
use sqlx::PgPool;
use core_library_db_types::error::ApiErrorCode;
use core_library_db_types::store::users::password::PasswordError;
use core_library_db_types::store::users::user::{GetUserRow, UserError, UserRole};
use core_web_types::infrastructure::users::forms::{AuthorizationUserForm, SessionUserInfo};

// pub struct User();
pub async fn get_user_by_id(
    id: i32,
    pool: PgPool,
) -> Result<GetUserRow, sqlx::Error> {
    sqlx::query_as!(
        GetUserRow,
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
    ).fetch_one(&pool).await

    // match response {
    //     Ok(user) => {
    //         (StatusCode::OK, Json(user)).into_response()
    //     },
    //     Err(sqlx::Error::RowNotFound) => {
    //         (StatusCode::NOT_FOUND, "User not found").into_response()
    //     },
    //     Err(e) => {
    //         (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    //     }
    // }
}

pub async fn check_user_is_exist(
    id: i32,
    pool: PgPool,
) -> bool {
    sqlx::query!("SELECT id FROM users WHERE id = $1", id)
        .fetch_one(&pool)
        .await
        .is_ok()
}

pub async fn check_user_auth(
    form: &AuthorizationUserForm,
    pool: PgPool,
) -> Result<SessionUserInfo, ApiErrorCode> {
    let response = sqlx::query!(
        "SELECT avatar_path, password_hash, id, username FROM users WHERE username = $1",
        form.username,
    ).fetch_one(&pool).await;

    let user_info = match response {
        Ok(record) => record,
        Err(sqlx::Error::RowNotFound) => {
            return Err(ApiErrorCode::User(UserError::UserNotFound))
        }
        Err(err) => {
            return Err(ApiErrorCode::Other { reason: err.to_string() })
        }
    };

    let password_hash_from_db = match PasswordHash::new(&user_info.password_hash) {
        Ok(hash) => hash,
        Err(err) => return Err(ApiErrorCode::Password(PasswordError::PasswordFailedToHash { reason: err.to_string() })),
    };

    match Argon2::default().verify_password(form.password.as_bytes(), &password_hash_from_db) {
        Ok(_) => Ok(SessionUserInfo {
            id: user_info.id,
            avatar_path: user_info.avatar_path,
            username: user_info.username,
        }),
        Err(err) => Err(ApiErrorCode::Password(PasswordError::PasswordFailedToVerify { reason: err.to_string() })),
    }
}

pub async fn fetch_user_info_for_auth(id: i32, pool: PgPool) -> Result<SessionUserInfo, ApiErrorCode> {
   let query = sqlx::query_as!(
       SessionUserInfo,
       "SELECT id, avatar_path, username FROM users WHERE id = $1",
       id,
   ).fetch_one(&pool).await;

    match query {
        Ok(info) => {
           Ok(info)
        }
        Err(sqlx::Error::RowNotFound) => {
            Err(ApiErrorCode::User(UserError::UserNotFound))
        }
        Err(err) => {
            Err(ApiErrorCode::User(UserError::UserFailAuth { reason: err.to_string() }))
        }
    }
}

