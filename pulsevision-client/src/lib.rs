// pulsevision-client: WebSocketExporter for PulseHive
// Full implementation in Sprint 2 / Sprint 5

use async_trait::async_trait;
use pulsehive_core::event::HiveEvent;
use pulsehive_core::export::EventExporter;

/// WebSocket exporter that sends HiveEvents to PulseVision's /ws/ingest endpoint.
pub struct WebSocketExporter {
    url: String,
}

impl WebSocketExporter {
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }
}

#[async_trait]
impl EventExporter for WebSocketExporter {
    async fn export(&self, event: &HiveEvent) {
        // TODO: Connect and send via tokio-tungstenite (Sprint 5)
        let _ = serde_json::to_string(event);
        tracing::debug!(url = %self.url, "WebSocketExporter: event queued");
    }

    async fn flush(&self) {
        tracing::debug!("WebSocketExporter: flush");
    }
}
