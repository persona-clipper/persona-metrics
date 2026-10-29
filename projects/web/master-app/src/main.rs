use askama::Template;
use axum::Router;
use axum::routing::get;
use master_app::modules::states::AppState;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = AppState::new();

    let app: Router = Router::new()
        .route("/", get(async || "Hello, AXUM!"))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct Piroska {
    pub age: String,
    pub name: String,
    pub users: Vec<String>,
}
