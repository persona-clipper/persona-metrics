#[derive(Clone, Default)]
pub struct Api {
    pub read_url: String,
    pub write_url: String,
}

#[derive(Clone, Default)]
pub struct AppState {
    pub api: Api,
    pub http_client: reqwest::Client,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            api: Api {
                read_url: "http://localhost:1313".to_string(),
                write_url: "http://localhost:1212".to_string(),
            },
            http_client: Default::default(),
        }
    }
}
