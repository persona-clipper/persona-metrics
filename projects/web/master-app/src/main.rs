use askama::Template;
use axum::Router;
use axum::routing::get;
use tower_http::services::ServeDir;
use master_app::handlers::main_page::{get_main_page_handler, get_physic_ball, get_registration_form_handler};
use master_app::handlers::users::registration::registration_user_handler;
use master_app::modules::states::AppState;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .compact()
        .without_time()
        .init();

    let state = AppState::new();

    let app: Router = Router::new()
        .route("/", get(get_main_page_handler))
        .route("/signup", get(get_registration_form_handler).post(registration_user_handler))
        .route("/login", get(|| async { "" }))
        .route("/physic", get(get_physic_ball))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state);

    tracing::info!("Starting main app at http://127.0.0.1:3030");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3030").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}