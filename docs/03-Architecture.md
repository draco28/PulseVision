# System Architecture Document

**Product:** PulseVision
**Version:** 0.1.0
**Date:** 2026-03-27
**Model:** C4 (adapted)

---

## 1. System Context (C4 Level 1)

```
┌──────────────────┐    ┌───────────────────┐
│  Developer        │    │  PulseHive SDK    │
│  (Browser)        │    │  (Agent Runtime)  │
└────────┬─────────┘    └─────────┬─────────┘
         │ REST/WS                │ WS/Channel
         ▼                        ▼
┌─────────────────────────────────────────┐
│         PulseVision Server              │
│         (Rust Axum)                     │
└────────────┬────────────┬───────────────┘
             │ File/Arc     │ SQL
             ▼              ▼
┌─────────────────┐  ┌─────────────────┐
│  PulseDB        │  │ SessionStore    │
│  (substrate)    │  │ (SQLite/PG)     │
└─────────────────┘  └─────────────────┘
```

### External Systems

| System | Relationship | Protocol |
|--------|-------------|----------|
| Developer Browser | Consumes visualizations | REST (JSON) + WebSocket (JSON frames) |
| PulseHive SDK | Produces HiveEvents | WebSocket push (/ws/ingest) or in-process broadcast channel |
| PulseDB | Knowledge substrate data source | Rust API (Arc<PulseDB> or file read-only) |
| SessionStore | Event persistence | SQLite (rusqlite) or PostgreSQL (sqlx) |

---

## 2. Container Diagram (C4 Level 2)

### 2.1 Deployment Mode: Standalone

```
┌──────────────────────────────────────────────────────────┐
│                pulsevision-cli process                    │
│                                                          │
│  ┌───────────────────┐  ┌──────────────────────────┐    │
│  │ Static File Server │  │ REST API Handlers        │    │
│  │ (tower-http)       │  │ /api/substrate/*         │    │
│  └───────────────────┘  │ /api/sessions/*          │    │
│                          └──────────────────────────┘    │
│  ┌───────────────────┐  ┌──────────────────────────┐    │
│  │ WebSocket Hub      │  │ Substrate Reader         │    │
│  │ /ws/ingest         │  │ (PulseDB read-only)      │    │
│  │ /ws/events         │  │ + ChangePoller (100ms)   │    │
│  │ /ws/substrate      │  └──────────────────────────┘    │
│  └───────────────────┘                                   │
│  ┌───────────────────┐  ┌──────────────────────────┐    │
│  │ PCA Engine         │  │ SessionStore (SQLite)     │    │
│  │ (nalgebra)         │  │ (rusqlite)                │    │
│  └───────────────────┘  └──────────────────────────┘    │
└──────────────────────────────────────────────────────────┘
```

### 2.2 Deployment Mode: Embedded

```
┌──────────────────────────────────────────────────────────┐
│                Host App (e.g., ProjectPulse)              │
│                                                          │
│  ┌─────────────────┐  ┌────────────────────────────┐    │
│  │ Host Routes      │  │ PulseVision Router         │    │
│  │ /api/tickets     │  │ /vision/api/substrate/*    │    │
│  │ /api/sprints     │  │ /vision/ws/events          │    │
│  └─────────────────┘  │ /vision/ws/substrate       │    │
│                        └────────────────────────────┘    │
│  ┌─────────────────┐                                    │
│  │ Shared PulseDB   │  Events via broadcast::Receiver   │
│  │ Arc<PulseDB>     │  SessionStore = Host's Postgres   │
│  └─────────────────┘                                    │
└──────────────────────────────────────────────────────────┘
```

---

## 3. Component Diagram (C4 Level 3)

### 3.1 Crate Structure

| Crate | Type | Purpose |
|-------|------|--------|
| `pulsevision` | Library | Core: Axum router, REST handlers, WS hub, substrate reader, PCA, attractor dynamics, SessionStore trait |
| `pulsevision-cli` | Binary | Standalone server: CLI args (clap), opens PulseDB read-only, SQLite session store |
| `pulsevision-client` | Library | WebSocketExporter implementing pulsehive-core::EventExporter for PulseHive consumers |

### 3.2 Module Map (pulsevision crate)

```
pulsevision/src/
├── lib.rs              ← pub fn router(config) -> Router
├── config.rs           ← PulseVisionConfig, SubstrateSource, EventSource
├── state.rs            ← AppState (shared across handlers)
├── error.rs            ← Error enum, IntoResponse impl
├── api/
│   ├── mod.rs          ← api_router() combining all routes
│   ├── substrate.rs    ← GET /api/substrate/* handlers
│   ├── projections.rs  ← PCA computation + GET /api/substrate/embeddings
│   └── attractors.rs   ← Attractor dynamics + GET /api/substrate/attractors
├── ws/
│   ├── mod.rs          ← ws_router() combining all WS routes
│   ├── events.rs       ← /ws/ingest + /ws/events handlers
│   └── substrate.rs    ← /ws/substrate handler + ChangePoller background task
├── db/
│   ├── mod.rs          ← SubstrateSource enum
│   └── reader.rs       ← Unified read API over both modes
└── session/
    ├── mod.rs          ← SessionStore trait, types
    ├── sqlite.rs       ← SqliteSessionStore (feature = "sqlite")
    └── postgres.rs     ← PostgresSessionStore (feature = "postgres")
```

### 3.3 Frontend Component Tree

```
frontend/src/
├── App.tsx                 ← Tab layout, toolbar, connection status
├── stores/
│   ├── eventStore.ts       ← HiveEvents, DAG nodes/edges, agent state
│   ├── spaceStore.ts       ← 3D positions, filters, selections
│   └── uiStore.ts          ← Active tab, selected node, connection status
├── hooks/
│   ├── useEventStream.ts   ← WebSocket /ws/events connection
│   └── useSubstrate.ts     ← REST fetch + /ws/substrate subscription
├── components/
│   ├── flow/
│   │   ├── FlowCanvas.tsx      ← React Flow + Dagre
│   │   ├── AgentNode.tsx       ← Rounded rect, blue
│   │   ├── ToolCallNode.tsx    ← Diamond, yellow
│   │   ├── LlmCallNode.tsx     ← Circle, green
│   │   ├── ExperienceNode.tsx  ← Star, teal
│   │   └── DetailPanel.tsx     ← Slide-in side panel
│   ├── space/
│   │   ├── SubstrateCanvas.tsx ← R3F Canvas + OrbitControls
│   │   ├── ExperienceCloud.tsx ← InstancedMesh spheres
│   │   ├── RelationLines.tsx   ← LineSegments
│   │   ├── AttractorField.tsx  ← Glow shader + influence radius
│   │   ├── FilterPanel.tsx     ← Type/importance/time/domain filters
│   │   └── NodeDetailPanel.tsx ← Experience detail on click
│   └── shared/
│       ├── StatsBar.tsx        ← Token totals, time, agent count
│       └── Toolbar.tsx         ← Connection status, collective selector, tabs
```

---

## 4. Data Flow

### 4.1 Live Session (Agent Running)

```
1. PulseHive emits HiveEvent
2. EventExporter serializes to JSON
3. WebSocket push to /ws/ingest (standalone) or broadcast channel (embedded)
4. PulseVision server:
   a. Persists event to SessionStore
   b. Broadcasts to all /ws/events subscribers
5. Browser receives event:
   a. eventStore processes: creates/updates FlowNode, FlowEdge
   b. React Flow re-renders DAG with new node
6. Meanwhile, PulseDB records experiences
7. ChangePoller (standalone) or WatchStream (embedded) detects change
8. Server fetches new experience, projects via PCA
9. Broadcasts to /ws/substrate
10. Browser updates spaceStore, 3D sphere materializes
```

### 4.2 Post-Hoc Analysis

```
1. User runs: pulsevision --substrate ./path.db --port 3333
2. Server opens PulseDB read-only
3. GET /api/substrate/experiences loads all experiences
4. GET /api/substrate/embeddings returns PCA 3D projections
5. GET /api/substrate/relations loads all relations
6. GET /api/substrate/attractors computes gravity wells
7. Frontend renders full Substrate Space view
8. User explores, filters, clicks nodes
```

---

## 5. Technology Decisions

| Decision | Choice | Rationale | Alternatives Considered |
|----------|--------|-----------|------------------------|
| Backend language | Rust | Ecosystem consistency (PulseHive, PulseDB are Rust) | Go, Node.js |
| Web framework | Axum | Async, tower middleware, WebSocket support | Actix-web, warp |
| Frontend framework | React 18 | React Flow and R3F are React-based | Vue, Svelte |
| DAG visualization | React Flow | Mature, customizable nodes, MIT license | D3.js, vis.js |
| 3D rendering | React Three Fiber | React integration, Three.js ecosystem | raw Three.js, Babylon.js |
| State management | Zustand | Lightweight, works with R3F (no context re-renders) | Redux, Jotai |
| Dimensionality reduction | PCA (nalgebra) | Fast, deterministic, sufficient for v1 | UMAP (deferred to v2) |
| Event persistence | SessionStore trait | Supports both deployment modes | Fixed SQLite only |
| Build tool | Vite | Fast HMR, ESM-native, TypeScript support | webpack, esbuild |

---

## 6. Concurrency Model

### 6.1 Tokio Runtime

| Component | Task Type | Notes |
|-----------|----------|-------|
| Axum HTTP handlers | tokio::spawn per request | Standard Axum pattern |
| WebSocket connections | tokio::spawn per connection | Long-lived, bidirectional |
| ChangePoller | tokio::spawn (loop with 100ms sleep) | Standalone mode only |
| PCA computation | tokio::spawn_blocking | CPU-intensive, nalgebra math |
| SessionStore writes | tokio::spawn_blocking (SQLite) or async (PostgreSQL) | SQLite requires blocking |
| Event broadcast | tokio::sync::broadcast | Lock-free fan-out, capacity 256 |

### 6.2 Shared State

```rust
struct AppState {
    substrate: SubstrateReader,
    event_tx: broadcast::Sender<HiveEvent>,
    substrate_tx: broadcast::Sender<SubstrateChange>,
    session_store: Arc<dyn SessionStore>,
    pca_cache: Arc<RwLock<PcaCache>>,
    attractor_cache: Arc<RwLock<AttractorCache>>,
}
```

All state is `Send + Sync`. No mutex contention on hot paths — broadcast channels are lock-free, PCA cache uses RwLock (reads dominate).

---

## 7. Security Architecture

| Layer | Measure |
|-------|--------|
| Network | CORS via tower-http (localhost for standalone, host-managed for embedded) |
| Transport | HTTP for standalone (localhost), host's TLS for embedded |
| Input | Serde strict deserialization, query param validation, UUID format checks |
| Data access | PulseDB opened read-only (standalone). No mutations. |
| WebSocket | Max message size 1MB. 30s ping heartbeat. Stale connection cleanup. |
| Authentication | None (standalone). Host's middleware (embedded). |
| Secrets | Zero. No API keys, no tokens, no environment variables. |

---

## 8. Deployment Architecture

### 8.1 Standalone

```bash
cargo install pulsevision-cli
pulsevision --substrate ./my_project.db --port 3333
```

Single binary, ~15MB. Includes embedded frontend assets (Vite build). Zero dependencies at runtime.

### 8.2 Embedded

```rust
let vision = pulsevision::router(PulseVisionConfig {
    substrate: SubstrateSource::Shared(pulsedb_handle),
    event_source: EventSource::Channel(event_rx),
    session_store: Arc::new(PostgresSessionStore::new(pg_pool)),
    collective_id: None,
});

let app = Router::new()
    .nest("/vision", vision)
    .merge(host_routes);
```

Adds ~2MB to host binary (excluding PulseDB which host already has). Frontend assets served from the same binary.
