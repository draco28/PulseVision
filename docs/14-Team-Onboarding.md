# Team Onboarding Guide

**Product:** PulseVision
**Version:** 0.1.0
**Date:** 2026-03-27

---

## 1. Overview

PulseVision is a real-time observability platform for PulseHive multi-agent systems providing:
1. **Agent Flow** — DAG of agent execution (React Flow)
2. **Substrate Space** — 3D visualization of PulseDB embeddings (React Three Fiber)

Two deployment modes: **embedded** (Rust crate in host Axum apps) and **standalone** (CLI binary).

---

## 2. Prerequisites

| Tool | Version |
|------|---------|
| Rust | 1.89+ |
| Node.js | 20+ |
| npm | 10+ |
| Git | 2.40+ |

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable && rustup component add clippy rustfmt
```

---

## 3. Getting Started

```bash
git clone https://github.com/draco28/PulseVision.git
cd PulseVision
cargo build
cd frontend && npm install && cd ..

# Terminal 1: Backend
cargo run -- --substrate ./tests/fixtures/test_substrate.db --port 3333

# Terminal 2: Frontend
cd frontend && npm run dev
```

---

## 4. Crate Map

| Crate | Type | Key Files |
|-------|------|-----------|
| `pulsevision` | Library | `src/lib.rs`, `src/api/`, `src/ws/`, `src/session/` |
| `pulsevision-cli` | Binary | `src/main.rs` |
| `pulsevision-client` | Library | `src/lib.rs` (WebSocketExporter) |

---

## 5. Data Flow

```
PulseHive → /ws/ingest → SessionStore + broadcast → /ws/events → Browser DAG
PulseDB → ChangePoller → PCA project → /ws/substrate → Browser 3D
```

---

## 6. Key Dependencies

| Dependency | Docs |
|------------|------|
| pulsehive-core | `/Users/draco/projects/PulseHive/docs/` |
| pulsehive-db | `/Users/draco/projects/PulseDB/docs/` |
| React Flow | reactflow.dev |
| React Three Fiber | docs.pmnd.rs/r3f |
| Zustand | docs.pmnd.rs/zustand |

---

## 7. Running Tests

```bash
cargo test && cargo clippy -- -D warnings && cargo fmt --check
cd frontend && npm test && npx tsc --noEmit && npx eslint .
```

---

## 8. Conventions

- Rust: `thiserror` errors, `tracing` logging, `spawn_blocking` for CPU work
- TypeScript: strict mode, functional components, Zustand stores
- 3D: InstancedMesh for >100 objects, `useFrame` for animation
- Tests: `#[tokio::test]` for Rust async, `describe/it` for TS
