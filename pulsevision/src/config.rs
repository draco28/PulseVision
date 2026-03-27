use std::sync::Arc;

use pulsedb::PulseDB;

use crate::session::SessionStore;

/// How PulseVision accesses the PulseDB substrate.
pub enum SubstrateSource {
    /// Shared instance from host app (embedded mode).
    /// Uses in-process watch streams for real-time updates.
    Shared(Arc<PulseDB>),

    /// File path to open in read-only mode (standalone mode).
    /// Uses ChangePoller for cross-process change detection.
    File { path: String },
}

/// How PulseVision receives HiveEvents.
pub enum EventSource {
    /// In-process broadcast channel from host app (embedded mode).
    Channel(tokio::sync::broadcast::Receiver<pulsehive_core::event::HiveEvent>),

    /// WebSocket ingest endpoint (standalone mode).
    /// PulseHive instances connect to /ws/ingest.
    WebSocketIngest,
}

/// Configuration for the PulseVision router.
pub struct PulseVisionConfig {
    /// How to access the PulseDB substrate.
    pub substrate: SubstrateSource,

    /// How to receive HiveEvents.
    pub event_source: EventSource,

    /// Session store for event persistence.
    pub session_store: Arc<dyn SessionStore>,

    /// Collective ID to observe (None = discover from PulseDB).
    pub collective_id: Option<pulsedb::CollectiveId>,
}
