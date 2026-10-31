use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde_json::json;
use core_library_db_queries::read::users::user::get_user_by_id;
use core_library_db_types::state::{ApiErrorCode, ApiErrorResponse};
use core_library_db_types::store::users::user::GetUserRow;
use crate::state::AppState;

/// Эндпоинт для получения информации о пользователе по его 
/// уникальному идентификатору
///
/// # Error
/// Вызовет ошибку если пользователь не найден 
/// или база данных не приняла запрос
pub async fn get_user_by_id_handler(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<GetUserRow>), (StatusCode, Json<ApiErrorResponse>)> {
    let user = match get_user_by_id(id, state.db.clone()).await {
        Ok(user) => user,
        Err(sqlx::Error::RowNotFound) => {
            let err = ApiErrorResponse {
                code: ApiErrorCode::UserNotFound,
                message: "User not found".to_string(),
            };
            return Err((StatusCode::NOT_FOUND, Json(err)));
        },
        Err(err) => {
            let err = ApiErrorResponse {
                code: ApiErrorCode::Unknown,
                message: format!("Error: {err}"),
            };
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(err)));
        },
    };

    Ok((StatusCode::OK, Json(user)))
}