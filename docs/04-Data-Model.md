# Data Model Document

**Product:** PulseVision
**Version:** 0.1.0
**Date:** 2026-03-27

---

## 1. Overview

PulseVision has two data domains:

1. **PulseDB Data (Read-Only)** — Substrate knowledge graph accessed via pulsehive-db crate APIs. PulseVision never writes to this data.
2. **PulseVision Data (Read-Write)** — Event sessions stored in SessionStore (SQLite or PostgreSQL).

---

## 2. PulseDB Data (Read-Only)

### 2.1 Experience

| Field | Type | Description |
|-------|------|-------------|
| id | ExperienceId (UUID v7) | Unique identifier |
| collective_id | CollectiveId (UUID v7) | Namespace |
| content | String | Immutable knowledge content |
| embedding | Vec<f32> (Nd, typically 384) | Dense vector for similarity search (dimension configured per database) |
| experience_type | ExperienceType (enum, 9 variants) | Generic, Solution, ErrorPattern, Difficulty, SuccessPattern, UserPreference, ArchitecturalDecision, TechInsight, Fact |
| importance | f32 (0.0-1.0) | Mutable significance score |
| confidence | f32 (0.0-1.0) | Mutable reliability score |
| applications | u32 | Reinforcement counter (times applied) |
| domain | Vec<String> | Category tags |
| related_files | Vec<String> | Source file paths |
| source_agent | AgentId (String) | Creating agent |
| source_task | Option<TaskId> | Creating task context |
| timestamp | Timestamp (i64 ms) | Creation time |
| archived | bool | Soft-delete flag |

### 2.2 ExperienceRelation

| Field | Type | Description |
|-------|------|-------------|
| id | RelationId (UUID v7) | Unique identifier |
| source_id | ExperienceId | Source experience |
| target_id | ExperienceId | Target experience |
| relation_type | RelationType | Supports, Contradicts, Elaborates, Supersedes, Implies, RelatedTo |
| strength | f32 (0.0-1.0) | Relationship strength |
| metadata | Option<String> | JSON metadata (max 10KB) |
| created_at | Timestamp | Creation time |

### 2.3 DerivedInsight

| Field | Type | Description |
|-------|------|-------------|
| id | InsightId (UUID v7) | Unique identifier |
| collective_id | CollectiveId | Namespace |
| content | String | Insight text |
| embedding | Vec<f32> | Insight embedding (inline) |
| source_experience_ids | Vec<ExperienceId> | Source experiences |
| insight_type | InsightType | Pattern, Synthesis, Abstraction, Correlation |
| confidence | f32 (0.0-1.0) | Insight confidence |
| domain | Vec<String> | Category tags |
| created_at | Timestamp | Creation time |
| updated_at | Timestamp | Last update time |

### 2.4 Collective

| Field | Type | Description |
|-------|------|-------------|
| id | CollectiveId (UUID v7) | Unique identifier |
| name | String | Display name |

---

## 3. PulseVision Data (Read-Write)

### 3.1 Session

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| id | UUID v7 | PRIMARY KEY | Session identifier |
| created_at | INTEGER | NOT NULL | Unix timestamp (ms) |
| metadata_json | TEXT | NOT NULL | JSON: {substrate_path, description} |
| event_count | INTEGER | NOT NULL, DEFAULT 0 | Number of stored events |
| status | TEXT | NOT NULL, DEFAULT 'recording' | 'recording' or 'completed' |

### 3.2 StoredEvent

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| session_id | UUID | NOT NULL, FK -> sessions.id | Parent session |
| seq | INTEGER | NOT NULL | Sequence number within session |
| event_type | TEXT | NOT NULL | HiveEvent type discriminator |
| event_json | TEXT | NOT NULL, MAX 64KB | Full serialized HiveEvent |
| timestamp_ms | INTEGER | NOT NULL | Event timestamp (from HiveEvent) |
| | | PRIMARY KEY (session_id, seq) | Composite key |

### 3.3 SQL Schema

**SQLite (standalone):**

```sql
CREATE TABLE IF NOT EXISTS sessions (
    id TEXT PRIMARY KEY,
    created_at INTEGER NOT NULL,
    metadata_json TEXT NOT NULL DEFAULT '{}',
    event_count INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'recording'
);

CREATE TABLE IF NOT EXISTS events (
    session_id TEXT NOT NULL REFERENCES sessions(id),
    seq INTEGER NOT NULL,
    event_type TEXT NOT NULL,
    event_json TEXT NOT NULL,
    timestamp_ms INTEGER NOT NULL,
    PRIMARY KEY (session_id, seq)
);

CREATE INDEX IF NOT EXISTS idx_events_timestamp ON events(timestamp_ms);
CREATE INDEX IF NOT EXISTS idx_events_type ON events(event_type);
```

**PostgreSQL (embedded):**

```sql
CREATE TABLE IF NOT EXISTS pulsevision_sessions (
    id UUID PRIMARY KEY,
    created_at BIGINT NOT NULL,
    metadata_json JSONB NOT NULL DEFAULT '{}',
    event_count BIGINT NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'recording'
);

CREATE TABLE IF NOT EXISTS pulsevision_events (
    session_id UUID NOT NULL REFERENCES pulsevision_sessions(id),
    seq BIGINT NOT NULL,
    event_type TEXT NOT NULL,
    event_json JSONB NOT NULL,
    timestamp_ms BIGINT NOT NULL,
    PRIMARY KEY (session_id, seq)
);

CREATE INDEX IF NOT EXISTS idx_pv_events_timestamp ON pulsevision_events(timestamp_ms);
CREATE INDEX IF NOT EXISTS idx_pv_events_type ON pulsevision_events(event_type);
```

---

## 4. Computed/Derived Data (In-Memory)

### 4.1 PCA Projection Cache

| Field | Type | Description |
|-------|------|-------------|
| transform_matrix | Matrix3x384 | PCA transform (top 3 eigenvectors) |
| mean_vector | Vec<f32> (Nd) | Centering vector (same dimension as embeddings) |
| projections | HashMap<ExperienceId, [f32; 3]> | Cached 3D coordinates |
| last_updated | Instant | For cache invalidation |

### 4.2 Attractor Cache

```rust
struct Attractor {
    experience_id: ExperienceId,
    position: [f32; 3],
    strength: f32,               // importance * confidence * (1 + log(applications + 1))
    influence_radius: f32,       // proportional to strength
    warp_factor: f32,            // strength normalized to [0, 1]
    experience_type: ExperienceType,
}
```

---

## 5. Entity Relationship Diagram

```
┌─────────────┐
│ Collective   │ 1
└──────┬──────┘
       │ N
┌──────▼──────┐       ┌────────────────┐
│ Experience   │ M───N │ Relation       │
│ (PulseDB)    │       │ (PulseDB)      │
└─────────────┘       └────────────────┘
       │
       │ derived
┌──────▼──────┐
│ Attractor    │
│ (in-memory)  │
└─────────────┘

┌─────────────┐ 1
│ Session      │─────┐
│ (PulseVision)│     │ N
└─────────────┘  ┌──▼───────────┐
                  │ StoredEvent    │
                  │ (PulseVision)  │
                  └───────────────┘
```
