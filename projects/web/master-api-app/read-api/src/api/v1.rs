use axum::Router;
use axum::routing::{get, post};
use crate::model::users;
use crate::model::organizations;
use crate::metrics;
use crate::state::AppState;

pub fn api_v1() -> Router<AppState> {
    Router::new()
        .route("/", get(|| async { "This is API V1" }))
        .route("/me/orgs", get(organizations::get_organizations_with_user_id))
        .route("/me/profile", get(users::get_by_id_handler))
        // .route("/users/{id}", get(users::get_by_id_handler))
        .route("/users/auth", post(users::auth_handler))
        // .route("/users/{id}/orgs", get(users::))
        // .route("/users/{id}/organizations")
        .route("/metrics/cpu/{hostname}", get(metrics::global_cpu_usage))
        .route("/orgs/{unique_id}", get(organizations::fetch_organization_info_handler))
}