# Product Backlog

**Product:** PulseVision
**Version:** 0.1.0
**Date:** 2026-03-27

---

## Epic 1: Backend Foundation

### Story 1.1: Cargo Workspace Setup (Sprint 1) — 3 pts
**FR Traces:** FR-056, FR-058, FR-060
- Workspace with 3 crates: pulsevision (lib), pulsevision-cli (bin), pulsevision-client (lib)
- All dependencies declared, `cargo build` succeeds
- Feature flags: sqlite (default), postgres (optional)

### Story 1.2: Substrate Reader (Sprint 1) — 5 pts
**FR Traces:** FR-001, FR-002, FR-003, FR-004, FR-005, FR-009, FR-010
- SubstrateSource enum with Shared and File variants
- Read-only mode for standalone, Arc<PulseDB> for embedded
- ChangePoller integration for standalone change detection

### Story 1.3: REST API Endpoints (Sprint 1) — 5 pts
**FR Traces:** FR-011 through FR-020
- All 10 REST endpoints returning JSON with pagination
- Proper error responses (404, 400, 500)

### Story 1.4: PCA Projection Engine (Sprint 1) — 5 pts
**FR Traces:** FR-006, FR-007
- PCA reduces Nd to 3d via nalgebra (dimension auto-detected from PulseDB)
- Transform matrix cached, recomputed on changes (debounced 1s)

### Story 1.5: Attractor Dynamics (Sprint 2) — 3 pts
**FR Traces:** FR-008, FR-016
- strength = importance * confidence * (1 + log(applications + 1))
- Influence radius proportional to strength

### Story 1.6: WebSocket Event Hub (Sprint 2) — 8 pts
**FR Traces:** FR-021 through FR-026
- /ws/ingest, /ws/events, /ws/substrate
- Broadcast fan-out, 30s ping heartbeat

### Story 1.7: Session Store (Sprint 2) — 8 pts
**FR Traces:** FR-027 through FR-030
- SessionStore trait + SQLite + PostgreSQL implementations
- Stores ALL event types, 100% test coverage

---

## Epic 2: Agent Flow View

### Story 2.1: Frontend Shell (Sprint 3) — 5 pts
**FR Traces:** FR-051, FR-052, FR-053, FR-054, FR-055
- Vite + React 18 + TS, dark theme, tabs, connection status, collective selector

### Story 2.2: DAG Canvas + Node Types (Sprint 3) — 8 pts
**FR Traces:** FR-031, FR-032, FR-033
- React Flow + Dagre, 4 custom node types, real-time updates

### Story 2.3: Detail Panel (Sprint 3) — 5 pts
**FR Traces:** FR-034 through FR-038
- Slide-in panel with type-specific content

### Story 2.4: Flow Animations + Stats (Sprint 4) — 5 pts
**FR Traces:** FR-039, FR-040
- Fade-in, pulse, checkmark, error flash, stats bar

---

## Epic 3: Substrate Space View

### Story 3.1: 3D Experience Cloud (Sprint 4) — 8 pts
**FR Traces:** FR-041, FR-042, FR-043, FR-046, FR-049, FR-050
- InstancedMesh, OrbitControls, hover labels, pop animation, 60fps@5000

### Story 3.2: Relations + Attractors (Sprint 4) — 8 pts
**FR Traces:** FR-044, FR-045
- LineSegments color-coded, attractor glow with influence radius

### Story 3.3: Filter Panel + Detail (Sprint 5) — 5 pts
**FR Traces:** FR-047, FR-048
- Type checkboxes, importance slider, time range, domain, toggles

---

## Epic 4: Polish & Ship

### Story 4.1: pulsevision-client Crate (Sprint 5) — 5 pts
**FR Traces:** FR-060
- WebSocketExporter implementing EventExporter trait

### Story 4.2: Production Build + CLI (Sprint 5) — 5 pts
**FR Traces:** FR-058, FR-059
- Vite build, ServeDir, CLI args, CI pipeline

### Story 4.3: Lens Viewport Cones (Sprint 5) — 5 pts
- Cone meshes showing agent lens perception volumes

---

## Sprint Summary

| Sprint | Points |
|--------|--------|
| Sprint 1 | 18 |
| Sprint 2 | 19 |
| Sprint 3 | 18 |
| Sprint 4 | 21 |
| Sprint 5 | 20 |
| **Total** | **96** |
