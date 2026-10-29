use sqlx::PgPool;

pub async fn create_database(url: &str) -> Result<PgPool, sqlx::Error> {
    PgPool::connect(url).await
}