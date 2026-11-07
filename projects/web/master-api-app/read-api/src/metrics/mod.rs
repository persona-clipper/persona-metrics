use axum::extract::State;
use crate::state::AppState;

pub async fn global_cpu_usage(
    State(state): State<AppState>,
) {}