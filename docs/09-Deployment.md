# Deployment Guide

**Product:** PulseVision
**Version:** 0.1.0
**Date:** 2026-03-27

---

## 1. Distribution Channels

| Channel | Command |
|---------|---------|
| crates.io (library) | `cargo add pulsevision` |
| crates.io (binary) | `cargo install pulsevision-cli` |
| crates.io (client) | `cargo add pulsevision-client` |
| GitHub Releases | Download pre-built binary |
| Docker | `docker run pulsevision` |

---

## 2. Standalone Deployment

### From crates.io

```bash
cargo install pulsevision-cli
pulsevision --substrate ./my_project.db --port 3333
```

### From GitHub Releases

```bash
# macOS (Apple Silicon)
curl -L https://github.com/draco28/PulseVision/releases/latest/download/pulsevision-aarch64-apple-darwin.tar.gz | tar xz
./pulsevision --substrate ./my_project.db --port 3333
```

### Docker

```bash
docker run -p 3333:3333 -v $(pwd)/data:/data pulsevision --substrate /data/substrate.db
```

### CLI Arguments

| Argument | Default | Description |
|----------|---------|-------------|
| `--substrate` | required | Path to PulseDB file |
| `--port` | 3333 | HTTP/WebSocket port |
| `--bind` | 127.0.0.1 | Bind address |
| `--session-db` | `./pulsevision_sessions.db` | SQLite path |
| `--log-level` | info | Logging level |

---

## 3. Embedded Deployment

```toml
[dependencies]
pulsevision = { version = "0.1", features = ["postgres"] }
```

```rust
let vision = pulsevision::router(PulseVisionConfig {
    substrate: SubstrateSource::Shared(pulsedb_handle),
    event_source: EventSource::Channel(event_rx),
    session_store: Arc::new(PostgresSessionStore::new(pg_pool)),
    collective_id: None,
});

let app = Router::new()
    .nest("/vision", vision)
    .merge(your_routes);
```

Run PostgreSQL migration: `pulsevision::session::PostgresSessionStore::run_migrations(&pg_pool).await?;`

---

## 4. Release Process

1. Update version: `cargo set-version 0.x.y`
2. Tag: `git tag v0.x.y && git push origin v0.x.y`
3. CI handles: test, build binaries, publish crates.io, GitHub Release, Docker image
4. Rollback: `cargo yank --version 0.x.y pulsevision`
