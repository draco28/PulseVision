# Software Requirements Specification (SRS)

**Product:** PulseVision
**Version:** 0.1.0
**Date:** 2026-03-27
**Standard:** IEEE 830-1998 (adapted)

---

## 1. Introduction

### 1.1 Purpose

This SRS defines the functional and non-functional requirements for PulseVision v0.1.0, a real-time observability platform for PulseHive multi-agent systems.

### 1.2 Scope

PulseVision provides two visualization views (Agent Flow DAG, Substrate Space 3D), a WebSocket event hub, REST APIs for substrate data, and event persistence. It operates in dual-mode: embeddable Rust crate or standalone server.

### 1.3 Definitions

| Term | Definition |
|------|------------|
| HiveEvent | One of 14 event types emitted by PulseHive during agent execution |
| Substrate | PulseDB knowledge base containing experiences, relations, and insights |
| Experience | A unit of learned knowledge stored in PulseDB with an Nd embedding (dimension configured per database) |
| Attractor | A high-importance experience that warps agent query trajectories |
| Collective | A namespace in PulseDB grouping related experiences |
| PCA | Principal Component Analysis — reduces Nd embeddings to 3d for visualization (N read from PulseDB at runtime) |

---

## 2. Functional Requirements

### 2.1 Backend — Substrate Access

| ID | Requirement | Priority | Traces To |
|----|------------|----------|----------|
| FR-001 | System shall open PulseDB in read-only mode (standalone) or accept shared Arc<PulseDB> (embedded) | High | PRD Feature 3 |
| FR-002 | System shall list experiences with pagination (limit, offset) via PulseDB list_experiences API | High | PRD Feature 2 |
| FR-003 | System shall list relations with pagination via PulseDB list_relations API | High | PRD Feature 2 |
| FR-004 | System shall list insights with pagination via PulseDB list_insights API | Medium | PRD Feature 2 |
| FR-005 | System shall list collectives via PulseDB list_collectives API | High | PRD Feature 2 |
| FR-006 | System shall compute PCA projection (Nd to 3d, dimension read from PulseDB) for all experience embeddings | High | PRD Feature 2 |
| FR-007 | System shall cache PCA transform matrix and recompute when new experiences arrive (debounced 1s) | High | PRD Feature 2 |
| FR-008 | System shall compute attractor dynamics: strength = importance * confidence * (1 + log(applications + 1)) | High | PRD Feature 2 |
| FR-009 | System shall detect substrate changes via ChangePoller (standalone, 100ms interval) or WatchStream (embedded) | High | PRD Feature 2 |
| FR-010 | System shall fetch individual experience detail by ID | Medium | PRD Feature 2 |

### 2.2 Backend — REST API

| ID | Requirement | Priority | Traces To |
|----|------------|----------|----------|
| FR-011 | GET /api/substrate/experiences shall return paginated experience list | High | PRD Feature 2 |
| FR-012 | GET /api/substrate/experiences/:id shall return single experience detail | Medium | PRD Feature 2 |
| FR-013 | GET /api/substrate/embeddings shall return 3D PCA projections for all experiences | High | PRD Feature 2 |
| FR-014 | GET /api/substrate/relations shall return all relations | High | PRD Feature 2 |
| FR-015 | GET /api/substrate/insights shall return all insights | Medium | PRD Feature 2 |
| FR-016 | GET /api/substrate/attractors shall return computed attractor dynamics | High | PRD Feature 2 |
| FR-017 | GET /api/substrate/collectives shall return available collectives | High | PRD Feature 2 |
| FR-018 | GET /api/substrate/stats shall return summary statistics | Medium | PRD Feature 2 |
| FR-019 | GET /api/sessions shall return list of recording sessions | Medium | PRD Feature 4 |
| FR-020 | GET /api/sessions/:id/events shall return events for a session (paginated) | Medium | PRD Feature 4 |

### 2.3 Backend — WebSocket

| ID | Requirement | Priority | Traces To |
|----|------------|----------|----------|
| FR-021 | WS /ws/ingest shall accept HiveEvents from PulseHive (standalone mode) | High | PRD Feature 5 |
| FR-022 | WS /ws/events shall broadcast HiveEvents to all connected browsers | High | PRD Feature 5 |
| FR-023 | WS /ws/substrate shall broadcast substrate change notifications to browsers | High | PRD Feature 5 |
| FR-024 | System shall auto-create a session when first event arrives on /ws/ingest | High | PRD Feature 4 |
| FR-025 | System shall persist all received events to SessionStore | High | PRD Feature 4 |
| FR-026 | System shall send WebSocket ping every 30s to detect stale connections | Medium | PRD Feature 5 |

### 2.4 Backend — Event Persistence

| ID | Requirement | Priority | Traces To |
|----|------------|----------|----------|
| FR-027 | SessionStore trait shall support create_session, store_event, list_events, list_sessions, get_session, complete_session | High | PRD Feature 4 |
| FR-028 | SqliteSessionStore shall implement SessionStore using rusqlite | High | PRD Feature 4 |
| FR-029 | PostgresSessionStore shall implement SessionStore using sqlx | High | PRD Feature 4 |
| FR-030 | System shall store ALL event types including LlmTokenStreamed | High | PRD Feature 4 |

### 2.5 Frontend — Agent Flow View

| ID | Requirement | Priority | Traces To |
|----|------------|----------|----------|
| FR-031 | System shall render agent execution as a DAG using React Flow | High | PRD Feature 1 |
| FR-032 | System shall display AgentNode (rounded rect, blue), ToolCallNode (diamond, yellow), LlmCallNode (circle, green), ExperienceNode (star, teal) | High | PRD Feature 1 |
| FR-033 | System shall auto-layout DAG via Dagre (sequential: LTR, parallel: fork/join, loop: circular) | High | PRD Feature 1 |
| FR-034 | Clicking any node shall open a detail panel showing type-specific information | High | PRD Feature 1 |
| FR-035 | AgentNode detail shall show name, kind, outcome, total time, total tokens | High | PRD Feature 1 |
| FR-036 | LlmCallNode detail shall show model, duration_ms, input_tokens, output_tokens | High | PRD Feature 1 |
| FR-037 | ToolCallNode detail shall show tool_name, params (pretty-printed JSON), result_preview, duration_ms | High | PRD Feature 1 |
| FR-038 | ExperienceNode detail shall show experience_id, content_preview, experience_type, importance | High | PRD Feature 1 |
| FR-039 | System shall animate node appearance (fade-in), active state (pulse glow), completion (checkmark), error (red flash) | Medium | PRD Feature 1 |
| FR-040 | Stats bar shall display total tokens, total time, agent count | Medium | PRD Feature 1 |

### 2.6 Frontend — Substrate Space View

| ID | Requirement | Priority | Traces To |
|----|------------|----------|----------|
| FR-041 | System shall render experiences as InstancedMesh spheres in a React Three Fiber canvas | High | PRD Feature 2 |
| FR-042 | Sphere size shall represent importance (0.0-1.0), color shall represent ExperienceType (9 colors defined in SPEC) | High | PRD Feature 2 |
| FR-043 | Sphere opacity shall represent temporal decay (fresh=solid, old=transparent) | Medium | PRD Feature 2 |
| FR-044 | System shall render relations as LineSegments color-coded by RelationType | High | PRD Feature 2 |
| FR-045 | System shall render attractor gravity wells with glow proportional to strength and visible influence radius | High | PRD Feature 2 |
| FR-046 | System shall provide OrbitControls (click-drag rotate, scroll zoom, right-click pan, double-click focus) | High | PRD Feature 2 |
| FR-047 | Filter panel shall support: ExperienceType checkboxes, importance slider, time range, domain multi-select, show/hide toggles | Medium | PRD Feature 2 |
| FR-048 | Clicking any sphere shall open a detail panel showing full experience content and metadata | High | PRD Feature 2 |
| FR-049 | New experiences shall appear with a pop animation at their PCA-projected position | Medium | PRD Feature 2 |
| FR-050 | System shall display hover labels (first 30 chars of content) on experience spheres | Medium | PRD Feature 2 |

### 2.7 Frontend — Shared

| ID | Requirement | Priority | Traces To |
|----|------------|----------|----------|
| FR-051 | System shall provide tab navigation between Agent Flow and Substrate Space views | High | PRD Feature 1, 2 |
| FR-052 | System shall display connection status indicator (connected/disconnected/reconnecting) | High | PRD Feature 5 |
| FR-053 | System shall provide collective selector dropdown populated from /api/substrate/collectives | High | PRD Feature 2 |
| FR-054 | System shall auto-reconnect WebSocket on disconnection with exponential backoff | High | PRD Feature 5 |
| FR-055 | System shall use dark theme only | Medium | PRD Constraint |

### 2.8 Dual-Mode Configuration

| ID | Requirement | Priority | Traces To |
|----|------------|----------|----------|
| FR-056 | Library crate shall export `pulsevision::router(PulseVisionConfig) -> Router` | High | PRD Feature 3 |
| FR-057 | PulseVisionConfig shall accept SubstrateSource (Shared or File), EventSource (Channel or WebSocketIngest), SessionStore | High | PRD Feature 3 |
| FR-058 | CLI binary shall accept --substrate, --port arguments via clap | High | PRD Feature 3 |
| FR-059 | CLI binary shall open PulseDB with Config::read_only() | High | PRD Feature 3 |
| FR-060 | pulsevision-client crate shall provide WebSocketExporter implementing EventExporter trait | High | PRD Feature 5 |

---

## 3. Non-Functional Requirements

| ID | Requirement | Category | Target |
|----|------------|----------|--------|
| NFR-001 | Event relay latency (WebSocket to browser) | Performance | <100ms |
| NFR-002 | 3D rendering at 5000 experience nodes | Performance | 60fps |
| NFR-003 | Substrate initial load (1000 experiences) | Performance | <2s |
| NFR-004 | PCA projection (1000 points, 384d default; scales with dimension) | Performance | <500ms at 384d |
| NFR-005 | REST API response time | Performance | <200ms |
| NFR-006 | Frontend bundle size (excluding Three.js) | Performance | <500KB gzipped |
| NFR-007 | WebSocket max message size | Security | 1MB |
| NFR-008 | All REST query params validated | Security | 100% coverage |
| NFR-009 | PulseDB opened read-only (standalone) | Security | Enforced |
| NFR-010 | CORS configured via tower-http | Security | Localhost (standalone), host-managed (embedded) |
| NFR-011 | Structured logging via tracing crate | Observability | All components |
| NFR-012 | Backend test coverage (business logic) | Quality | 70% |
| NFR-013 | SessionStore test coverage | Quality | 100% |
| NFR-014 | cargo clippy zero warnings | Quality | Enforced in CI |
| NFR-015 | TypeScript strict mode, zero type errors | Quality | Enforced in CI |
| NFR-016 | Desktop Chrome and Firefox support | Compatibility | Required |
| NFR-017 | WebGL 2.0 required | Compatibility | Required |

---

## 4. Data Requirements

### 4.1 PulseDB Data (Read-Only)

| Entity | Key Fields | Access Pattern |
|--------|-----------|----------------|
| Experience | id, content, embedding(Nd), experience_type, importance, confidence, applications, domain, timestamp | List (paginated), Get by ID, PCA projection |
| ExperienceRelation | id, source_id, target_id, relation_type, strength | List all for collective |
| DerivedInsight | id, content, embedding, source_experience_ids, insight_type, confidence | List all for collective |
| Collective | id, name | List all |

### 4.2 PulseVision Data (Read-Write)

| Entity | Key Fields | Storage |
|--------|-----------|--------|
| Session | id (UUID v7), created_at, metadata_json, event_count, status | SQLite or PostgreSQL |
| StoredEvent | session_id, seq, event_type, event_json, timestamp_ms | SQLite or PostgreSQL |

---

## 5. Interface Requirements

### 5.1 PulseHive Interface

- Receives HiveEvents via WebSocket (/ws/ingest in standalone) or broadcast::Receiver (embedded)
- Events are JSON with `type` discriminator and `timestamp_ms` field
- 14 event types: agent_started, agent_completed, llm_call_started, llm_call_completed, llm_token_streamed, tool_call_started, tool_call_completed, tool_approval_requested, experience_recorded, relationship_inferred, insight_generated, substrate_perceived, embedding_computed, watch_notification

### 5.2 PulseDB Interface

- Opens via PulseDB::open(path, Config::read_only()) or receives Arc<PulseDB>
- Uses SubstrateProvider trait: list_experiences, list_relations, list_insights, list_collectives, get_experience
- Change detection: ChangePoller::poll_changes(since_seq) or watch_experiences() stream

### 5.3 Browser Interface

- REST API (JSON over HTTP) for initial data load
- WebSocket (JSON frames) for real-time updates
- Frontend served as static assets (Vite build) via tower-http ServeDir
