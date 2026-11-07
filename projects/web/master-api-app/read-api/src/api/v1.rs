use axum::Router;
use axum::routing::{get, post};
use crate::model::users;
use crate::metrics;
use crate::state::AppState;

pub fn api_v1() -> Router<AppState> {
    Router::new()
        .route("/", get(|| async { "This is API V1" }))
        .route("/users/{id}", get(users::get_by_id_handler))
        .route("/users/auth", post(users::auth_handler))
        .route("/metrics/cpu/{hostname}", get(metrics::global_cpu_usage))
}