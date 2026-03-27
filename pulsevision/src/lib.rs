pub mod api;
pub mod config;
pub mod db;
pub mod error;
pub mod session;
pub mod state;
pub mod ws;

use std::sync::Arc;

use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;

use config::PulseVisionConfig;
use db::SubstrateReader;
use state::AppState;

/// Create the PulseVision Axum router.
pub fn router(config: PulseVisionConfig) -> Router {
    let substrate = SubstrateReader::new(config.substrate)
        .expect("Failed to initialize substrate reader");

    let (event_tx, _) = tokio::sync::broadcast::channel::<String>(256);
    let (substrate_tx, _) = tokio::sync::broadcast::channel::<String>(256);

    let state = AppState {
        substrate: Arc::new(substrate),
        event_tx,
        substrate_tx,
        session_store: config.session_store,
    };

    Router::new()
        .merge(api::router())
        .merge(ws::router())
        .route("/api/health", get(health))
        .with_state(state)
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    substrate: String,
    version: String,
}

async fn health(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> Json<HealthResponse> {
    let substrate_status = if state.substrate.is_read_only() {
        "connected (read-only)"
    } else {
        "connected"
    };

    Json(HealthResponse {
        status: "healthy".to_string(),
        substrate: substrate_status.to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}
