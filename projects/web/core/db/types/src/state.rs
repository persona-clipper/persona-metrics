use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use compact_str::CompactString;
use serde::{Deserialize, Serialize};
use crate::store::users::password::PasswordError;

#[derive(Serialize, Deserialize)]
pub struct ApiInfo {
    pub name: String,
    pub version: String,
    pub status: String,
}