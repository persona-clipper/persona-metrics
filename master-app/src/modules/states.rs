
pub struct AppState {
    pub api_url: String
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            api_url: "http://localhost:8000".to_string()
        }
    }
}