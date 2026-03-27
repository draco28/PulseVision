# PulseVision — Product Specification

> **Version:** 0.2.0-spec
> **Status:** Design — SDK prerequisites complete, ready for implementation
> **Created:** March 2026
> **Updated:** March 2026
> **Product type:** Real-time observability and visualization platform
> **Dependencies:** PulseHive SDK v2.0.0+ (enriched events, EventExporter), PulseDB v0.4.0+ (list APIs, read-only mode)

---

## 1. Overview

PulseVision is a **real-time observability and visualization platform** for PulseHive multi-agent systems and PulseDB knowledge substrates. It provides two complementary views:

1. **Agent Flow** — A visual execution trace showing agents as a DAG (directed acyclic graph), with tool calls, LLM interactions, timing, token usage, and file changes. Like LangSmith, but for PulseHive.

2. **Substrate Space** — A 3D interactive visualization of PulseDB's embedding space, showing experiences as nodes in a "solar system" with attractor gravity wells, lens perception viewports, relation connections, and real-time decay animations. **No equivalent exists in any other agent framework.**

### Why PulseVision

| Problem | Solution |
|---------|----------|
| Text logs are hard to debug | Visual DAG shows exact agent flow |
| Can't see how agents perceive the substrate | 3D view shows what each lens "sees" |
| Can't understand embedding relationships | Spatial layout shows clusters, attractors, distances |
| No real-time visibility into multi-agent execution | Live WebSocket streaming with animated transitions |
| Token costs are invisible | Per-node token counters with cost estimation |

### Relationship to Other Products

| Product | Role |
|---------|------|
| **PulseHive SDK** | Emits HiveEvents via WebSocket (SDK change needed) |
| **PulseDB** | Provides substrate data (experiences, embeddings, relations) |
| **PulseVision** | Visualizes both (this product) |
| **ProjectPulse** | Project management (separate concern) |
| **DevStudio** | CLI code agent (a PulseHive consumer that PulseVision observes) |
| **PulseEval** | Future — evaluates/scores agent output quality (separate product) |

---

## 2. Architecture

```
┌──────────────────────────────────────────────────────────────────┐
│                    Browser (React + Three.js)                     │
│                                                                  │
│  ┌─────────────────────────┐  ┌────────────────────────────────┐ │
│  │   Agent Flow View       │  │   Substrate Space View         │ │
│  │   (React Flow DAG)      │  │   (React Three Fiber 3D)       │ │
│  │                         │  │                                │ │
│  │   ○ → ○ → ○ → ○        │  │     ◉  ·  ·                  │ │
│  │   Explorer Planner      │  │   ·  ☀  ·  ◉                │ │
│  │         Coder Tester    │  │     ◉  ·  ·  ·              │ │
│  │                         │  │                                │ │
│  │   Click: timing, tokens │  │   Click: content, metadata    │ │
│  │   Tool calls, outcomes  │  │   Attractors, relations       │ │
│  └────────────┬────────────┘  └────────────────┬───────────────┘ │
│               │ WebSocket                      │ REST API        │
└───────────────┼────────────────────────────────┼─────────────────┘
                │                                │
┌───────────────┼────────────────────────────────┼─────────────────┐
│               ▼                                ▼                 │
│         PulseVision API Server (Rust Axum)                       │
│                                                                  │
│  ┌─────────────────────┐  ┌──────────────────────────────────┐  │
│  │ WebSocket Hub       │  │ Substrate Reader                  │  │
│  │ - Receives events   │  │ - Opens PulseDB file              │  │
│  │   from PulseHive    │  │ - Reads experiences + embeddings  │  │
│  │ - Broadcasts to     │  │ - Computes attractor dynamics     │  │
│  │   browser clients   │  │ - Serves via REST API             │  │
│  └──────────┬──────────┘  └──────────┬───────────────────────┘  │
│             │                        │                           │
└─────────────┼────────────────────────┼───────────────────────────┘
              │ WebSocket              │ File read
              │                        │
┌─────────────┼────────────┐  ┌────────┼───────────────────────────┐
│             ▼            │  │        ▼                           │
│  PulseHive SDK           │  │  PulseDB substrate file            │
│  (HiveMind + agents)     │  │  (.devstudio/substrate.db)         │
│                          │  │                                    │
│  EventExporter trait     │  │  Experiences, Relations, Insights  │
│  → WebSocket exporter    │  │  Embeddings (384d vectors)         │
│                          │  │  Timestamps, metadata              │
└──────────────────────────┘  └────────────────────────────────────┘
```

### Tech Stack

| Component | Technology |
|-----------|-----------|
| **Frontend** | React 18, TypeScript |
| **Agent Flow DAG** | React Flow (reactflow.dev) |
| **Substrate 3D** | React Three Fiber + @react-three/drei |
| **State management** | Zustand (lightweight, works with R3F) |
| **WebSocket client** | Native WebSocket API |
| **Backend** | Rust, Axum, tokio |
| **WebSocket server** | axum::extract::ws |
| **PulseDB reader** | `pulsehive-db` crate (read-only mode) |
| **Build** | Vite (frontend), Cargo (backend) |

---

## 3. View 1: Agent Flow Visualizer

### What it shows

A directed acyclic graph (DAG) where each node is an agent, tool call, or LLM interaction. Edges show the execution flow. Nodes are color-coded by type and annotated with timing and token data.

### Node Types

| Node | Shape | Color | Data Shown |
|------|-------|-------|------------|
| **Agent (LLM)** | Rounded rectangle | Blue | name, model, total time, total tokens |
| **Agent (Sequential)** | Container with children | Gray border | name, child count, total time |
| **Agent (Parallel)** | Container with parallel children | Purple border | name, child count |
| **Agent (Loop)** | Container with iteration count | Orange border | name, iteration/max |
| **LLM Call** | Small circle | Green | model, duration_ms, token count |
| **Tool Call** | Diamond | Yellow | tool name, duration_ms |
| **Experience Recorded** | Star | Teal | experience_id, content preview |
| **Error** | Red octagon | Red | error message |

### Node Detail Panel (Click to Inspect)

Clicking any node opens a side panel showing:

**For Agent nodes** (from `agent_started` + `agent_completed` events):
- Agent name, kind (Llm/Sequential/Parallel/Loop)
- Outcome (Complete with response / Error with message / MaxIterationsReached)
- Total wall time (computed from timestamps)
- Total tokens (summed from child LLM calls)
- Experiences recorded during this agent's run
- *Note: system_prompt and lens config are NOT in events (deferred to v2.1) — PulseVision can fetch via PulseDB if needed*

**For LLM Call nodes** (from `llm_call_started` + `llm_call_completed` events):
- Model name
- Message count sent
- Duration (ms)
- Token usage: `input_tokens` + `output_tokens` (available in v2.0.0 events)
- Cost estimate (computed from model pricing × token counts)

**For Tool Call nodes** (from `tool_call_started` + `tool_call_completed` events):
- Tool name
- Parameters: `params` field (JSON string, pretty-printed in panel)
- Result preview: `result_preview` (first 200 chars — available in v2.0.0 events)
- Duration (ms)
- Approval status (from `tool_approval_requested` event if present)

**For Experience Recorded nodes** (from `experience_recorded` event):
- Experience ID
- Content preview: `content_preview` (first 200 chars — available in v2.0.0 events)
- Type: `experience_type` (e.g., "Generic", "Solution" — available in v2.0.0 events)
- Importance: `importance` (f32 — available in v2.0.0 events)
- Full content: fetch via PulseDB `get_experience(id)` on click
- Embedding dimensions: fetch via PulseDB on demand

### Real-Time Animation

When connected to a live PulseHive session:
- Nodes appear with a fade-in animation as agents start
- Active nodes pulse with a glow
- Edges draw as execution progresses
- Completed nodes get a checkmark overlay
- Errored nodes flash red
- Token counter increments live during LLM streaming

### Layout

- Sequential agents: left-to-right horizontal flow
- Parallel agents: vertical fork with simultaneous branches
- Loop agents: circular arrow with iteration counter
- Tool calls: small nodes branching down from the agent
- LLM calls: small nodes branching up from the agent

---

## 4. View 2: Substrate Space Visualizer

### What it shows

A 3D interactive visualization of the PulseDB embedding space. Every experience, relation, insight, and attractor is rendered as a node in 3D space, positioned by its embedding vector (reduced to 3D via UMAP or t-SNE).

### Visual Elements

#### Experiences (Spheres)

```
Size    = importance (0.0-1.0 → small-large)
Color   = ExperienceType
          Generic: #4A9EFF (blue)
          Solution: #4AFF7F (green)
          ErrorPattern: #FF4A4A (red)
          Difficulty: #FFA94A (orange)
          SuccessPattern: #7FFF4A (lime)
          UserPreference: #FF4AFF (magenta)
          ArchitecturalDecision: #4AFFFF (cyan)
          TechInsight: #FFD700 (gold)
          Fact: #C0C0C0 (silver)
Opacity = temporal decay (fresh = solid, old = transparent)
Label   = first 30 chars of content (on hover)
```

#### Attractors (Glowing Fields)

```
Position  = same as experience (attractors ARE experiences with high strength)
Glow      = strength (importance × confidence × reinforcement)
Radius    = visible sphere showing influence radius
Color     = warm gradient (yellow → orange → red based on warp_factor)
Animation = gentle pulsing proportional to strength
```

When an agent's query enters an attractor's radius, the attractor brightens and the query trajectory bends toward it (animated).

#### Lens Viewports (Cones)

```
Position  = origin of the lens query point
Direction = purpose_embedding direction
Angle     = attention_budget (wider = more experiences perceived)
Color     = semi-transparent agent color
Label     = agent name + domain focus
```

Shows exactly what each agent "sees" — experiences inside the cone are perceived, those outside are invisible to that agent.

#### Relations (Connecting Lines)

```
RelatedTo:    thin gray line
Supports:     green line with arrow
Contradicts:  red dashed line
Supersedes:   thick blue line with arrow
Implies:      thin blue dotted line
```

#### Insights (Cluster Halos)

```
Shape     = cluster halo around source experiences
Color     = golden glow
Label     = insight summary (first 50 chars)
Animation = gentle rotation
```

#### Real-Time Events

When connected live:
- **New experience stored**: Sphere materializes with a "pop" animation at its embedding position
- **Experience perceived**: Brief beam from lens cone to the experience node
- **Relation inferred**: Line draws between two nodes with a spark
- **Attractor warping**: Query trajectory visibly bends toward strong attractors
- **Decay**: Nodes slowly fade over time (accelerated for visual clarity)

### Camera Controls

- **Orbit**: Click-drag to rotate around the space
- **Zoom**: Scroll wheel to zoom in/out
- **Pan**: Right-click drag to pan
- **Focus**: Double-click a node to center and zoom to it
- **Reset**: Button to reset to default view
- **Time scrubber**: Slider to replay the substrate state at any point in time

### Dimensionality Reduction

PulseDB stores 384-dimensional embeddings. To render in 3D:

1. **Server-side**: Run UMAP (or t-SNE) on all experience embeddings when the substrate is loaded
2. **Output**: 3D coordinates (x, y, z) for each experience
3. **Update**: When new experiences arrive, project them using the existing UMAP transform
4. **Alternative**: PCA for faster but less clustered layout (user toggle)

### Filters

- Filter by ExperienceType (checkboxes)
- Filter by domain (multi-select)
- Filter by agent source (which agent created it)
- Filter by time range (slider)
- Filter by importance threshold (slider)
- Show/hide relations, insights, attractors, lenses (toggles)

---

## 5. SDK Prerequisites (Implemented)

All SDK changes needed by PulseVision are **shipped and published**. No further SDK work is required before building PulseVision.

### PulseHive v2.0.0 (Published on crates.io)

**HiveEvent is now JSON-serializable** — `#[derive(Serialize, Deserialize)]` with `#[serde(tag = "type", rename_all = "snake_case")]`. Every event can be transmitted over WebSocket with a single `serde_json::to_string(&event)`.

**All 14 events include `timestamp_ms: u64`** — epoch milliseconds, enabling accurate timeline reconstruction and animation.

**Enriched event data** — PulseVision gets everything it needs without follow-up queries:

| Event | New Fields (v2.0.0) | PulseVision Use |
|-------|---------------------|-----------------|
| `LlmCallCompleted` | `input_tokens`, `output_tokens` | Token counter, cost estimation |
| `ToolCallStarted` | `params` (JSON string) | Tool call inspection panel |
| `ToolCallCompleted` | `result_preview` (200 chars) | Tool result display |
| `ExperienceRecorded` | `content_preview`, `experience_type`, `importance` | 3D node rendering (size, color, label) |
| `RelationshipInferred` | `agent_id` | Agent correlation in flow view |
| `InsightGenerated` | `agent_id` | Agent correlation in flow view |

**EventExporter trait** (`pulsehive_core::export::EventExporter`):
```rust
#[async_trait]
pub trait EventExporter: Send + Sync {
    async fn export(&self, event: &HiveEvent);
    async fn flush(&self);
}
```

Register with HiveMind:
```rust
let hive = HiveMind::builder()
    .substrate_path("my.db")
    .llm_provider("openai", provider)
    .event_exporter(my_ws_exporter)  // ← PulseVision connector
    .build()?;
```

### PulseDB v0.4.0 (Published on crates.io)

**List APIs for full substrate enumeration:**
```rust
// All have default impls returning empty vecs (backward-compatible)
async fn list_experiences(&self, collective: CollectiveId, limit: usize, offset: usize) -> Result<Vec<Experience>>;
async fn list_relations(&self, collective: CollectiveId, limit: usize, offset: usize) -> Result<Vec<ExperienceRelation>>;
async fn list_insights(&self, collective: CollectiveId, limit: usize, offset: usize) -> Result<Vec<DerivedInsight>>;
```

**Read-only mode** — PulseVision opens the DB safely:
```rust
let config = Config::read_only();
let db = PulseDB::open("substrate.db", config)?;
// All mutations return PulseDBError::ReadOnly
```

**Enriched WatchEvent** — includes full Experience data on Created/Updated:
```rust
pub struct WatchEvent {
    pub experience_id: ExperienceId,
    pub collective_id: CollectiveId,
    pub event_type: WatchEventType,
    pub timestamp: Timestamp,
    pub experience: Option<Experience>,  // Populated for Created/Updated
}
```

### What PulseVision Needs to Implement

PulseVision only needs to provide a `WebSocketExporter` implementing `EventExporter`:
```rust
struct WebSocketExporter { /* tokio-tungstenite sender */ }

#[async_trait]
impl EventExporter for WebSocketExporter {
    async fn export(&self, event: &HiveEvent) {
        let json = serde_json::to_string(event).unwrap();
        self.sender.send(json).await.ok();
    }
    async fn flush(&self) {}
}
```

This lives in PulseVision's server crate, not in PulseHive.

---

## 6. API Design (PulseVision Server)

### REST Endpoints

```
GET  /api/substrate/experiences       → List all experiences (paginated)
GET  /api/substrate/experiences/:id   → Single experience detail
GET  /api/substrate/embeddings        → All embeddings with 3D projections
GET  /api/substrate/relations         → All relations
GET  /api/substrate/insights          → All insights
GET  /api/substrate/attractors        → Computed attractor dynamics
GET  /api/substrate/stats             → Summary statistics
```

### WebSocket Endpoints

```
WS  /ws/events          → Real-time HiveEvent stream (from PulseHive)
WS  /ws/substrate        → Real-time substrate changes (from PulseDB Watch)
```

### Event Wire Format (PulseHive v2.0.0 actual format)

Events arrive as flat JSON with `type` discriminator (no nested `data` object):

```json
{
  "type": "llm_call_completed",
  "timestamp_ms": 1711500000000,
  "agent_id": "019d2475-ab8c-7ea2-ae48-32236c1ddfea",
  "model": "GLM-4.7",
  "duration_ms": 1500,
  "input_tokens": 200,
  "output_tokens": 50
}
```

```json
{
  "type": "tool_call_started",
  "timestamp_ms": 1711500001000,
  "agent_id": "019d2475-ab8c-7ea2-ae48-32236c1ddfea",
  "tool_name": "file_read",
  "params": "{\"path\":\"src/main.rs\"}"
}
```

```json
{
  "type": "experience_recorded",
  "timestamp_ms": 1711500002000,
  "experience_id": "019d2475-ab92-7a82-af17-cc293d6a5c4e",
  "agent_id": "019d2475-ab8c-7ea2-ae48-32236c1ddfea",
  "content_preview": "Task: Analyze codebase\n\nResult: Found 12 source files...",
  "experience_type": "Generic { category: Some(\"task_completion\") }",
  "importance": 0.7
}
```

All 14 HiveEvent variants serialize to this flat JSON format via `serde_json::to_string(&event)`. No custom parsing needed — PulseVision's frontend can use `event.type`, `event.timestamp_ms`, `event.agent_id` directly.

---

## 7. Frontend Components

### Agent Flow View

```
<AgentFlowView>
├── <FlowCanvas>          (React Flow canvas)
│   ├── <AgentNode>       (per agent)
│   ├── <ToolCallNode>    (per tool call)
│   ├── <LlmCallNode>    (per LLM call)
│   └── <ExperienceNode> (per recorded experience)
├── <DetailPanel>          (side panel on click)
│   ├── <AgentDetail>
│   ├── <ToolCallDetail>
│   └── <LlmCallDetail>
├── <TimelineBar>          (horizontal timeline at bottom)
└── <StatsBar>             (total tokens, total time, agent count)
```

### Substrate Space View

```
<SubstrateSpaceView>
├── <Canvas>              (React Three Fiber)
│   ├── <OrbitControls>   (camera)
│   ├── <ExperienceCloud> (instanced spheres)
│   ├── <AttractorFields> (glow effects)
│   ├── <LensViewports>  (cone meshes)
│   ├── <RelationLines>  (line segments)
│   └── <InsightHalos>   (cluster effects)
├── <FilterPanel>          (right side)
│   ├── <TypeFilter>
│   ├── <DomainFilter>
│   ├── <TimeRangeSlider>
│   └── <ImportanceSlider>
├── <NodeDetailPanel>      (on click)
└── <TimeScrubber>         (bottom)
```

---

## 8. Data Flow

### Live Session (Agent Running)

```
1. User starts DevStudio: devstudio "Add auth" --repo ./app
2. PulseHive emits HiveEvents → WebSocket → PulseVision server
3. Server broadcasts to browser clients
4. Agent Flow view updates in real-time (nodes appear, edges draw)
5. PulseDB records experiences → Watch system → PulseVision server
6. Substrate Space view updates (new nodes materialize)
```

### Post-Hoc Analysis (After Agent Run)

```
1. User opens PulseVision and points to a .db file
2. Server reads all experiences, relations, insights from PulseDB
3. Server computes UMAP projection for 3D layout
4. Frontend renders the full substrate space
5. User can explore, filter, click nodes
```

---

## 9. Release History

### v0.1.0 (Released 2026-03-28)

| Feature | Status |
|---------|--------|
| Agent Flow DAG (all 14 event types) | Shipped |
| Node detail panel (click to inspect) | Shipped |
| Real-time WebSocket connection | Shipped |
| Substrate Space 3D view | Shipped |
| Experience nodes with type coloring (9 types) | Shipped |
| Relation lines (color-coded by type) | Shipped |
| Camera controls (orbit, zoom, pan) | Shipped |
| Node click → detail panel | Shipped |
| PCA dimensionality reduction (dynamic dimension) | Shipped (UMAP deferred) |
| REST API for substrate data (10 endpoints) | Shipped |
| Filter panel (type checkboxes, importance slider) | Shipped |
| Attractor gravity wells with glow | Shipped |
| Token usage display (stats bar) | Shipped |
| WebSocket event hub (/ws/ingest, /ws/events, /ws/substrate) | Shipped |
| SqliteSessionStore (event persistence) | Shipped |
| WebSocketExporter client crate | Shipped |
| Dual-mode deployment (embeddable + standalone) | Shipped |
| Dockerfile for containerized deployment | Shipped |
| Flow animations (fade-in, pulse, checkmark, error flash) | Shipped |
| Dark theme design system | Shipped |

### v0.2.0 Roadmap (Planned)

#### Visualization Enhancements

| Feature | Priority | Description |
|---------|----------|-------------|
| Lens viewport cones | High | Cone meshes showing agent perception volumes. Requires new REST endpoint `/api/substrate/lenses` exposing `domain_focus`, `purpose_embedding`, `attention_budget` per agent. |
| UMAP dimensionality reduction | High | Better cluster separation than PCA. Evaluate `umap-js` (client-side) or Rust UMAP implementation. User toggle between PCA/UMAP. |
| Bloom postprocessing | Medium | `@react-three/postprocessing` Bloom effect for attractor glow. Resolve peer dependency conflict with R3F v8. |
| Insight cluster halos | Medium | Golden glow halos around `DerivedInsight` source experience clusters. Requires insight embedding positions. |
| Query trajectory bending | Low | Animated visualization of how attractor gravity wells warp agent query trajectories. Custom GLSL shaders. |

#### Replay & Analysis

| Feature | Priority | Description |
|---------|----------|-------------|
| Time scrubber / session replay | High | Slider to replay recorded sessions from SessionStore. Requires snapshot-based state reconstruction from stored events. |
| Multi-session comparison | Medium | Side-by-side or overlay comparison of two agent runs. Session events already persisted in SQLite. |
| Export trace as JSON | Low | Download button to export a session's events as a JSON file for sharing/analysis. |

#### Infrastructure

| Feature | Priority | Description |
|---------|----------|-------------|
| crates.io publishing | High | Publish `pulsevision`, `pulsevision-cli`, `pulsevision-client` to crates.io. Fix path deps → version refs. |
| PostgresSessionStore | High | `SessionStore` trait implementation using `sqlx` for embedded mode (host app's PostgreSQL). Trait + schema defined, needs implementation. |
| GitHub Actions CI | Medium | Automated CI: `cargo test`, `clippy`, `fmt`, `npm test`, `tsc`, frontend build. Run on push + PRs. |
| Pre-built binaries | Medium | GitHub Actions release workflow producing macOS (arm64/x64) and Linux (x64) binaries on tag push. |
| Cost estimation per model | Low | Model pricing database (OpenAI, Anthropic, GLM) with per-node cost display in detail panels. |

#### Integrations

| Feature | Priority | Description |
|---------|----------|-------------|
| Embedded in ProjectPulse Desktop | High | Tauri integration: embed PulseVision routes at `/vision` in the ProjectPulse Axum backend. Share `Arc<PulseDB>` and event broadcast channel. |
| PulseEval integration | Low | Connect to PulseEval (separate product) for agent output quality scoring. Display quality scores alongside agent nodes. |
| Authentication / multi-tenant | Low | JWT or OAuth for team deployments. Required when PulseVision is exposed beyond localhost. |

#### v0.2.0 Dependencies

| Dependency | Required For |
|------------|-------------|
| PulseHive lens data API | Lens viewport cones |
| PulseDB v0.5+ (if lens data stored) | Lens perception data |
| `@react-three/postprocessing` v3+ | Bloom effect (peer dep fix) |
| Mature Rust UMAP crate OR `umap-js` | UMAP projection |
| GitHub Actions setup | CI, pre-built binaries |

---

## 10. Project Setup

```
pulsevision/
├── Cargo.toml                 ← Axum server
├── src/
│   ├── main.rs               ← Server entry point
│   ├── api/                  ← REST endpoints
│   │   ├── mod.rs
│   │   ├── substrate.rs      ← /api/substrate/*
│   │   └── projections.rs    ← UMAP computation
│   ├── ws/                   ← WebSocket handlers
│   │   ├── mod.rs
│   │   ├── events.rs         ← HiveEvent relay
│   │   └── substrate.rs      ← PulseDB Watch relay
│   └── db/                   ← PulseDB reader
│       └── reader.rs         ← Read-only substrate access
├── frontend/
│   ├── package.json          ← React + R3F + React Flow
│   ├── vite.config.ts
│   ├── src/
│   │   ├── App.tsx           ← Tab layout (Flow | Space)
│   │   ├── components/
│   │   │   ├── flow/         ← Agent Flow view
│   │   │   │   ├── FlowCanvas.tsx
│   │   │   │   ├── AgentNode.tsx
│   │   │   │   ├── ToolCallNode.tsx
│   │   │   │   └── DetailPanel.tsx
│   │   │   ├── space/        ← Substrate Space view
│   │   │   │   ├── SubstrateCanvas.tsx
│   │   │   │   ├── ExperienceCloud.tsx
│   │   │   │   ├── AttractorField.tsx
│   │   │   │   ├── LensViewport.tsx
│   │   │   │   └── RelationLines.tsx
│   │   │   └── shared/       ← Shared components
│   │   │       ├── FilterPanel.tsx
│   │   │       └── StatsBar.tsx
│   │   ├── stores/           ← Zustand state
│   │   │   ├── flowStore.ts
│   │   │   └── spaceStore.ts
│   │   └── hooks/            ← WebSocket + API hooks
│   │       ├── useEventStream.ts
│   │       └── useSubstrate.ts
│   └── public/
└── README.md
```

### Dependencies

**Backend (Cargo.toml):**
```toml
[dependencies]
axum = { version = "0.8", features = ["ws"] }
tokio = { version = "1", features = ["full"] }
pulsehive-db = { version = "0.4", features = ["builtin-embeddings"] }
pulsehive-core = { version = "2.0" }  # For HiveEvent deserialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tower-http = { version = "0.6", features = ["cors"] }
umap-rs = "0.2"  # or linfa-reduction for UMAP
```

**Frontend (package.json):**
```json
{
  "dependencies": {
    "react": "^18",
    "@reactflow/core": "^12",
    "@react-three/fiber": "^8",
    "@react-three/drei": "^9",
    "three": "^0.170",
    "zustand": "^5"
  }
}
```

---

## 11. Usage

### Starting PulseVision

```bash
# Start the PulseVision server
pulsevision --substrate ./path/to/substrate.db --port 3333

# Open in browser
open http://localhost:3333
```

### Connecting PulseHive to PulseVision

```rust
use pulsehive::prelude::*;
use pulsehive::{HiveMind, Task};

// PulseVision provides this WebSocketExporter
// (implements pulsehive_core::export::EventExporter)
use pulsevision_client::WebSocketExporter;

let hive = HiveMind::builder()
    .substrate_path("my_project.db")
    .llm_provider("openai", provider)
    .event_exporter(WebSocketExporter::new("ws://localhost:3333/ws/ingest"))
    .build()?;

// All HiveEvents now stream to PulseVision in real-time as JSON:
// {"type":"agent_started","timestamp_ms":1711...,"agent_id":"019d...","name":"explorer","kind":"llm"}
// {"type":"llm_call_completed","timestamp_ms":1711...,"input_tokens":200,"output_tokens":50,...}
// {"type":"tool_call_started","timestamp_ms":1711...,"tool_name":"search","params":"{\"query\":\"test\"}",...}
```

The `EventExporter` trait and `HiveMindBuilder::event_exporter()` are part of PulseHive v2.0.0 (published). PulseVision only needs to implement the `WebSocketExporter` struct.

---

## 12. Success Criteria

### v0.1.0 Results (Achieved 2026-03-28)

| Criteria | Target | Result |
|----------|--------|--------|
| Agent Flow | Visualize 4-agent pipeline as interactive DAG | Shipped: Explorer→Planner→Coder→Tester with timing + tokens |
| Substrate Space | Render 1000+ experiences in 3D | Shipped: InstancedMesh with type coloring + relation lines |
| Real-time | Events in UI within 100ms | Shipped: WebSocket relay with <100ms latency |
| Click-to-inspect | Full detail on any node | Shipped: 4 detail panel variants + space detail |
| Filtering | Filter by type, domain, time | Shipped: Type checkboxes + importance slider + toggles |
| Performance | 60fps at 5000 nodes | Shipped: InstancedMesh single draw call |
| Zero SDK overhead | PulseHive runs identically with/without exporter | Shipped: Fire-and-forget via tokio::spawn |

### Test Coverage

- 45 tests total (23 Rust + 22 TypeScript)
- Real GLM LLM integration tests
- Full PulseHive → WebSocket → PulseVision E2E pipeline verified
- Zero clippy warnings

---

*PulseVision transforms invisible agent reasoning and knowledge dynamics into an interactive visual experience. It's the developer tool that makes PulseHive's "shared consciousness" model tangible.*
