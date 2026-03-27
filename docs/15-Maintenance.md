# Maintenance Guide

**Product:** PulseVision
**Version:** 0.1.0
**Date:** 2026-03-27

---

## 1. Dependency Management

```bash
# Rust
cargo outdated && cargo audit && cargo update

# Frontend
cd frontend && npm outdated && npm audit && npm update
```

Update frequency: monthly for minor/patch. Major versions reviewed individually.

### Critical Dependencies

| Dependency | Priority | Impact |
|------------|----------|--------|
| axum | High | WebSocket, HTTP |
| tokio | High | Async runtime |
| pulsehive-core | High | HiveEvent format |
| pulsehive-db | High | Substrate API |
| React Flow | Medium | DAG rendering |
| React Three Fiber | Medium | 3D rendering |

---

## 2. PulseHive/PulseDB Compatibility

| Crate | Current | Notes |
|-------|---------|-------|
| pulsehive-core | 2.0.x | HiveEvent changes require PulseVision update |
| pulsehive-db | 0.4.x | New SubstrateProvider methods get default impls |

**When PulseHive releases:** Check CHANGELOG for event changes. New variants = new frontend node type. Changed fields = update detail panel.

**When PulseDB releases:** Check for SubstrateProvider changes. List API changes are backward compatible.

---

## 3. Session Store Cleanup

```bash
# SQLite: delete old sessions
sqlite3 ./pulsevision_sessions.db \
  "DELETE FROM events WHERE session_id IN (SELECT id FROM sessions WHERE created_at < strftime('%s','now','-30 days') * 1000);"
sqlite3 ./pulsevision_sessions.db "VACUUM;"
```

PostgreSQL: host app manages table cleanup.

---

## 4. Release Checklist

- [ ] `cargo test --all-features`
- [ ] `cargo clippy -- -D warnings`
- [ ] `cargo audit`
- [ ] `cd frontend && npm test && npm run build`
- [ ] Test standalone with real PulseDB
- [ ] Test embedded mode
- [ ] Update CHANGELOG.md
- [ ] Tag: `git tag v0.x.y && git push origin v0.x.y`
- [ ] Verify crates.io publish
- [ ] Verify `cargo install pulsevision-cli`

---

## 5. Troubleshooting

| Issue | Fix |
|-------|-----|
| pulsehive-db won't compile | Ensure correct feature flags |
| PulseDB lock error | Check no other PulseVision instance running |
| WS connection refused | Verify port argument |
| PCA all zeros | Check PulseDB has diverse embeddings |
| SQLite locked | PulseVision uses WAL mode; check other clients |
| Frontend build fails | Requires Node.js 20+ |
| 3D view blank | Check WebGL 2.0 support |
