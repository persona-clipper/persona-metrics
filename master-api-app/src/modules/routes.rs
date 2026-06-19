use axum::Router;
use axum::routing::{get, post};

pub fn api_v_1() -> Router {
    Router::new()
        .route("/", get(|| async { "api_v1"}))
        .route("/metrics", post(|| async { "api_v1metrics" }))
}