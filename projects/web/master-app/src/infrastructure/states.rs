use std::env;
use std::fs::{read, write};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone)]
pub struct Api {
    pub read_url: url::Url,
    pub write_url: url::Url,
}

#[derive(Clone)]
pub struct AppState {
    pub api: Api,
    pub http_client: reqwest::Client,
}

impl AppState {
    pub fn new() -> AppState {
        let read_api_url = env::var("READ_API_URL").expect("Failed to get env var `READ_API_URL`");
        let write_api_url = env::var("WRITE_API_URL").expect("Failed to get env var `WRITE_API_URL`");

        AppState {
            api: Api {
                read_url: Url::parse(&read_api_url).unwrap(),
                write_url: Url::parse(&write_api_url).unwrap(),
            },
            http_client: reqwest::Client::new(),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        let read_api = env::var("READ_API_URL").expect("Failed to get env var `READ_API_URL`");
        let write_api = env::var("WRITE_API_URL").expect("Failed to get env var `WRITE_API_URL`");

        AppState {
            api: Api {
                read_url: Url::parse(&read_api).unwrap(),
                write_url: Url::parse(&write_api).unwrap(),
            },
            http_client: reqwest::Client::new(),
        }
    }
}



