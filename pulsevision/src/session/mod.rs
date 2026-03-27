use async_trait::async_trait;
use pulsehive_core::event::HiveEvent;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::Result;

#[cfg(feature = "sqlite")]
pub mod sqlite;

#[cfg(feature = "postgres")]
pub mod postgres;

/// Unique session identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionId(pub Uuid);

impl SessionId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for SessionId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for SessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Metadata for a recording session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionMetadata {
    pub substrate_path: Option<String>,
    pub description: Option<String>,
}

/// A persisted session record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: SessionId,
    pub created_at: u64,
    pub metadata: SessionMetadata,
    pub event_count: u64,
    pub status: SessionStatus,
}

/// Session lifecycle status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SessionStatus {
    Recording,
    Completed,
}

/// A stored event with sequence number.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredEvent {
    pub seq: u64,
    pub event_type: String,
    pub event_json: String,
    pub timestamp_ms: u64,
}

/// Trait for persisting HiveEvent sessions.
#[async_trait]
pub trait SessionStore: Send + Sync {
    async fn create_session(&self, metadata: SessionMetadata) -> Result<SessionId>;
    async fn store_event(&self, session_id: SessionId, event: &HiveEvent) -> Result<()>;
    async fn list_events(&self, session_id: SessionId, limit: usize, offset: usize) -> Result<Vec<StoredEvent>>;
    async fn list_sessions(&self) -> Result<Vec<Session>>;
    async fn get_session(&self, id: SessionId) -> Result<Option<Session>>;
    async fn complete_session(&self, id: SessionId) -> Result<()>;
}

/// No-op session store for initial development. Discards all events.
pub struct NoopSessionStore;

#[async_trait]
impl SessionStore for NoopSessionStore {
    async fn create_session(&self, _metadata: SessionMetadata) -> Result<SessionId> {
        Ok(SessionId::new())
    }
    async fn store_event(&self, _session_id: SessionId, _event: &HiveEvent) -> Result<()> {
        Ok(())
    }
    async fn list_events(&self, _session_id: SessionId, _limit: usize, _offset: usize) -> Result<Vec<StoredEvent>> {
        Ok(vec![])
    }
    async fn list_sessions(&self) -> Result<Vec<Session>> {
        Ok(vec![])
    }
    async fn get_session(&self, _id: SessionId) -> Result<Option<Session>> {
        Ok(None)
    }
    async fn complete_session(&self, _id: SessionId) -> Result<()> {
        Ok(())
    }
}
