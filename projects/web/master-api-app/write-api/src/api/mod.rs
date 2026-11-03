use axum::Json;
use core_library_db_types::state::ApiInfo;

pub mod v1;

pub async fn get_api_info() -> Json<ApiInfo> {
   Json(
       ApiInfo {
           name: "Write API Persona Metrics".to_string(),
           version: "0.1.0".to_string(),
           status: "healthy".to_string(),
       }
   )
}