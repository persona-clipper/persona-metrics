use sqlx::PgPool;
use std::env;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;
use core_library_tsdb::types::client::VictoriaClient;
use url::Url;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub vm: Arc<Mutex<VictoriaClient>>,
}

impl AppState {
    pub async fn new() -> Self {
        let db_url = env::var("DATABASE_URL").expect("Failed to get env var `DATABASE_URL`");
        let db_connection = PgPool::connect(&db_url).await.unwrap();

        let vm_url = env::var("VM_URL").expect("Failed to get env var `VM_URL`");
        let vm_url = url::Url::from_str(&vm_url).expect("Failed to parse url");

        Self {
            db: db_connection,
            vm: Arc::new(Mutex::new(VictoriaClient::new(vm_url))),
        }
    }
}