# PulseVision — Multi-stage Docker build
# Builds the Rust backend + React frontend into a single container

# Stage 1: Build frontend
FROM node:20-slim AS frontend-builder
WORKDIR /app/frontend
COPY frontend/package.json frontend/package-lock.json ./
RUN npm ci
COPY frontend/ .
RUN npm run build

# Stage 2: Build Rust backend
FROM rust:1.89-slim AS rust-builder
WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
COPY Cargo.toml Cargo.lock ./
COPY pulsevision/ pulsevision/
COPY pulsevision-cli/ pulsevision-cli/
COPY pulsevision-client/ pulsevision-client/
RUN cargo build --release -p pulsevision-cli

# Stage 3: Runtime
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=rust-builder /app/target/release/pulsevision /usr/local/bin/pulsevision
COPY --from=frontend-builder /app/frontend/dist /app/frontend/dist

WORKDIR /app
EXPOSE 3333

ENTRYPOINT ["pulsevision", "--bind", "0.0.0.0", "--static-dir", "/app/frontend/dist"]
