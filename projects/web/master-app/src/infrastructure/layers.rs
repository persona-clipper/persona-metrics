use std::time::Duration;
use axum::response::Response;
use axum::Router;
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};
use tower_sessions::cookie::time::SignedDuration;

pub fn init_session_layer() -> SessionManagerLayer<MemoryStore> {
    let session_store = MemoryStore::default();

    SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(SignedDuration::new(3600, 0)))
}