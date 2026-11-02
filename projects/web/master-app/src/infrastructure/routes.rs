use axum::Router;
use axum::routing::{get, Route};
use tower_http::services::ServeDir;
use crate::infrastructure::states::AppState;
use crate::interface::handlers::main_page::{get_main_page_handler, get_physic_ball};
use crate::interface::handlers::users::*;

pub fn app_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(get_main_page_handler))
        .route("/signup", get(registration::get_form).post(registration::create_user_handler))
        .route("/login", get(authorization::get_form).post(authorization::auth_user_handler))
        .route("/physic", get(get_physic_ball))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state)
}