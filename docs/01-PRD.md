# Product Requirements Document (PRD)

**Product:** PulseVision
**Version:** 0.1.0
**Date:** 2026-03-27
**Author:** Draco
**Status:** Approved

---

## 1. Product Vision

PulseVision is a **real-time observability and visualization platform** for PulseHive multi-agent systems and PulseDB knowledge substrates. It transforms invisible agent reasoning and knowledge dynamics into interactive visual experiences, enabling developers to debug, understand, and optimize their AI agent pipelines.

### 1.1 Mission Statement

Make every PulseHive agent pipeline transparent — from execution flow to knowledge perception — through real-time visualization that works everywhere PulseHive runs.

### 1.2 Product Goals

| Goal | Metric | Target |
|------|--------|--------|
| G1: Real-time agent observability | Event relay latency | <100ms |
| G2: Knowledge substrate visualization | 3D render performance | 60fps at 5000 nodes |
| G3: Universal deployment | Deployment modes supported | 2 (embedded + standalone) |
| G4: Zero-friction adoption | Time from install to first visualization | <60 seconds |
| G5: Full event capture | Event types visualized | All 14 HiveEvent types |

---

## 2. User Personas

### Persona 1: Platform Developer ("Alex")

- **Role:** Backend developer building hosted products with PulseHive (ProjectPulse, AI Video Editor, Expense Tracker)
- **Environment:** Hosted Axum backend servers
- **Pain Points:** Can't see agent execution flow in production, token costs are invisible, debugging multi-agent failures requires log parsing
- **Goal:** Embed PulseVision into their Axum app with one line of code for always-on observability
- **Success:** Identifies and fixes agent pipeline bottleneck in under 5 minutes using the DAG view

### Persona 2: CLI Agent User ("Jordan")

- **Role:** Developer using DevStudio or custom PulseHive CLI agents
- **Environment:** Local machine, terminal-based workflows
- **Pain Points:** Text logs are unreadable for parallel agent runs, can't understand how agents perceive the substrate
- **Goal:** Run `pulsevision --substrate ./path.db` and see their agent run visualized in the browser
- **Success:** Understands why an agent made a specific decision by inspecting the Substrate Space lens view

### Persona 3: AI Engineer ("Sam")

- **Role:** AI engineer building multi-agent systems with PulseHive SDK
- **Environment:** Development and testing environments
- **Pain Points:** Embedding relationships are invisible, attractor dynamics are opaque, can't see knowledge graph evolution
- **Goal:** Visualize the PulseDB substrate in 3D to understand embedding clusters, attractor gravity wells, and relation connections
- **Success:** Discovers unexpected knowledge cluster that explains agent behavior anomaly

---

## 3. Core Features

### Feature 1: Agent Flow DAG (Priority: HIGH)

Real-time directed acyclic graph showing agent execution flow.

- Visualizes all 14 HiveEvent types as nodes with type-specific shapes and colors
- Auto-layout via Dagre (sequential: left-to-right, parallel: fork/join, loop: circular)
- Click-to-inspect detail panels showing timing, tokens, tool params, results
- Live animations: node fade-in, edge draw, active node pulse, token counter increment
- Stats bar: total tokens, total time, agent count

### Feature 2: Substrate Space 3D View (Priority: HIGH)

Interactive 3D visualization of PulseDB embedding space.

- PCA projection (Nd to 3d, where N is the configured embedding dimension) computed server-side via nalgebra
- Experience nodes as InstancedMesh spheres (size=importance, color=ExperienceType, opacity=decay)
- Relation lines color-coded by RelationType (Supports=green, Contradicts=red, etc.)
- Attractor gravity wells with glow proportional to strength (importance * confidence * reinforcement)
- Filter panel: type checkboxes, importance slider, time range, domain multi-select
- OrbitControls: click-drag rotate, scroll zoom, right-click pan, double-click focus

### Feature 3: Dual-Mode Deployment (Priority: HIGH)

- **Embedded mode:** `pulsevision::router(config)` returns Axum routes for host apps
- **Standalone mode:** `pulsevision --substrate ./path.db --port 3333` CLI binary
- Same frontend, same API surface, different substrate access (shared Arc vs read-only file)

### Feature 4: Event Persistence (Priority: HIGH)

- SessionStore trait with SQLite (standalone) and PostgreSQL (embedded) implementations
- Stores ALL events including LlmTokenStreamed for full replay fidelity
- Session lifecycle: auto-create on first event, complete on disconnect

### Feature 5: WebSocket Event Hub (Priority: HIGH)

- `/ws/ingest`: PulseHive pushes events (standalone mode)
- `/ws/events`: Browsers subscribe to HiveEvent stream
- `/ws/substrate`: Browsers subscribe to PulseDB changes
- tokio::sync::broadcast for server-side fan-out

---

## 4. Success Metrics (KPIs)

| KPI | Target | Measurement |
|-----|--------|-------------|
| Event relay latency | <100ms | Timestamp diff: HiveEvent.timestamp_ms vs browser receipt |
| 3D render FPS | 60fps at 5000 nodes | Browser performance.now() measurements |
| Substrate initial load | <2s for 1000 experiences | REST response time |
| PCA projection time | <500ms for 1000 points | Server tracing spans |
| Adoption | Embedded in 3+ PulseHive products | Count of consumers |
| Time to first visualization | <60s from install | User testing |

---

## 5. Out of Scope (v1)

| Feature | Reason | Planned For |
|---------|--------|-------------|
| UMAP dimensionality reduction | Rust ecosystem immature; PCA sufficient | v2 |
| Time scrubber / session replay UI | Complex state management | v2 |
| Cost estimation per model | Needs pricing database | v2 |
| Multi-session comparison | Requires session storage (v1 enables this) | v2 |
| Query trajectory bending animation | Complex shader work | v2 |
| Insight cluster halos | Visual polish | v2 |
| Export trace as JSON | Low priority | v2 |
| Authentication / multi-tenant | Not needed for dev tool | v2 |
| Mobile support | 3D visualization requires large screens | v2+ |
| Tauri/ProjectPulse Desktop embedding | Separate integration effort | v2 |
| PulseEval integration | Separate product | v2+ |

---

## 6. Dependencies

| Dependency | Version | Status | Risk |
|------------|---------|--------|------|
| pulsehive-core | v2.0.0 | Published on crates.io | None |
| pulsehive-db | v0.4.0 | Published on crates.io | None |
| React Flow | v12+ | Stable, MIT license | None |
| React Three Fiber | v8+ | Stable, MIT license | None |
| nalgebra | v0.33 | Stable, mature | None |
| EventExporter wiring | v2.0.0 | Implemented and published | None |

All dependencies are either internal (own ecosystem) or mature open-source. Zero third-party API risk.

---

## 7. Constraints

- **Read-only:** PulseVision never writes to PulseDB (Config::read_only())
- **Secretless:** Zero API keys, zero credentials, zero environment variables
- **Auth-agnostic:** No built-in auth; embedded mode inherits host's auth
- **Dark theme only:** Optimized for 3D visualization contrast
- **Desktop-first:** Chrome and Firefox, WebGL 2.0 required
- **PCA for v1:** Top 3 principal components from Nd embeddings (dimension read from PulseDB at runtime — 384 default, supports 768, 1536, up to 4096)
