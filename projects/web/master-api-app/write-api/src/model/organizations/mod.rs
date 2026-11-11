use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use sqlx::PgPool;
use core_library_db_queries::model::json_token::context::AuthClaims;
use core_library_db_queries::write::organizations::insert_organization;
use core_library_db_types::error::ApiErrorCode;
use core_library_db_types::store::users::user::CreateOrganizationForm;
use crate::state::AppState;

pub async fn create(
    claims: AuthClaims,
    State(state): State<AppState>,
    Json(payload): Json<CreateOrganizationForm>,
) -> Result<(StatusCode, Json<i32>), ApiErrorCode> {

    tracing::info!("{:#?}", payload);
    tracing::info!("{:#?}", claims);
    match insert_organization(&payload, claims.0.sub, state.db.clone()).await {
        Ok(id) => {
            Ok((StatusCode::CREATED, Json(id)))
        },
        Err(err) => {
            tracing::error!("{:#?}", err.to_string());
            Err(ApiErrorCode::Other { reason: err.to_string() })
        }
    }
}