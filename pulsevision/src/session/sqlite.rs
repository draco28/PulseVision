use std::path::Path;
use std::sync::Mutex;

use async_trait::async_trait;
use pulsehive_core::event::HiveEvent;
use rusqlite::{params, Connection};

use super::{Session, SessionId, SessionMetadata, SessionStatus, SessionStore, StoredEvent};
use crate::error::{Error, Result};

/// SQLite-backed session store for standalone mode.
///
/// Uses WAL mode for concurrent reads during writes.
/// Thread-safe via Mutex (rusqlite Connection is not Send).
pub struct SqliteSessionStore {
    conn: Mutex<Connection>,
}

impl SqliteSessionStore {
    /// Open or create a SQLite session store at the given path.
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let conn = Connection::open(path)
            .map_err(|e| Error::SessionStore(format!("Failed to open SQLite: {e}")))?;

        conn.pragma_update(None, "journal_mode", "WAL")
            .map_err(|e| Error::SessionStore(format!("Failed to set WAL mode: {e}")))?;

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS sessions (
                id TEXT PRIMARY KEY,
                created_at INTEGER NOT NULL,
                metadata_json TEXT NOT NULL DEFAULT '{}',
                event_count INTEGER NOT NULL DEFAULT 0,
                status TEXT NOT NULL DEFAULT 'recording'
            );
            CREATE TABLE IF NOT EXISTS events (
                session_id TEXT NOT NULL REFERENCES sessions(id),
                seq INTEGER NOT NULL,
                event_type TEXT NOT NULL,
                event_json TEXT NOT NULL,
                timestamp_ms INTEGER NOT NULL,
                PRIMARY KEY (session_id, seq)
            );
            CREATE INDEX IF NOT EXISTS idx_events_timestamp ON events(timestamp_ms);
            CREATE INDEX IF NOT EXISTS idx_events_type ON events(event_type);",
        )
        .map_err(|e| Error::SessionStore(format!("Failed to create tables: {e}")))?;

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    fn extract_event_type(event: &HiveEvent) -> String {
        if let Ok(json) = serde_json::to_value(event) {
            json.get("type")
                .and_then(|t| t.as_str())
                .unwrap_or("unknown")
                .to_string()
        } else {
            "unknown".to_string()
        }
    }

    fn extract_timestamp(event: &HiveEvent) -> u64 {
        if let Ok(json) = serde_json::to_value(event) {
            json.get("timestamp_ms")
                .and_then(|t| t.as_u64())
                .unwrap_or(0)
        } else {
            0
        }
    }
}

#[async_trait]
impl SessionStore for SqliteSessionStore {
    async fn create_session(&self, metadata: SessionMetadata) -> Result<SessionId> {
        let session_id = SessionId::new();
        let now = pulsehive_core::event::now_ms();
        let metadata_json = serde_json::to_string(&metadata)
            .map_err(|e| Error::SessionStore(e.to_string()))?;

        let conn = self.conn.lock().map_err(|e| Error::SessionStore(e.to_string()))?;
        conn.execute(
            "INSERT INTO sessions (id, created_at, metadata_json, status) VALUES (?1, ?2, ?3, 'recording')",
            params![session_id.to_string(), now, metadata_json],
        )
        .map_err(|e| Error::SessionStore(format!("Failed to create session: {e}")))?;

        Ok(session_id)
    }

    async fn store_event(&self, session_id: SessionId, event: &HiveEvent) -> Result<()> {
        let event_type = Self::extract_event_type(event);
        let event_json = serde_json::to_string(event)
            .map_err(|e| Error::SessionStore(e.to_string()))?;
        let timestamp_ms = Self::extract_timestamp(event);
        let sid = session_id.to_string();

        let conn = self.conn.lock().map_err(|e| Error::SessionStore(e.to_string()))?;

        let seq: u64 = conn
            .query_row(
                "SELECT COALESCE(MAX(seq), 0) + 1 FROM events WHERE session_id = ?1",
                params![sid],
                |row| row.get(0),
            )
            .map_err(|e| Error::SessionStore(format!("Failed to get sequence: {e}")))?;

        conn.execute(
            "INSERT INTO events (session_id, seq, event_type, event_json, timestamp_ms) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![sid, seq, event_type, event_json, timestamp_ms],
        )
        .map_err(|e| Error::SessionStore(format!("Failed to store event: {e}")))?;

        conn.execute(
            "UPDATE sessions SET event_count = event_count + 1 WHERE id = ?1",
            params![sid],
        )
        .map_err(|e| Error::SessionStore(format!("Failed to update event count: {e}")))?;

        Ok(())
    }

    async fn list_events(
        &self,
        session_id: SessionId,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<StoredEvent>> {
        let sid = session_id.to_string();
        let conn = self.conn.lock().map_err(|e| Error::SessionStore(e.to_string()))?;

        let mut stmt = conn
            .prepare(
                "SELECT seq, event_type, event_json, timestamp_ms FROM events WHERE session_id = ?1 ORDER BY seq ASC LIMIT ?2 OFFSET ?3",
            )
            .map_err(|e| Error::SessionStore(format!("Failed to prepare query: {e}")))?;

        let events = stmt
            .query_map(params![sid, limit as i64, offset as i64], |row| {
                Ok(StoredEvent {
                    seq: row.get(0)?,
                    event_type: row.get(1)?,
                    event_json: row.get(2)?,
                    timestamp_ms: row.get(3)?,
                })
            })
            .map_err(|e| Error::SessionStore(format!("Failed to query events: {e}")))?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| Error::SessionStore(format!("Failed to read events: {e}")))?;

        Ok(events)
    }

    async fn list_sessions(&self) -> Result<Vec<Session>> {
        let conn = self.conn.lock().map_err(|e| Error::SessionStore(e.to_string()))?;

        let mut stmt = conn
            .prepare("SELECT id, created_at, metadata_json, event_count, status FROM sessions ORDER BY created_at DESC")
            .map_err(|e| Error::SessionStore(format!("Failed to prepare query: {e}")))?;

        let sessions = stmt
            .query_map([], |row| {
                let id_str: String = row.get(0)?;
                let metadata_json: String = row.get(2)?;
                let status_str: String = row.get(4)?;

                Ok(Session {
                    id: SessionId(id_str.parse().unwrap_or_default()),
                    created_at: row.get(1)?,
                    metadata: serde_json::from_str(&metadata_json).unwrap_or(SessionMetadata {
                        substrate_path: None,
                        description: None,
                    }),
                    event_count: row.get(3)?,
                    status: if status_str == "completed" {
                        SessionStatus::Completed
                    } else {
                        SessionStatus::Recording
                    },
                })
            })
            .map_err(|e| Error::SessionStore(format!("Failed to query sessions: {e}")))?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| Error::SessionStore(format!("Failed to read sessions: {e}")))?;

        Ok(sessions)
    }

    async fn get_session(&self, id: SessionId) -> Result<Option<Session>> {
        let sid = id.to_string();
        let conn = self.conn.lock().map_err(|e| Error::SessionStore(e.to_string()))?;

        let result = conn.query_row(
            "SELECT id, created_at, metadata_json, event_count, status FROM sessions WHERE id = ?1",
            params![sid],
            |row| {
                let id_str: String = row.get(0)?;
                let metadata_json: String = row.get(2)?;
                let status_str: String = row.get(4)?;

                Ok(Session {
                    id: SessionId(id_str.parse().unwrap_or_default()),
                    created_at: row.get(1)?,
                    metadata: serde_json::from_str(&metadata_json).unwrap_or(SessionMetadata {
                        substrate_path: None,
                        description: None,
                    }),
                    event_count: row.get(3)?,
                    status: if status_str == "completed" {
                        SessionStatus::Completed
                    } else {
                        SessionStatus::Recording
                    },
                })
            },
        );

        match result {
            Ok(session) => Ok(Some(session)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(Error::SessionStore(format!("Failed to get session: {e}"))),
        }
    }

    async fn complete_session(&self, id: SessionId) -> Result<()> {
        let sid = id.to_string();
        let conn = self.conn.lock().map_err(|e| Error::SessionStore(e.to_string()))?;

        let rows = conn
            .execute(
                "UPDATE sessions SET status = 'completed' WHERE id = ?1",
                params![sid],
            )
            .map_err(|e| Error::SessionStore(format!("Failed to complete session: {e}")))?;

        if rows == 0 {
            return Err(Error::NotFound(format!("Session {id} not found")));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_store() -> (TempDir, SqliteSessionStore) {
        let dir = TempDir::new().unwrap();
        let store = SqliteSessionStore::new(dir.path().join("test_sessions.db")).unwrap();
        (dir, store)
    }

    fn make_test_event(event_type: &str) -> HiveEvent {
        let json = match event_type {
            "agent_started" => serde_json::json!({
                "type": "agent_started",
                "timestamp_ms": 1711500000000u64,
                "agent_id": "agent-1",
                "name": "test-agent",
                "kind": "llm"
            }),
            "llm_call_completed" => serde_json::json!({
                "type": "llm_call_completed",
                "timestamp_ms": 1711500001000u64,
                "agent_id": "agent-1",
                "model": "GLM-4.7",
                "duration_ms": 1500,
                "input_tokens": 200,
                "output_tokens": 50
            }),
            "agent_completed" => serde_json::json!({
                "type": "agent_completed",
                "timestamp_ms": 1711500002000u64,
                "agent_id": "agent-1",
                "outcome": { "status": "complete", "response": "Done" }
            }),
            _ => serde_json::json!({
                "type": "agent_started",
                "timestamp_ms": 1711500000000u64,
                "agent_id": "agent-1",
                "name": "default",
                "kind": "llm"
            }),
        };
        serde_json::from_value(json).unwrap()
    }

    #[tokio::test]
    async fn test_create_session() {
        let (_dir, store) = create_test_store();
        let meta = SessionMetadata {
            substrate_path: Some("./test.db".into()),
            description: Some("Test session".into()),
        };
        let id = store.create_session(meta).await.unwrap();
        assert_ne!(id.0, uuid::Uuid::nil());
    }

    #[tokio::test]
    async fn test_get_session() {
        let (_dir, store) = create_test_store();
        let meta = SessionMetadata {
            substrate_path: None,
            description: Some("My session".into()),
        };
        let id = store.create_session(meta).await.unwrap();

        let session = store.get_session(id).await.unwrap().unwrap();
        assert_eq!(session.id, id);
        assert_eq!(session.event_count, 0);
        assert_eq!(session.status, SessionStatus::Recording);
        assert_eq!(session.metadata.description, Some("My session".into()));
    }

    #[tokio::test]
    async fn test_get_session_not_found() {
        let (_dir, store) = create_test_store();
        let result = store.get_session(SessionId::new()).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_store_and_list_events() {
        let (_dir, store) = create_test_store();
        let session_id = store
            .create_session(SessionMetadata { substrate_path: None, description: None })
            .await.unwrap();

        store.store_event(session_id, &make_test_event("agent_started")).await.unwrap();
        store.store_event(session_id, &make_test_event("llm_call_completed")).await.unwrap();
        store.store_event(session_id, &make_test_event("agent_completed")).await.unwrap();

        let events = store.list_events(session_id, 100, 0).await.unwrap();
        assert_eq!(events.len(), 3);
        assert_eq!(events[0].seq, 1);
        assert_eq!(events[0].event_type, "agent_started");
        assert_eq!(events[1].seq, 2);
        assert_eq!(events[1].event_type, "llm_call_completed");
        assert_eq!(events[2].seq, 3);
        assert_eq!(events[2].event_type, "agent_completed");
        assert_eq!(events[0].timestamp_ms, 1711500000000);

        let parsed: serde_json::Value = serde_json::from_str(&events[0].event_json).unwrap();
        assert_eq!(parsed["type"], "agent_started");
        assert_eq!(parsed["name"], "test-agent");
    }

    #[tokio::test]
    async fn test_list_events_pagination() {
        let (_dir, store) = create_test_store();
        let session_id = store
            .create_session(SessionMetadata { substrate_path: None, description: None })
            .await.unwrap();

        for _ in 0..5 {
            store.store_event(session_id, &make_test_event("agent_started")).await.unwrap();
        }

        let page1 = store.list_events(session_id, 2, 0).await.unwrap();
        assert_eq!(page1.len(), 2);
        assert_eq!(page1[0].seq, 1);

        let page2 = store.list_events(session_id, 2, 2).await.unwrap();
        assert_eq!(page2.len(), 2);
        assert_eq!(page2[0].seq, 3);

        let page3 = store.list_events(session_id, 2, 4).await.unwrap();
        assert_eq!(page3.len(), 1);
        assert_eq!(page3[0].seq, 5);
    }

    #[tokio::test]
    async fn test_event_count_increments() {
        let (_dir, store) = create_test_store();
        let session_id = store
            .create_session(SessionMetadata { substrate_path: None, description: None })
            .await.unwrap();

        store.store_event(session_id, &make_test_event("agent_started")).await.unwrap();
        store.store_event(session_id, &make_test_event("llm_call_completed")).await.unwrap();

        let session = store.get_session(session_id).await.unwrap().unwrap();
        assert_eq!(session.event_count, 2);
    }

    #[tokio::test]
    async fn test_list_sessions() {
        let (_dir, store) = create_test_store();
        for i in 0..3 {
            store.create_session(SessionMetadata {
                substrate_path: None,
                description: Some(format!("Session {i}")),
            }).await.unwrap();
        }

        let sessions = store.list_sessions().await.unwrap();
        assert_eq!(sessions.len(), 3);
        assert!(sessions[0].created_at >= sessions[1].created_at);
    }

    #[tokio::test]
    async fn test_complete_session() {
        let (_dir, store) = create_test_store();
        let session_id = store
            .create_session(SessionMetadata { substrate_path: None, description: None })
            .await.unwrap();

        let session = store.get_session(session_id).await.unwrap().unwrap();
        assert_eq!(session.status, SessionStatus::Recording);

        store.complete_session(session_id).await.unwrap();

        let session = store.get_session(session_id).await.unwrap().unwrap();
        assert_eq!(session.status, SessionStatus::Completed);
    }

    #[tokio::test]
    async fn test_complete_nonexistent_session() {
        let (_dir, store) = create_test_store();
        let result = store.complete_session(SessionId::new()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_multiple_sessions_isolated() {
        let (_dir, store) = create_test_store();
        let meta = SessionMetadata { substrate_path: None, description: None };

        let s1 = store.create_session(meta.clone()).await.unwrap();
        let s2 = store.create_session(meta).await.unwrap();

        store.store_event(s1, &make_test_event("agent_started")).await.unwrap();
        store.store_event(s1, &make_test_event("agent_completed")).await.unwrap();
        store.store_event(s2, &make_test_event("agent_started")).await.unwrap();

        let e1 = store.list_events(s1, 100, 0).await.unwrap();
        let e2 = store.list_events(s2, 100, 0).await.unwrap();
        assert_eq!(e1.len(), 2);
        assert_eq!(e2.len(), 1);

        let session1 = store.get_session(s1).await.unwrap().unwrap();
        let session2 = store.get_session(s2).await.unwrap().unwrap();
        assert_eq!(session1.event_count, 2);
        assert_eq!(session2.event_count, 1);
    }

    #[tokio::test]
    async fn test_metadata_roundtrip() {
        let (_dir, store) = create_test_store();
        let meta = SessionMetadata {
            substrate_path: Some("/path/to/substrate.db".into()),
            description: Some("Integration test run".into()),
        };
        let session_id = store.create_session(meta).await.unwrap();

        let session = store.get_session(session_id).await.unwrap().unwrap();
        assert_eq!(session.metadata.substrate_path, Some("/path/to/substrate.db".into()));
        assert_eq!(session.metadata.description, Some("Integration test run".into()));
    }
}
