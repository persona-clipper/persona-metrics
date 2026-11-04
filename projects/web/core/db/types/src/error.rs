use std::fmt::format;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use crate::infrastructure::model::{ToHttpCode, ToHumanMessage};
use crate::store::users::password::PasswordError;
use crate::store::users::user::UserError;

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ApiErrorCode {
    Password(PasswordError),
    User(UserError),
    Other { reason: String },
    // UserNotFound,
}

impl ToHttpCode for ApiErrorCode {
    fn http_code(&self) -> StatusCode {
        match &self {
            ApiErrorCode::Password(err) => err.http_code(),
            ApiErrorCode::Other { .. } => StatusCode::BAD_GATEWAY,
            ApiErrorCode::User(err) => err.http_code(),
        }
    }
}

impl ToHumanMessage for ApiErrorCode {
    fn message(&self) -> String  {
        match self {
            ApiErrorCode::Password(err) => err.message(),
            ApiErrorCode::User(err) => err.message(),
            ApiErrorCode::Other { reason } => reason.clone()
        }
    }
}

// impl ApiErrorCode {
//     fn to_response(&self) -> ApiErrorResponse {
//         let response = ApiErrorResponse {
//             code: ApiErrorCode::Other,
//             message: "".to_string(),
//         };
//     }
// }

impl IntoResponse for ApiErrorCode {
    fn into_response(self) -> Response {
        let response = ApiErrorResponse {
            code: self.clone(),
            message: self.message().to_string(),
        };

        (self.http_code(), Json(response)).into_response()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ApiErrorResponse {
    pub code: ApiErrorCode,
    pub message: String,
}

impl IntoResponse for ApiErrorResponse {
    fn into_response(self) -> Response {
        (self.code.http_code(), Json(self)).into_response()
    }
}