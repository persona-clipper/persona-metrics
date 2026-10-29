use std::env;
use axum::Router;
use axum::routing::get;
use sqlx::PgPool;
use api_write_side::api::v1::api_v1;
use api_write_side::state::AppState;
use core_library_db_queries::db::create_database;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .compact()
        .without_time()
        .init();

    let db_url = env::var("DATABASE_URL").expect("Failed to get env var `DATABASE_URL`");
    let db_connection = create_database(&db_url).await.map_err(|e| {
        tracing::error!("Failed to connect db. Error: {}", e);
    }).unwrap();

    let state = AppState {
        db: db_connection,
    };

    let router = Router::new()
        .route("/", get(|| async { "Hello, Write API" }))
        .nest("/v1/", api_v1())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:1212").await.unwrap();

    tracing::info!("API (Write): Starting at http://127.0.0.1:1212");
    axum::serve(listener, router).await.unwrap();
}
