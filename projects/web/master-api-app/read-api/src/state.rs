use sqlx::PgPool;
use std::env;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

impl AppState {
    pub async fn new() -> Self {
        let db_url = env::var("DATABASE_URL").expect("Failed to get env var `DATABASE_URL`");
        let db_connection = PgPool::connect(&db_url).await.unwrap();

        Self {
            db: db_connection,
        }
    }
}