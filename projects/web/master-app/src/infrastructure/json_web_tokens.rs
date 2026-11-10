use std::time::{SystemTime, UNIX_EPOCH};
use askama::filters::json;
use jsonwebtoken::{encode, EncodingKey, Header};
use core_library_db_queries::model::json_token::context::Claims;
use core_web_types::infrastructure::users::forms::SessionUserInfo;

pub fn generate_json_web_token(
    user_info: &SessionUserInfo,
    jwt_secret: String,
) -> Result<String, String> {
    let expiry = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_secs();

    let claims = Claims {
        sub: user_info.id,
        username: user_info.username.clone(),
        exp: expiry + 800,
    };

    let secret = EncodingKey::from_secret(&jwt_secret.into_bytes());

    let encoding_claims = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &secret,
    ).map_err(|e| e.to_string())?;

    Ok(encoding_claims)
}