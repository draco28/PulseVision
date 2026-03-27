use std::time::Duration;

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use tokio::sync::broadcast;

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/ws/substrate", get(substrate_upgrade))
}

async fn substrate_upgrade(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_substrate(socket, state))
}

/// Handle a browser WebSocket subscribing to substrate changes.
///
/// Substrate changes are broadcast from the ChangePoller background task
/// (started in lib.rs when using SubstrateSource::File) or from
/// in-process WatchStream (when using SubstrateSource::Shared).
async fn handle_substrate(mut socket: WebSocket, state: AppState) {
    tracing::info!("Browser substrate subscriber connected");

    let mut rx = state.substrate_tx.subscribe();
    let mut ping_interval = tokio::time::interval(Duration::from_secs(30));

    loop {
        tokio::select! {
            result = rx.recv() => {
                match result {
                    Ok(msg) => {
                        if socket.send(Message::Text(msg.into())).await.is_err() {
                            break;
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(n)) => {
                        tracing::warn!("Substrate subscriber lagged, skipped {n} changes");
                    }
                    Err(broadcast::error::RecvError::Closed) => break,
                }
            }
            _ = ping_interval.tick() => {
                if socket.send(Message::Ping(vec![].into())).await.is_err() {
                    break;
                }
            }
            msg = socket.recv() => {
                match msg {
                    Some(Ok(Message::Close(_))) | None => break,
                    _ => {}
                }
            }
        }
    }

    tracing::info!("Browser substrate subscriber disconnected");
}
