use std::error::Error;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde_json::json;
use core_library_db_queries::read::users::user::{check_user_auth, get_user_by_id};
use core_library_db_types::error::{ApiErrorCode, ApiErrorResponse};
use core_library_db_types::store::users::user::{GetUserRow, UserError};
use core_web_types::infrastructure::users::forms::{AuthorizationUserForm, SessionUserInfo};
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
) -> Result<(StatusCode, Json<GetUserRow>), ApiErrorCode> {
    let user = match get_user_by_id(id, state.db.clone()).await {
        Ok(user) => user,
        Err(sqlx::Error::RowNotFound) => {
            return Err(ApiErrorCode::User(UserError::UserNotFound));
        },
        Err(err) => {
            return Err(ApiErrorCode::Other { reason: err.to_string() });
        },
    };

    Ok((StatusCode::OK, Json(user)))
}

pub async fn auth_handler(
    State(state): State<AppState>,
    Json(form): Json<AuthorizationUserForm>,
) -> Result<(StatusCode, Json<SessionUserInfo>), ApiErrorCode> {
    match check_user_auth(&form, state.db.clone()).await {
        Ok(info) => {
            Ok((StatusCode::OK, Json(info)))
        },
        Err(err_code) => {
            Err(err_code)
        },
    }
}

// pub async fn get_organizations(
//     Path(user_id): Path<i32>,
//     State(state): State<AppState>,
// ) -> Result<(StatusCode, Json<>), ApiErrorCode> {
// 
// }
// 
