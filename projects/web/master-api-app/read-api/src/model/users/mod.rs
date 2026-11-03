use std::error::Error;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde_json::json;
use core_library_db_queries::read::users::user::{check_user_auth, get_user_by_id};
use core_library_db_types::error::{ApiErrorCode, ApiErrorResponse};
use core_library_db_types::store::users::user::{GetUserRow, UserError};
use core_web_types::infrastructure::users::forms::AuthorizationUserForm;
use crate::state::AppState;

/// Эндпоинт для получения информации о пользователе по его 
/// уникальному идентификатору
///
/// # Error
/// Вызовет ошибку если пользователь не найден 
/// или база данных не приняла запрос
pub async fn get_by_id_handler(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<GetUserRow>), ApiErrorResponse> {
    let user = match get_user_by_id(id, state.db.clone()).await {
        Ok(user) => user,
        Err(sqlx::Error::RowNotFound) => {
            let err = ApiErrorResponse {
                code: ApiErrorCode::User(UserError::UserNotFound),
                message: "User not found".to_string(),
            };
            return Err(err);
        },
        Err(err) => {
            let err = ApiErrorResponse {
                code: ApiErrorCode::Other,
                message: format!("Error: {err}"),
            };
            return Err(err);
        },
    };

    Ok((StatusCode::OK, Json(user)))
}

pub async fn auth_handler(
    State(state): State<AppState>,
    Json(form): Json<AuthorizationUserForm>,
) -> Result<(StatusCode, Json<i32>), ApiErrorResponse> {
    match check_user_auth(&form, state.db.clone()).await {
        Ok(id) => {
            Ok((StatusCode::OK, Json(id)))
        },
        Err(err_code) => {
            let err = ApiErrorResponse {
                code: err_code,
                message: "Пользователь не найден".into(),
            };
            Err(err)
        },
    }
}