# Performance Guide

**Product:** PulseVision
**Version:** 0.1.0
**Date:** 2026-03-27

---

## 1. Targets

| Metric | Target |
|--------|--------|
| Event relay latency | <100ms |
| 3D render (5000 nodes) | 60fps |
| Substrate load (1000 exp) | <2s |
| PCA projection (1000 pts) | <500ms |
| REST response | <200ms |
| Bundle size (excl. Three.js) | <500KB gzipped |
| Server memory (1000 exp) | <100MB |

---

## 2. Backend Optimization

### PCA
- Compute once, cache transform matrix, update incrementally
- New experiences projected using cached transform (O(d) per point)
- Debounced at 1s, runs in `spawn_blocking`

### Substrate Data
- All list APIs paginated (max 1000)
- Embeddings stay server-side — only 3D coordinates sent to browser (12B vs 1536B per point)
- Full content fetched on click, not in list

### WebSocket
- `tokio::sync::broadcast` with capacity 256
- Serialize JSON once, send same bytes to all subscribers
- Slow subscribers get `Lagged` error (acceptable for visualization)

### Session Store
- SQLite WAL mode for concurrent reads
- Events batched in 100ms windows
- PostgreSQL uses connection pool (5 connections)

---

## 3. Frontend Optimization

### 3D (React Three Fiber)
- **InstancedMesh** for all spheres (1 draw call for 5000 nodes)
- Per-instance attributes (no material switching)
- **LineSegments** for all relations (1 draw call)
- Sphere segments: 16 for <5K, 8 for >10K nodes

### DAG (React Flow)
- Dagre layout debounced at 100ms
- React Flow built-in viewport culling
- Custom nodes wrapped in `React.memo`

### State (Zustand)
- Granular selectors (not subscribing to entire store)
- Separate eventStore and spaceStore

---

## 4. Scaling Limits

| Dimension | Comfortable | Maximum |
|-----------|-------------|---------|
| 3D experiences | 5,000 | 20,000 |
| Visible relations | 500 | 5,000 |
| DAG nodes | 200 | 1,000 |
| Browser connections | 10 | 50 |
| Events per session | 10,000 | 100,000 |
