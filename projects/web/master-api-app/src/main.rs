use axum::Router;
use axum::routing::get;
use master_api_app::modules::routes::api_v_1;
use master_api_app::modules::strutures::AppState;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_env_filter("info").init();

    let state = AppState { pool: None };
    let app: Router = Router::new()
        .route("/", get(|| async { "this is API APP PERSONA!!!!!!" }))
        .nest("/api/v1", api_v_1())
        .with_state(state);

    let url = "0.0.0.0:3030";
    let listener = tokio::net::TcpListener::bind(url).await.unwrap();

    tracing::info!("Server start at http://{url}");
    axum::serve(listener, app).await.unwrap();
}
