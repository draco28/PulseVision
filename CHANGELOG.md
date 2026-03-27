# Changelog

All notable changes to PulseVision will be documented in this file.

## [0.1.0] - 2026-03-28

### Added

#### Agent Flow View
- Real-time DAG visualization of PulseHive agent execution using React Flow v12
- 4 custom node types: AgentNode, LlmCallNode, ToolCallNode, ExperienceNode
- Dagre auto-layout (left-to-right with vertical stacking for parallel agents)
- Click-to-inspect detail panel with type-specific content (timing, tokens, params, results)
- CSS animations: node fade-in, running agent pulse, completion checkmark, error flash
- Stats bar showing live token count, agent count, and event count
- WebSocket auto-reconnect with exponential backoff

#### Substrate Space View
- 3D interactive visualization of PulseDB embedding space using React Three Fiber
- InstancedMesh rendering for 5000+ experience nodes at 60fps
- Spheres colored by ExperienceType (9 distinct colors)
- Spheres sized by importance (0.0-1.0)
- Relation lines between connected experiences (color-coded by RelationType)
- Attractor gravity wells with emissive glow and pulse animation
- OrbitControls: click-drag rotate, scroll zoom, right-click pan
- Hover labels showing experience content preview
- Click-to-inspect detail panel
- Filter panel: experience type checkboxes, importance threshold slider, relation/attractor toggles

#### Backend (Rust Axum)
- Dual-mode architecture: embeddable crate (`pulsevision::router()`) + standalone CLI binary
- PCA projection engine (supports any embedding dimension: 384, 768, 1536, up to 4096)
- Attractor dynamics computation (strength = importance * confidence * (1 + log(applications + 1)))
- 10 REST API endpoints for substrate data (experiences, relations, insights, attractors, projections, collectives, stats, sessions)
- WebSocket event hub: `/ws/ingest` (from PulseHive), `/ws/events` (to browsers), `/ws/substrate` (change notifications)
- SqliteSessionStore with WAL mode for event persistence (stores ALL events including LlmTokenStreamed)
- Auto-session creation on first ingest event, completion on disconnect
- Static file serving for production frontend via tower-http ServeDir

#### Client Library
- `pulsevision-client` crate with `WebSocketExporter` implementing PulseHive's `EventExporter` trait
- Background connection management with auto-reconnect (exponential backoff)
- Channel-based fire-and-forget event forwarding

#### Infrastructure
- Cargo workspace with 3 crates: `pulsevision` (lib), `pulsevision-cli` (bin), `pulsevision-client` (lib)
- Dark theme design system with CSS custom properties
- Dockerfile for containerized deployment
- 45 tests (23 Rust unit/integration + 22 TypeScript Vitest)
- Real GLM LLM integration tests with PulseHive E2E pipeline

### Deferred to v0.2.0
- Lens viewport cones (needs lens data REST endpoint)
- UMAP dimensionality reduction (PCA sufficient for v0.1)
- Time scrubber / session replay
- Cost estimation per LLM model
- Bloom postprocessing for attractor glow
- crates.io publishing
