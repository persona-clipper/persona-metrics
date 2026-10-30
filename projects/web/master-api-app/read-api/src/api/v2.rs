use axum::Router;
use axum::routing::get;
use crate::state::AppState;

pub fn api_v2() -> Router<AppState> {
    Router::new()
        .route("/", get(|| async {"This is Read APIv2 (example)"}))
}