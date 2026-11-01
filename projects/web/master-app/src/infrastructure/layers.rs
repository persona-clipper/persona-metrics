use std::time::Duration;
use axum::Router;
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};
use tower_sessions::cookie::time::SignedDuration;

pub fn apply_session_layer(router: Router) -> Router {
    let session_store = MemoryStore::default();

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(SignedDuration::new(3600, 0)));

    router.layer(session_layer)
}