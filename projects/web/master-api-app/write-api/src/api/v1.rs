use axum::Router;
use axum::routing::{get, post};
use crate::state::AppState;

pub fn api_v1() -> Router<AppState> {
    Router::new()
        .route("/", get(|| async { "API v1: Write side"} ))
        // .route("/users/{id}", get(get_user_by_id_handler))
        .route("/users", get(|| async {"Добавить пользователя"}))
}