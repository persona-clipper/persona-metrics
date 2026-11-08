use std::time::Duration;
use axum::response::Response;
use axum::Router;
use sqlx::PgPool;
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};
use tower_sessions::cookie::time::SignedDuration;
use crate::infrastructure::sessions::store::PostgresStore;

pub fn init_test_session_layer() -> SessionManagerLayer<MemoryStore> {
    let session_store = MemoryStore::default();

    SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(SignedDuration::new(3600, 0)))
}

pub async fn init_session_store(pool: PgPool) -> SessionManagerLayer<PostgresStore> {
    let session_store = PostgresStore::new(pool);

    session_store.migrate().await.expect("Failed to migrate sessions store");

    SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(SignedDuration::new(3600, 0)))
}