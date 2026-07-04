use crate::modules::sql_queries::common;
use crate::modules::sql_queries::common::get_metrics;
use crate::modules::strutures::AppState;
use axum::Router;
use axum::routing::{get, post};

pub fn api_v_1() -> Router<AppState> {
    Router::new()
        .route("/", get(|| async { "api_v1" }))
        .route("/metrics/generate", post(|| async { "generate_token" }))
        .route("/metrics", post(get_metrics))
}
