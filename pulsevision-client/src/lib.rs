//! PulseVision client — WebSocket exporter for PulseHive.
//!
//! Connects to PulseVision's `/ws/ingest` endpoint and forwards
//! all HiveEvents in real-time for visualization.
//!
//! ```rust,ignore
//! use pulsevision_client::WebSocketExporter;
//!
//! let hive = HiveMind::builder()
//!     .substrate_path("my.db")
//!     .llm_provider("openai", provider)
//!     .event_exporter(WebSocketExporter::new("ws://localhost:3333/ws/ingest"))
//!     .build()?;
//! ```

use async_trait::async_trait;
use futures_util::{SinkExt, StreamExt};
use pulsehive_core::event::HiveEvent;
use pulsehive_core::export::EventExporter;
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::Message;

/// WebSocket exporter that sends HiveEvents to PulseVision's /ws/ingest endpoint.
///
/// Events are sent via an internal mpsc channel to a background task that
/// manages the WebSocket connection with auto-reconnect.
pub struct WebSocketExporter {
    tx: mpsc::UnboundedSender<String>,
}

impl WebSocketExporter {
    /// Create a new WebSocketExporter that connects to the given URL.
    ///
    /// Spawns a background task that manages the WebSocket connection.
    /// The connection will auto-reconnect with exponential backoff if dropped.
    pub fn new(url: impl Into<String>) -> Self {
        let url: String = url.into();
        let (tx, rx) = mpsc::unbounded_channel::<String>();

        // Spawn background connection manager
        tokio::spawn(connection_loop(url, rx));

        Self { tx }
    }
}

#[async_trait]
impl EventExporter for WebSocketExporter {
    async fn export(&self, event: &HiveEvent) {
        if let Ok(json) = serde_json::to_string(event) {
            // Send to background task via channel (fire-and-forget)
            let _ = self.tx.send(json);
        }
    }

    async fn flush(&self) {
        // Channel is unbounded, messages are sent immediately
        // Nothing to flush explicitly
    }
}

/// Background task that manages the WebSocket connection.
///
/// Reads events from the mpsc channel and sends them over WebSocket.
/// Auto-reconnects with exponential backoff on connection failure.
async fn connection_loop(url: String, mut rx: mpsc::UnboundedReceiver<String>) {
    let mut retry_delay = std::time::Duration::from_secs(1);
    let max_delay = std::time::Duration::from_secs(30);

    loop {
        match tokio_tungstenite::connect_async(&url).await {
            Ok((ws_stream, _)) => {
                tracing::info!(url = %url, "Connected to PulseVision");
                retry_delay = std::time::Duration::from_secs(1);

                let (mut write, _read) = StreamExt::split(ws_stream);

                while let Some(json) = rx.recv().await {
                    let msg = Message::Text(json.into());
                    if SinkExt::send(&mut write, msg).await.is_err() {
                        tracing::warn!("WebSocket send failed, reconnecting...");
                        break;
                    }
                }

                // If channel closed (exporter dropped), exit
                if rx.is_closed() {
                    tracing::info!("WebSocketExporter dropped, closing connection");
                    return;
                }
            }
            Err(e) => {
                tracing::warn!(
                    url = %url,
                    delay_secs = retry_delay.as_secs(),
                    error = %e,
                    "Failed to connect to PulseVision, retrying..."
                );
            }
        }

        // Wait before reconnecting
        tokio::time::sleep(retry_delay).await;
        retry_delay = (retry_delay * 2).min(max_delay);
    }
}
