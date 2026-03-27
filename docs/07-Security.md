# Security Plan

**Product:** PulseVision
**Version:** 0.1.0
**Date:** 2026-03-27

---

## 1. Threat Model

PulseVision has an unusually small attack surface:

| Property | Security Impact |
|----------|----------------|
| **Read-only** | No data mutations. PulseDB opened with `Config::read_only()`. |
| **Secretless** | Zero API keys, credentials, or tokens. |
| **Auth-agnostic** | No authentication system to compromise. |
| **Local/embedded** | Standalone runs on localhost. Embedded behind host's perimeter. |
| **No user data** | Zero PII collection. |

### Threat Assessment

| Threat | Risk | Applicable? |
|--------|------|-------------|
| SQL Injection | N/A | PulseDB uses redb (key-value). SessionStore uses parameterized queries. |
| XSS | Low | React auto-escapes. |
| CSRF | N/A | No state-mutating endpoints. |
| Auth bypass | N/A | No authentication. |
| DoS (WebSocket) | Medium | Max message size 1MB. Standalone is localhost-only. |
| Supply chain | Low | Dependencies are mature (axum, tokio, serde). |

---

## 2. Security Controls

### Input Validation

| Input | Validation |
|-------|------------|
| REST query params | UUID format, pagination bounds (limit 1-1000, offset >= 0) |
| REST path params | UUID format, 400 on invalid |
| WebSocket messages | serde_json strict deserialization, malformed JSON rejected |
| WebSocket frames | Max 1MB per frame |

### Network

- **Standalone:** Binds to `127.0.0.1` by default
- **Embedded:** Inherits host's network configuration
- CORS via tower-http

### WebSocket

- Max message size: 1MB
- Ping/pong keepalive: 30s
- Stale connection cleanup on 3 missed pongs

### Data Protection

- PulseDB: `Config::read_only()` enforced
- SessionStore: parameterized queries only
- No encryption needed on localhost; host's TLS for embedded

---

## 3. Dependency Security

- `cargo audit` in CI (RustSec advisories)
- `npm audit` for frontend
- Dependabot enabled on GitHub
- All critical deps (axum, tokio, serde) maintained by well-known teams

---

## 4. Future Security (v2)

| Feature | When Needed |
|---------|-------------|
| JWT/OAuth authentication | Multi-tenant team deployments |
| TLS for standalone | When `--bind 0.0.0.0` is used |
| Rate limiting | Shared server deployments |
| Audit logging | Enterprise compliance |
