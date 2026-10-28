#[derive(Clone)]
pub struct AppState {
    pub api_url: String,
    pub http_client: reqwest::Client,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            api_url: "http://localhost:8000".to_string(),
            http_client: Default::default(),
        }
    }
}
