use compact_str::CompactString;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum ApiErrorCode {
    Unknown,
    PasswordMismatch,
    PasswordNotEnoughLength,
    UserNotFound,
}

#[derive(Serialize, Deserialize)]
pub struct ApiErrorResponse {
    pub code: ApiErrorCode,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApiInfo {
    pub name: String,
    pub version: String,
    pub status: String,
}