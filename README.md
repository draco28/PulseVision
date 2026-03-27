# PulseVision

**Real-time observability and visualization for PulseHive multi-agent systems.**

PulseVision transforms invisible agent reasoning and knowledge dynamics into interactive visual experiences. It provides two complementary views for debugging, understanding, and optimizing AI agent pipelines built with [PulseHive](https://github.com/pulsehive/pulsehive).

---

## Views

### Agent Flow

A real-time directed acyclic graph (DAG) showing agent execution flow. Nodes represent agents, tool calls, LLM interactions, and recorded experiences. Click any node to inspect timing, token usage, parameters, and results.

- Visualizes all 14 HiveEvent types
- Auto-layout via Dagre (sequential, parallel, loop topologies)
- Live animations as agents execute
- Per-node token counters with cost tracking

### Substrate Space

A 3D interactive visualization of PulseDB's embedding space. Every experience is a sphere positioned by PCA projection of its embedding vector (dimension auto-detected from PulseDB — 384, 768, 1536, or custom up to 4096), colored by type, sized by importance.

- Attractor gravity wells with glow effects
- Knowledge graph relation lines (color-coded by type)
- Filter by experience type, domain, importance, time range
- Camera orbit, zoom, pan, and double-click focus

---

## Architecture

PulseVision is distributed as a **Rust crate** (embeddable) and a **standalone binary**.

```
PulseHive ──WS──> PulseVision Server ──WS──> Browser
                        |
                   PulseDB (read-only)
```

### Embedded Mode

Add observability to any PulseHive-powered Axum application with one line:

```rust
let vision = pulsevision::router(PulseVisionConfig {
    substrate: SubstrateSource::Shared(pulsedb_handle),
    event_source: EventSource::Channel(event_rx),
    session_store: Arc::new(PostgresSessionStore::new(pg_pool)),
    collective_id: None,
});

let app = Router::new()
    .nest("/vision", vision)
    .merge(your_app_routes);
```

### Standalone Mode

Visualize local agent runs from the terminal:

```bash
cargo install pulsevision-cli
pulsevision --substrate ./my_project.db --port 3333
# Open http://localhost:3333
```

---

## Quick Start

### Connect PulseHive to PulseVision

```rust
use pulsevision_client::WebSocketExporter;

let hive = HiveMind::builder()
    .substrate_path("my_project.db")
    .llm_provider("openai", provider)
    .event_exporter(WebSocketExporter::new("ws://localhost:3333/ws/ingest"))
    .build()?;
```

All 14 HiveEvent types stream to PulseVision in real-time as JSON.

---

## Tech Stack

| Component | Technology |
|-----------|-----------|
| Backend | Rust, Axum 0.8, tokio |
| Frontend | React 18, TypeScript, Vite |
| Agent Flow | React Flow v12 |
| Substrate 3D | React Three Fiber, Three.js |
| State | Zustand |
| Math | nalgebra (PCA projection) |
| Storage | SQLite (standalone) / PostgreSQL (embedded) |

---

## Crates

| Crate | Purpose |
|-------|---------|
| `pulsevision` | Library — Axum router, REST/WS handlers, SessionStore trait |
| `pulsevision-cli` | Binary — Standalone server with CLI |
| `pulsevision-client` | Library — WebSocketExporter for PulseHive consumers |

---

## Development

```bash
# Backend
cargo build
cargo run -- --substrate ./tests/fixtures/test_substrate.db --port 3333
cargo test

# Frontend
cd frontend
npm install
npm run dev    # Vite dev server on :5173
npm test
```

---

## Performance Targets

| Metric | Target |
|--------|--------|
| Event relay latency | <100ms |
| 3D render (5000 nodes) | 60fps |
| Substrate initial load | <2s |
| PCA projection (1000 pts) | <500ms |
| REST API response | <200ms |

---

## Documentation

| Document | Description |
|----------|-------------|
| [SPEC.md](SPEC.md) | Full product specification |
| [docs/01-PRD.md](docs/01-PRD.md) | Product Requirements |
| [docs/02-SRS.md](docs/02-SRS.md) | Software Requirements Specification |
| [docs/03-Architecture.md](docs/03-Architecture.md) | System Architecture |
| [docs/04-Data-Model.md](docs/04-Data-Model.md) | Data Model |
| [docs/05-API-Spec.md](docs/05-API-Spec.md) | API Specification |
| [docs/06-UI-UX.md](docs/06-UI-UX.md) | UI/UX Guidelines |
| [docs/07-Security.md](docs/07-Security.md) | Security Plan |
| [docs/08-Testing.md](docs/08-Testing.md) | Testing Strategy |
| [docs/09-Deployment.md](docs/09-Deployment.md) | Deployment Guide |

---

## Ecosystem

PulseVision is the third product in the Pulse ecosystem:

| Product | Role |
|---------|------|
| [PulseDB](https://github.com/pulsehive/pulsedb) | Embedded agentic vector database |
| [PulseHive](https://github.com/pulsehive/pulsehive) | Multi-agent orchestration SDK |
| **PulseVision** | Real-time observability and visualization |
| DevStudio | CLI code agent (PulseVision consumer) |
| ProjectPulse | AI-native project management platform |

---

## License

AGPL-3.0-only
