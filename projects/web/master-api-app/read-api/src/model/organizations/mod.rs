use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use sqlx::Error;
use core_library_db_queries::model::json_token::context::AuthClaims;
use core_library_db_queries::read::organizations::{fetch_organization_info, get_organizations_by_user_id};
use core_library_db_types::error::ApiErrorCode;
use core_library_db_types::store::organizations::list::OrganizationListItem;
use core_library_db_types::store::organizations::OrganizationInfo;
use crate::state::AppState;

pub async fn fetch_organization_info_handler(
    Path(org_hash_id): Path<String>,
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<OrganizationInfo>), ApiErrorCode> {
    match fetch_organization_info(org_hash_id, state.db.clone()).await {
        Ok(val) => {
           Ok((StatusCode::OK, Json(val)))
        },
        Err(e) => {
           Err(ApiErrorCode::Other { reason: e.to_string() })
        }
    }
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
