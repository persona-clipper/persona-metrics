use axum::Router;
use axum::routing::{get, post};
use crate::api::get_api_info;
use crate::model::users::{create_organization_handler, create_user_handler};
use crate::state::AppState;

pub fn api_v1() -> Router<AppState> {
    Router::new()
        .route("/", get(get_api_info))
        // .route("/users/{id}", get(get_user_by_id_handler))
        .route("/users", post(create_user_handler))
        .route("/users/orgs", post(create_organization_handler))
}