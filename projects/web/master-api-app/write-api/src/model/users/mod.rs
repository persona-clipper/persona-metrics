use std::hash::Hash;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use tracing::error_span;
use core_library_db_queries::read::users::user::check_user_auth;
use core_library_db_queries::write::users::insert_user_query;
use core_library_db_types::error::{ApiErrorCode, ApiErrorResponse};
use core_library_db_types::store::users::password::{Password, PasswordError};
use core_library_db_types::store::users::user::CreateUserRow;
use core_web_types::infrastructure::users::forms::{AuthorizationUserForm, CreateUserResponse, RegistrationUserForm};
use crate::state::{AppState};

pub async fn create_user_handler(
    State(state): State<AppState>,
    Json(form): Json<RegistrationUserForm>,
) -> Result<(StatusCode, Json<CreateUserResponse>), ApiErrorResponse> {
    let password = Password::new(&form.password);
    match password.check(&form.password_confirm) {
        Ok(_) => {},
        Err(PasswordError::PasswordNotEnoughLength) => {
            let err = ApiErrorResponse {
                code: ApiErrorCode::Password(PasswordError::PasswordNotEnoughLength),
                message: "Пароль недостаточно длинный (минимум 8 символов)".into(),
            };
            return Err(err);
        },
        Err(PasswordError::PasswordMismatch) => {
            let err = ApiErrorResponse {
                code: ApiErrorCode::Password(PasswordError::PasswordMismatch),
                message: "Пароли не совпадают".into(),
            };
            return Err(err);
        },
        Err(err) => {}
    };

    if let Err(err) = password.check(&form.password_confirm) {
        let mut response = ApiErrorResponse {
            code: ApiErrorCode::Password(err),
            message: String::new(),
        };
        match err {
            PasswordError::PasswordMismatch => {
                response.message = "Пароли не совпадают".into();
            },
            PasswordError::PasswordNotEnoughLength => {
                response.message = "Не достаточная длина пароля".into();
            },
            _ => {
                response.message = format!("Что-то пошло не так: {:?}", err);
            }
        }

        return Err(response);
    }

    let hash = match password.hashing() {
        Ok(hash) => hash,
        Err(err) => {
            tracing::error!("Failed to hash password: {}", err);
            let err = ApiErrorResponse {
                code: ApiErrorCode::Password(PasswordError::PasswordFailedToHash),
                message: "Не удалось захешировать пароль".to_string(),
            };
            return Err(err);
        }
    };

    let request = CreateUserRow {
        password_hash: hash.to_string(),
        username: form.username,
        display_name: form.display_name,
        email: form.email,
    };
    match insert_user_query(request, state.db.clone()).await {
        Ok(response) => {
            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(err) => {
            tracing::error!("{}", err);
            let err = ApiErrorResponse {
                code: ApiErrorCode::Other,
                message: err.to_string(),
            };
            Err(err)
        }
    }
}

// pub async fn auth_user_handler(
//     State(state): State<AppState>,
//     Json(form): Json<AuthorizationUserForm>,
// ) {
//     todo!("Доделать авторизацию");
//     check_user_auth(&form, state.db.clone()).await;
// }
// 


