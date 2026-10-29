use axum::Router;
use axum::routing::get;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .compact()
        .without_time()
        .init();

    let router = Router::new()
        .route("/", get(|| async { "Hello, Read API"}));

    let listener = tokio::net::TcpListener::bind("127.0.0.1::1313").await.unwrap();

    tracing::info!("API (Read): Starting at http://127.0.0.1:1313");
    axum::serve(listener, router).await.unwrap();
}
