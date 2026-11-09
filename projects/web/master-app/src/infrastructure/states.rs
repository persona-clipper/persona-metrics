use std::env;
use std::fs::{read, write};
use std::ptr::write_unaligned;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone)]
pub struct Api {
    pub read: ApiInfo,
    pub write: ApiInfo,
}

#[derive(Clone)]
pub struct ApiInfo {
    pub url: url::Url,
    pub jwt_secret_token: String,
}

#[derive(Clone)]
pub struct AppState {
    pub api: Api,
    pub http_client: reqwest::Client,
}

impl Default for AppState {
    fn default() -> Self {
        let write_api = env::var("WRITE_API_URL").expect("Failed to get env var `WRITE_API_URL`");
        let read_api = env::var("READ_API_URL").expect("Failed to get env var `READ_API_URL`");
        
        let jwt_secret_read = env::var("JWT_SECRET_READ").expect("");
        let jwt_secret_write = env::var("JWT_SECRET_WRITE").expect("");

        AppState {
            api: Api {
                read: ApiInfo {
                    url: Url::parse(&read_api).unwrap(),
                    jwt_secret_token: jwt_secret_read,
                },
                write: ApiInfo { 
                    url: Url::parse(&write_api).unwrap(), 
                    jwt_secret_token: jwt_secret_write 
                },
            },
            http_client: reqwest::Client::new(),
        }
    }
}



