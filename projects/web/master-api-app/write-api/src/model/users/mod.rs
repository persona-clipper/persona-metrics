use std::hash::Hash;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use core_library_db_queries::write::users::insert_user_query;
use core_library_db_types::state::{ApiErrorResponse, ApiErrorCode};
use core_library_db_types::store::users::password::{Password, PasswordError};
use core_library_db_types::store::users::user::CreateUserRow;
use core_web_types::infrastructure::users::forms::RegistrationUserForm;
use crate::state::{AppState};

pub async fn create_user_handler(
    State(state): State<AppState>,
    Json(form): Json<RegistrationUserForm>,
) -> impl IntoResponse {
    let password = Password::new(&form.password);
    match password.check(&form.password_confirm) {
        Ok(_) => {},
        Err(PasswordError::NotEnoughLength) => {
            let err = ApiErrorResponse {
                code: ApiErrorCode::Password(PasswordError::NotEnoughLength),
                message: "Пароль недостаточно длинный (минимум 8 символов)".to_string(),
            };
            return (StatusCode::UNPROCESSABLE_ENTITY, Json(err)).into_response()
        },
        Err(PasswordError::Mismatch) => {
            let err = ApiErrorResponse {
                code: ApiErrorCode::Password(PasswordError::Mismatch),
                message: "Пароли не совпадают".to_string(),
            };
            return (StatusCode::UNPROCESSABLE_ENTITY, Json(err)).into_response()
        },
        _ => {}
    };

    match password.hashing() {
        Ok(_) => {}
        Err(err) => {
            tracing::error!("Failed to hash password: {}", err);
            let err = ApiErrorResponse {
                code: ApiErrorCode::Password(PasswordError::FailedToHash),
                message: "Не удалось захешировать пароль".to_string(),
            };
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(err)).into_response();
        }
    }

    let request = CreateUserRow {
        password_hash: password.to_string(),
        username: form.username,
        display_name: form.display_name,
        email: form.email,
    };
    match insert_user_query(request, state.db.clone()).await {
        Ok(response) => {
            (StatusCode::CREATED, Json(response)).into_response()
        }
        Err(err) => {
            tracing::error!("{}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(err.to_string())).into_response()
        }
    }
    // let response = insert_user().await;
}



