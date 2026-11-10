use std::hash::Hash;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Form, Json};
use axum::response::IntoResponse;
use tracing::error_span;
use core_library_db_queries::model::json_token::context::AuthClaims;
use core_library_db_queries::model::sessions::context::SessionContext;
use core_library_db_queries::read::users::user::check_user_auth;
use core_library_db_queries::write::users::{insert_user_query};
use core_library_db_types::error::{ApiErrorCode, ApiErrorResponse};
use core_library_db_types::store::users::password::{Password, PasswordError};
use core_library_db_types::store::users::user::{CreateOrganizationForm, CreateUserRow};
use core_library_db_types::store::users::user::UserError::UserFailAuth;
use core_web_types::infrastructure::users::forms::{AuthorizationUserForm, CreateUserResponse, RegistrationUserForm, SessionUserInfo};
use crate::state::{AppState};

pub async fn create_user_handler(
    State(state): State<AppState>,
    Json(form): Json<RegistrationUserForm>,
) -> Result<(StatusCode, Json<SessionUserInfo>), ApiErrorCode> {
    let password = Password::new(&form.password);
    if let Err(err) = password.check(&form.password_confirm) {
        return Err(ApiErrorCode::Password(err));
    }

    let hash = match password.hashing() {
        Ok(hash) => hash,
        Err(err) => {
            return Err(ApiErrorCode::Password(PasswordError::PasswordFailedToHash { reason: err.to_string() }));
        }
    };

    let request = CreateUserRow {
        password_hash: hash.to_string(),
        username: form.username,
        display_name: form.display_name,
        email: form.email,
    };
    match insert_user_query(request, state.db.clone()).await {
        Ok(user_info) => {
            Ok((StatusCode::CREATED, Json(user_info)))
        }
        Err(err) => {
            tracing::error!("{}", err);
            Err(ApiErrorCode::Other { reason: err.to_string()})
        }
    }
}


