use axum::Json;
use core_library_db_types::state::ApiInfo;
use crate::state::AppState;

pub mod v1;
pub mod v2;

pub async fn get_api_info() -> Json<ApiInfo> {
    Json(
        ApiInfo {
            name: "Read API Persona Metrics".to_string(),
            version: "0.0.1".to_string(),
            status: "healthy".to_string(),
        }
    )
}