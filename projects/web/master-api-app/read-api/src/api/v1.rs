use axum::Router;
use axum::routing::{get, post};
use crate::model::users::{auth_handler, get_by_id_handler};
use crate::state::AppState;

pub fn api_v1() -> Router<AppState> {
    Router::new()
        .route("/", get(|| async { "This is API V1" }))
        .route("/users/{id}", get(get_by_id_handler))
        .route("/users/auth", post(auth_handler))
}