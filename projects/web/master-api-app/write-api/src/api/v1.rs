use axum::Router;
use axum::routing::get;
use crate::state::AppState;

pub fn api_v1() -> Router<AppState> {
    Router::new()
        .route("/", get(|| async { "API v1: Write side"} ))
        .route("/users/:id", get(|| async {"Получить пользователя по айди"}))
        .route("/users", get(|| async {"Добавить пользователя"}))
}