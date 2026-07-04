use crate::modules::strutures::AppState;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::{Error, Json};
use axum_extra::TypedHeader;
use headers::Authorization;
use headers::authorization::Bearer;
use persona_exporter_types::ServerMetrics;

pub async fn get_metrics(
    State(state): State<AppState>,
    // headers: HeaderMap,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Json(payload): Json<ServerMetrics>,
) -> &'static str {
    tracing::info!("{:#?}", auth.0.token());
    tracing::info!("{:#?}", payload);

    "Ok"
}
