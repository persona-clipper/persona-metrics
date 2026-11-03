use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http;
use axum::http::request::Parts;
use serde::{Deserialize, Serialize};
use tower_http::classify::ServerErrorsFailureClass::StatusCode;
use tower_sessions::Session;
use core_library_db_types::store::users::user::User;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserInfo {
    pub id: i32,
    pub avatar_path: String,

}
#[derive(Clone, Debug)]
pub struct SessionContext {
    pub session: Session,
    pub message: Option<String>,
    pub user: Option<UserInfo>,
}

// #[async_trait]
impl<T> FromRequestParts<T> for SessionContext
where
    T: Send + Sync
{
    type Rejection = (http::StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &T) -> Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(parts, state)
            .await
            .map_err(|e| (http::StatusCode::INTERNAL_SERVER_ERROR, e.0.to_string()))?;

        let user: Option<UserInfo> = session
            .get("user")
            .await
            .map_err(|_| (http::StatusCode::NOT_FOUND, "Failed to get `user` from session".into()))?;

        let message: Option<String> = session
            .get("message")
            .await
            .map_err(|_| (http::StatusCode::NOT_FOUND, "Failed to get `message` from session".into()))?;

        Ok(Self {
            session,
            message,
            user,
        })
    }
}