use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use core_library_db_queries::write::users::insert_user;
use core_library_db_types::state::{ApiErrorResponse, ApiErrorCode};
use core_library_db_types::store::users::password::{Password, PasswordError};
use core_library_db_types::store::users::user::CreateUserRow;
use core_web_types::modules::users::forms::RegistrationUserForm;
use crate::state::{AppState};

pub async fn create_user_handler(
    State(state): State<AppState>,
    Json(form): Json<RegistrationUserForm>,
) -> impl IntoResponse {
    let password = match Password::new(&form.password, &form.password_confirm) {
        Ok(password) => {
            match password.hash() {
                Ok(hash) => hash,
                Err(err) => return (StatusCode::UNAUTHORIZED, err.to_string()).into_response(),
            }
        }
        Err(PasswordError::NotEnoughLength) => {
            let err = ApiErrorResponse {
                code: ApiErrorCode::PasswordNotEnoughLength,
                message: "Пароль недостаточно длинный (минимум 8 символов)".to_string(),
            };
            return (StatusCode::UNPROCESSABLE_ENTITY, Json(err)).into_response()
        },
        Err(PasswordError::Mismatch) => {
            let err = ApiErrorResponse {
                code: ApiErrorCode::PasswordMismatch,
                message: "Пароли не совпадают".to_string(),
            };
            return (StatusCode::UNPROCESSABLE_ENTITY, Json(err)).into_response()
        }
    };

    let request = CreateUserRow {
        password_hash: password.to_string(),
        username: form.username,
        display_name: form.display_name,
        email: form.email,
    };
    match insert_user(request, state.db.clone()).await {
        Ok(id) => (StatusCode::CREATED, Json(id)).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(err.to_string())).into_response(),
    }
    // let response = insert_user().await;
}



