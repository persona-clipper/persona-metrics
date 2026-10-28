use crate::modules::states::AppState;
use axum::Router;
use axum::routing::get;

pub fn route_v_1() -> Router<AppState> {
    Router::new().route("/", get(|| async { "Hello, world!" }))
}
