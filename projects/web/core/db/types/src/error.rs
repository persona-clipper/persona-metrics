use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use crate::infrastructure::model::ToHttpCode;
use crate::store::users::password::PasswordError;
use crate::store::users::user::UserError;

#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(untagged)]
pub enum ApiErrorCode {
    Password(PasswordError),
    User(UserError),
    Other,
    // UserNotFound,
}

impl ToHttpCode for ApiErrorCode {
    fn http_code(&self) -> StatusCode {
        match &self {
            ApiErrorCode::Password(err) => err.http_code(),
            ApiErrorCode::Other => StatusCode::BAD_GATEWAY,
            ApiErrorCode::User(err) => err.http_code(),
        }
    }
}
#[derive(Serialize, Deserialize, Clone)]
pub struct ApiErrorResponse {
    pub code: ApiErrorCode,
    pub message: String,
}

impl IntoResponse for ApiErrorResponse {
    fn into_response(self) -> Response {
        let status = self.code.http_code();

        (status, Json(self)).into_response()
    }
}