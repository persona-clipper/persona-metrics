use std::env;
use sqlx::PgPool;

pub async fn create_database() -> PgPool {
    dotenvy::dotenv().ok();

    let url = env::var("DATABASE_URL").expect("Failed to get env var `DATABASE_URL`");

    PgPool::connect(&url).await.expect("Cant connect to database")
}

pub async fn create_session_layer() {

}