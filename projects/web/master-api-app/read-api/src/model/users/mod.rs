use axum::extract::{Path, State};
use axum::response::IntoResponse;
use core_library_db_queries::read::users::get_user_by_id;
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
) -> impl IntoResponse {
    get_user_by_id(id, state.db.clone()).await
}