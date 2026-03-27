use std::sync::Arc;

use crate::db::SubstrateReader;
use crate::session::SessionStore;

/// Shared application state across all handlers.
#[derive(Clone)]
pub struct AppState {
    pub substrate: Arc<SubstrateReader>,
    pub event_tx: tokio::sync::broadcast::Sender<String>,
    pub substrate_tx: tokio::sync::broadcast::Sender<String>,
    pub session_store: Arc<dyn SessionStore>,
}
