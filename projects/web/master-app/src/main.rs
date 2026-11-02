use axum::Router;
use axum::routing::get;
use master_app::infrastructure::layers::apply_session_layer;
use master_app::infrastructure::routes::app_router;
use master_app::interface::handlers::main_page::{get_main_page_handler, get_physic_ball};
use master_app::interface::handlers::users::registration::{create_user_handler, get_form};
use tower_http::services::ServeDir;
use tower_sessions::SessionManager;
use master_app::infrastructure::states::AppState;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .compact()
        .without_time()
        .init();

    let state = AppState::new();

    let router = app_router(state);
    let app: Router = apply_session_layer(router);

    // let app = Router::new()
    //     .route("/", get(get_main_page_handler))
    //     .route("/signup", get(get_registration_form_handler).post(create_user_handler))
    //     .route("/login", get(|| async { "" }))
    //     .route("/physic", get(get_physic_ball))
    //     .nest_service("/static", ServeDir::new("static"))
    //     .with_state(state);

    tracing::info!("Starting main app at http://127.0.0.1:3030");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3030").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}