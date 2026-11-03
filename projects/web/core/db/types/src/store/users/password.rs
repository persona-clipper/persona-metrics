use std::fmt::{Display, Formatter};
use argon2::{Argon2, PasswordHash};
use axum::http::StatusCode;
use password_hash::{PasswordHasher, PasswordVerifier};
use password_hash::phc::SaltString;
use serde::{Deserialize, Serialize};
use crate::infrastructure::model::ToHttpCode;

pub struct Password(String);

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub enum PasswordError {
    PasswordMismatch,
    PasswordNotEnoughLength,
    PasswordFailedToHash,
    PasswordFailedToVerify,
}

impl ToHttpCode for PasswordError {
    fn http_code(&self) -> StatusCode {
        match &self {
            PasswordError::PasswordMismatch => StatusCode::UNAUTHORIZED,
            PasswordError::PasswordNotEnoughLength => StatusCode::BAD_REQUEST,
            PasswordError::PasswordFailedToHash => StatusCode::INTERNAL_SERVER_ERROR,
            PasswordError::PasswordFailedToVerify => StatusCode::UNAUTHORIZED,
        }
    }
}

impl Display for Password {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Password {
    pub fn check(&self, confirm: &str) -> Result<(), PasswordError> {
        if self.0.len() < 8 {
            return Err(PasswordError::PasswordNotEnoughLength);
        }
        if self.0 != confirm {
            return Err(PasswordError::PasswordMismatch);
        }

        Ok(())
    }

    pub fn new(input: &str) -> Self {
        Self(input.to_string())
    }
    // pub fn new(input_password: &str, confirm_password: &str) -> Result<Self, PasswordError> {
    //     if input_password.len() < 8 {
    //         return Err(PasswordError::NotEnoughLength)
    //     }
    //
    //     if input_password != confirm_password {
    //         return Err(PasswordError::Mismatch)
    //     }
    //
    //     Ok(Self(input_password.to_string()))
    // }

    pub fn hashing(&self) -> password_hash::Result<PasswordHash> {
        let salt = SaltString::generate();
        Argon2::default()
            .hash_password_with_salt(self.0.as_bytes(), salt.as_bytes())
    }

    pub fn verify(&self, hash: &str) -> bool {
        let parsed_hash = match PasswordHash::new(hash) {
            Ok(hash) => hash,
            Err(_) => return false,
        };

        Argon2::default()
            .verify_password(self.0.as_bytes(), &parsed_hash)
            .is_ok()
    }
}

