use axum::Router;
use axum::routing::get;

fn route_v_1() -> Router {
    Router::new().route("/", get(|| async { "Hello, world!" }))
}
