# Testing Strategy

**Product:** PulseVision
**Version:** 0.1.0
**Date:** 2026-03-27

---

## 1. Testing Pyramid

```
          /\
         /  \        Manual QA (3D rendering, WebSocket reconnection)
        /    \
       /------\      Integration Tests (REST endpoints, WS pipeline)
      /        \
     /----------\    Unit Tests (PCA, session store, attractor dynamics,
    /            \   DAG construction, event parsing, store reducers)
   /______________\
```

---

## 2. Backend Testing (cargo test)

### Unit Tests

| Module | Coverage Target |
|--------|----------------|
| `api/projections.rs` (PCA) | 90% |
| `api/attractors.rs` | 90% |
| `session/sqlite.rs` | 100% |
| `session/postgres.rs` | 100% |
| `db/reader.rs` | 80% |
| `error.rs` | 100% |

### Integration Tests

| Test | Description |
|------|-------------|
| REST endpoints | Start Axum with test PulseDB, hit all endpoints |
| WebSocket pipeline | Ingest event, verify broadcast + persistence |
| Substrate changes | Write to PulseDB, verify /ws/substrate notification |
| Embedded mode | Test with `SubstrateSource::Shared` |

### Fixtures

- `tests/fixtures/test_substrate.db` — 100 experiences, 20 relations, 5 insights
- `tests/fixtures/test_events.json` — 50 HiveEvents (4-agent pipeline)

---

## 3. Frontend Testing (Vitest)

### Unit Tests

| Module | Coverage Target |
|--------|----------------|
| `stores/eventStore.ts` (DAG construction) | 80% |
| `stores/spaceStore.ts` (filters) | 70% |
| `stores/uiStore.ts` | 70% |
| Event type parsing | 100% |

### Component Tests (React Testing Library)

- DetailPanel: correct content per node type
- StatsBar: displays correct values
- FilterPanel: toggles propagate to store
- Toolbar: tab switching, connection indicator

**NOT tested:** React Flow canvas, React Three Fiber canvas (visual libraries)

---

## 4. Manual QA Checklist

### Agent Flow
- [ ] DAG builds in real-time from live events
- [ ] All 4 node types render correctly
- [ ] Click → detail panel shows correct info
- [ ] Sequential/parallel/loop layouts work
- [ ] Stats bar updates live

### Substrate Space
- [ ] 1000+ experiences at 60fps
- [ ] Sphere colors match ExperienceType
- [ ] Relation lines visible and color-coded
- [ ] Attractor glow effects
- [ ] Camera controls (orbit, zoom, pan, focus)
- [ ] Filter panel works

### WebSocket
- [ ] Auto-reconnect after server restart
- [ ] Multiple browser tabs receive events

---

## 5. Performance Testing

| Test | Target | Method |
|------|--------|--------|
| 3D FPS (5000 nodes) | 60fps | Chrome DevTools |
| PCA (1000 points) | <500ms | tracing spans |
| REST responses | <200ms | curl timing |
| WS latency | <100ms | timestamp diff |
| Bundle size | <500KB gzipped | Vite build output |

---

## 6. CI Pipeline

**Rust:** `cargo fmt --check` → `cargo clippy -- -D warnings` → `cargo test --all-features` → `cargo audit`

**Frontend:** `npm ci` → `tsc --noEmit` → `eslint .` → `npm test` → `npm run build`

Both triggered on push to `main` and PRs. Must pass before merge.
