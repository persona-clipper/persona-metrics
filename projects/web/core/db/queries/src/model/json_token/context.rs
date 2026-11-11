use std::env;
use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum_extra::TypedHeader;
use headers::Authorization;
use headers::authorization::Bearer;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    /// `id` пользователя
    pub sub: i32,
    pub username: String,
    /// Срок жизни JWT токена
    pub exp: u64,
}

#[derive(Debug)]
pub struct AuthClaims(pub Claims);

impl<T> FromRequestParts<T> for AuthClaims
where
    T: Send + Sync
{
    type Rejection = (http::StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &T) -> Result<Self, Self::Rejection> {
        let header  =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|e| {
                    tracing::error!("Claim JWT error: {}", e.to_string());
                    (StatusCode::UNAUTHORIZED, e.to_string())
                })?;

        let access_token = env::var("JWT_SECRET")
            .expect("Failed to get `JWT_SECRET`")
            .into_bytes();

        let jwt_data = decode::<Claims>(
            header.token(),
            &DecodingKey::from_secret(&access_token),
            &Validation::default(),
        )
            .map_err(|e| {
                tracing::error!("Claim JWT error: {}", e.to_string());
                (StatusCode::UNAUTHORIZED, e.to_string())
            })?;

        Ok(AuthClaims(jwt_data.claims))
    }
}