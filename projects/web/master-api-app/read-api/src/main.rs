use std::env;
use axum::middleware::from_extractor_with_state;
use axum::Router;
use axum::routing::get;
use api_read_side::api::get_api_info;
use api_read_side::api::v1::api_v1;
use api_read_side::api::v2::api_v2;
use api_read_side::state::AppState;
use core_library_db_queries::db::create_database;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .compact()
        .without_time()
        .init();

    let state = AppState::new().await;

    let router = Router::new()
        .route("/", get(get_api_info))
        .nest("/v1/", api_v1())
        .nest("/v2/", api_v2())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:1313").await.unwrap();

    tracing::info!("API (Read): Starting at http://127.0.0.1:1313");
    axum::serve(listener, router).await.unwrap();
}
