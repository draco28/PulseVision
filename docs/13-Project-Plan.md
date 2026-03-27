# Project Plan

**Product:** PulseVision
**Version:** 0.1.0
**Timeline:** 6 weeks (5 sprints)
**Start Date:** 2026-03-31
**Target Launch:** 2026-05-09

---

## 1. Phase Overview

| Phase | Duration | Sprints | Goal |
|-------|----------|---------|------|
| Phase 1: Backend Foundation | 2 weeks | Sprint 1-2 | Axum server, REST APIs, WebSocket hub, PCA, event persistence |
| Phase 2: Frontend Views | 2 weeks | Sprint 3-4 | Agent Flow DAG + Substrate Space 3D |
| Phase 3: Polish & Ship | 1 week | Sprint 5 | Filters, client crate, production build, CI/CD |

---

## 2. Sprint Breakdown

### Sprint 1: Core Backend (Week 1)

| Day | Deliverable |
|-----|------------|
| Mon-Tue | Cargo workspace (3 crates), dependencies, error types, config |
| Wed | SubstrateSource enum, PulseDB reader, ChangePoller wrapper |
| Thu | REST endpoints (10 total, paginated) |
| Fri | PCA projection engine with nalgebra |

### Sprint 2: Event Infrastructure (Week 2)

| Day | Deliverable |
|-----|------------|
| Mon | Attractor dynamics computation |
| Tue-Wed | WebSocket hub (/ws/ingest, /ws/events, /ws/substrate) |
| Thu-Fri | SessionStore trait + SQLite + PostgreSQL implementations |

### Sprint 3: Agent Flow View (Week 3)

| Day | Deliverable |
|-----|------------|
| Mon | Vite + React scaffold, dark theme, tabs, WS hook |
| Tue-Wed | Zustand store, DAG construction, React Flow + Dagre |
| Thu | 4 custom node types |
| Fri | Detail panel (slide-in) |

### Sprint 4: Substrate Space View (Week 4)

| Day | Deliverable |
|-----|------------|
| Mon | Flow animations + stats bar |
| Tue-Wed | R3F canvas, InstancedMesh experience cloud, hover labels |
| Thu | Relation LineSegments, pop animation |
| Fri | Attractor glow effects |

### Sprint 5: Polish & Release (Week 5)

| Day | Deliverable |
|-----|------------|
| Mon | Filter panel |
| Tue | Substrate detail panel |
| Wed | pulsevision-client crate (WebSocketExporter) |
| Thu | Production build, CLI polish |
| Fri | Lens cones, CI, tag v0.1.0 |

---

## 3. Buffer Week (Week 6: 2026-05-05 to 2026-05-09)

Reserved for bug fixes, performance optimization, documentation, integration testing with DevStudio.

---

## 4. Critical Path

```
Workspace → Substrate Reader → REST APIs → PCA → Frontend Shell → DAG Canvas → 3D Cloud
```

WebSocket hub and SessionStore are on a parallel track converging at Sprint 3.

---

## 5. Definition of Done (v0.1.0)

- [ ] Agent Flow: 4-agent pipeline as interactive DAG
- [ ] Substrate Space: 1000+ experiences in 3D with type coloring and relations
- [ ] Real-time: Events in UI within 100ms
- [ ] Click-to-inspect: Full node detail
- [ ] Filtering: By type, domain, time range
- [ ] Performance: 60fps at 5000 nodes
- [ ] Dual-mode: Embedded and standalone work
- [ ] Persistence: All events in SessionStore
- [ ] CI: All checks pass
- [ ] Published: crates.io v0.1.0
