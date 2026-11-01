use axum::Router;
use axum::routing::{get, Route};
use tower_http::services::ServeDir;
use crate::infrastructure::states::AppState;
use crate::interface::handlers::main_page::{get_main_page_handler, get_physic_ball};
use crate::interface::handlers::users::authorization::get_authorization_user_form;
use crate::interface::handlers::users::registration::{create_user_handler, get_registration_form_handler};

pub fn app_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(get_main_page_handler))
        .route("/signup", get(get_registration_form_handler).post(create_user_handler))
        .route("/login", get(get_authorization_user_form))
        .route("/physic", get(get_physic_ball))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state)
}