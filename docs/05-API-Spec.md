# API Specification

**Product:** PulseVision
**Version:** 0.1.0
**Date:** 2026-03-27
**Base URL:** `http://localhost:3333` (standalone) or `/vision` (embedded)

---

## 1. REST API

### 1.1 GET /api/substrate/experiences

List experiences with pagination.

| Param | Type | Default | Description |
|-------|------|---------|-------------|
| collective_id | UUID | required | Collective to query |
| limit | integer | 100 | Max results (1-1000) |
| offset | integer | 0 | Skip N results |

**Response 200:**
```json
{
  "experiences": [{ "id": "...", "content_preview": "...", "experience_type": "Generic", "importance": 0.7, "confidence": 0.85, "applications": 3, "domain": ["rust"], "timestamp_ms": 1711500000000 }],
  "total": 250, "limit": 100, "offset": 0
}
```

### 1.2 GET /api/substrate/experiences/:id

Single experience with full content. Returns 404 if not found.

### 1.3 GET /api/substrate/embeddings

PCA-projected 3D coordinates. Response includes `projections: [{id, x, y, z}]`, `method: "pca"`, `variance_explained`, `total_points`.

### 1.4 GET /api/substrate/relations

All relations for a collective (paginated). Response: `relations: [{id, source_id, target_id, relation_type, strength}]`.

### 1.5 GET /api/substrate/insights

All insights for a collective (paginated). Response: `insights: [{id, content, insight_type, confidence, source_count, domain}]`.

### 1.6 GET /api/substrate/attractors

Computed attractor gravity wells. Accepts `threshold` query param (default 0.5). Response: `attractors: [{experience_id, position: {x,y,z}, strength, influence_radius, warp_factor, experience_type}]`.

### 1.7 GET /api/substrate/collectives

List available collectives. Response: `collectives: [{id, name}]`.

### 1.8 GET /api/substrate/stats

Summary statistics including experience_count, relation_count, type_distribution, avg_importance, embedding_dimension.

### 1.9 GET /api/sessions

List recording sessions. Response: `sessions: [{id, created_at, event_count, status, metadata}]`.

### 1.10 GET /api/sessions/:id/events

Replay events from a session (paginated). Response: `events: [{seq, event_type, event_json, timestamp_ms}]`.

---

## 2. WebSocket API

### 2.1 WS /ws/ingest (Standalone Only)

Accepts HiveEvents from PulseHive. One JSON HiveEvent per frame. Server persists to SessionStore and broadcasts to /ws/events subscribers. Malformed JSON silently dropped.

### 2.2 WS /ws/events

Broadcasts HiveEvents to browser clients. Same JSON format as ingest. 30s ping keepalive.

### 2.3 WS /ws/substrate

Broadcasts substrate changes. Message format: `{change_type, experience_id, collective_id, position: {x,y,z}, experience_type, importance}`.

---

## 3. Error Format

```json
{ "error": "Human-readable error message" }
```

| Status | Meaning |
|--------|--------|
| 200 | Success |
| 400 | Invalid request |
| 404 | Not found |
| 500 | Internal error |

---

## 4. Authentication

None for v1. Standalone runs on localhost. Embedded protected by host's auth middleware.

## 5. CORS

Standalone: `AllowOrigin::any()`. Embedded: inherits host configuration.
