use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use core_library_db_queries::model::json_token::context::AuthClaims;
use core_library_db_queries::read::organizations::get_organizations_by_user_id;
use core_library_db_types::error::ApiErrorCode;
use core_library_db_types::store::organizations::list::OrganizationListItem;
use crate::state::AppState;

pub async fn fetch_organization_info(
    claims: AuthClaims,
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<OrganizationListItem>>), ApiErrorCode> {
    
}

pub async fn get_organizations_with_user_id(
    claims: AuthClaims,
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<OrganizationListItem>>), ApiErrorCode> {
    match get_organizations_by_user_id(claims.0.sub, state.db.clone()).await {
        Ok(val) => {
            Ok((StatusCode::OK, Json(val)))
        },
        Err(err) => {
            Err(ApiErrorCode::Other { reason: err.to_string() })
        }
    }
}
