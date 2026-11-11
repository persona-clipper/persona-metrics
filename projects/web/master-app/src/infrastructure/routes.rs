use crate::infrastructure::states::AppState;
use crate::interface::handlers::main_page::{get_main_page_handler, get_physic_ball};
use crate::interface::handlers::users::*;
use crate::interface::handlers::organizations;
use axum::Router;
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::{IntoResponse, Redirect};
use axum::routing::get;
use tower_http::services::ServeDir;
use tower_sessions::{SessionManagerLayer, SessionStore};
use core_library_db_queries::model::sessions::context::SessionContext;
use crate::interface::handlers::common::not_found_handler;

pub fn app_router<T: SessionStore + Clone>(
    state: AppState,
    session: SessionManagerLayer<T>,
) -> Router {
    // Роуты доступнеы только пользователям с аккаунтом
    let private_routes = Router::new()
        .route("/logout", get(model::logout_handler))
        .route("/organizations/create", get(organizations::get_create_form))
        .route("/organizations", get(organizations::get_page).post(organizations::create_organization_handler))
        .route_layer(axum::middleware::from_fn(require_auth));

    Router::new()
        .route("/", get(get_main_page_handler))
        .route("/signup", get(registration::get_form).post(registration::create_user_handler))
        .route("/login", get(authorization::get_form).post(authorization::auth_user_handler))
        .route("/physic", get(get_physic_ball))
        .merge(private_routes)
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state)
        .layer(session)
        .fallback(not_found_handler)
}

pub async fn require_auth(
    session_res: SessionContext,
    request: Request,
    next: Next,
) -> axum::response::Response {
    match session_res.user {
        Some(_) => next.run(request).await,
        None => Redirect::temporary("/login").into_response()
    }
}
