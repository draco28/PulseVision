use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use tokio::sync::broadcast;

use crate::session::{SessionId, SessionMetadata};
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/ws/ingest", get(ingest_upgrade))
        .route("/ws/events", get(events_upgrade))
}

async fn ingest_upgrade(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.max_message_size(1024 * 1024)
        .on_upgrade(move |socket| handle_ingest(socket, state))
}

async fn events_upgrade(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_events(socket, state))
}

async fn handle_ingest(mut socket: WebSocket, state: AppState) {
    tracing::info!("PulseHive ingest connection established");

    let mut session_id: Option<SessionId> = None;
    let seq = AtomicU64::new(0);

    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Text(text) => {
                match serde_json::from_str::<pulsehive_core::event::HiveEvent>(&text) {
                    Ok(event) => {
                        let sid = match session_id {
                            Some(id) => id,
                            None => {
                                let meta = SessionMetadata {
                                    substrate_path: None,
                                    description: Some("WebSocket ingest session".into()),
                                };
                                match state.session_store.create_session(meta).await {
                                    Ok(id) => {
                                        tracing::info!(session_id = %id, "Auto-created session");
                                        session_id = Some(id);
                                        id
                                    }
                                    Err(e) => {
                                        tracing::warn!("Failed to create session: {e}");
                                        continue;
                                    }
                                }
                            }
                        };

                        if let Err(e) = state.session_store.store_event(sid, &event).await {
                            tracing::warn!("Failed to persist event: {e}");
                        }

                        seq.fetch_add(1, Ordering::Relaxed);
                        let _ = state.event_tx.send(text.to_string());
                    }
                    Err(e) => {
                        tracing::warn!("Malformed HiveEvent: {e}");
                    }
                }
            }
            Message::Close(_) => break,
            _ => {}
        }
    }

    if let Some(sid) = session_id {
        let event_count = seq.load(Ordering::Relaxed);
        tracing::info!(session_id = %sid, events = event_count, "Ingest connection closed");
        if let Err(e) = state.session_store.complete_session(sid).await {
            tracing::warn!("Failed to complete session: {e}");
        }
    }
}

async fn handle_events(mut socket: WebSocket, state: AppState) {
    tracing::info!("Browser events subscriber connected");

    let mut rx = state.event_tx.subscribe();
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
                        tracing::warn!("Browser client lagged, skipped {n} events");
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

    tracing::info!("Browser events subscriber disconnected");
}
