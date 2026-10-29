use axum::Router;
use axum::routing::get;
use crate::model::users::get_user_by_id_handler;
use crate::state::AppState;

pub fn api_v1() -> Router<AppState> {
    Router::new()
        .route("/", get(|| async { "This is API V1" }))
        .route("/users/{id}", get(get_user_by_id_handler))
}