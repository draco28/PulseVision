# Observability Guide

**Product:** PulseVision
**Version:** 0.1.0
**Date:** 2026-03-27

---

## 1. Logging

Uses `tracing` crate with structured fields.

| Level | Usage |
|-------|-------|
| ERROR | PulseDB open failure, SessionStore write failure |
| WARN | Malformed HiveEvent JSON, broadcast channel lagged |
| INFO | Server startup, WebSocket connect/disconnect, session lifecycle |
| DEBUG | Event received, PCA recomputation, ChangePoller tick |
| TRACE | Raw WebSocket frames, individual broadcasts |

```bash
pulsevision --substrate ./path.db --log-level info
RUST_LOG=pulsevision=debug pulsevision --substrate ./path.db
```

---

## 2. Metrics (via tracing spans)

| Metric | Span | Fields |
|--------|------|--------|
| PCA time | `pca_compute` | point_count, duration_ms |
| REST latency | `http_request` | method, path, status, duration_ms |
| WS throughput | `ws_broadcast` | event_type, subscriber_count |
| Poll latency | `change_poll` | changes_found, duration_ms |
| Store write | `session_store_write` | event_type, duration_ms |

---

## 3. Health Check

```
GET /api/health → { status, substrate, session_store, active_ws_connections, uptime_seconds }
```

---

## 4. Debugging Guide

| Symptom | Diagnosis | Fix |
|---------|-----------|-----|
| No events in Flow | Check /ws/ingest | Verify WebSocketExporter configured |
| 3D view empty | Check /api/substrate/experiences | Verify collective_id, check substrate has data |
| Low FPS | Check node count | Filter to reduce visible nodes |
| WS disconnects | Check server logs | Verify network, check max message size |
| PCA returns NaN | Check embeddings | Verify valid floats, not all zeros |
