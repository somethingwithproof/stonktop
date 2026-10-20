# syntax=docker/dockerfile:1

# Multi-stage build for stonktop TUI/CLI
# Produces a small runtime image with the release binary.
# Used for integration and "live" E2E testing in Docker (reproducible packaged app).

# Build stage - use MSRV base for consistency with project
FROM rust:1.89-slim-bookworm@sha256:d7fc7de78bb8c1469933aeecbf801314d30d7d6e9f0578bba4cfa285bfa37fe6 AS builder

WORKDIR /app

# System deps for build (if any native needed; stonktop is pure for most targets)
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests first for better layer caching
COPY Cargo.toml Cargo.lock ./

# Copy only what's needed for the binary (tests/ excluded via .dockerignore)
COPY src ./src

# Build the release binary (uses the release profile with lto/strip)
RUN cargo build --release --locked

# Runtime stage - minimal
FROM debian:bookworm-slim@sha256:7c7b2c966bc9ee8cedfeef67e0e279108992c77681fa595db4a9d65c06ccc587

# Runtime deps: ca-certs for HTTPS to Yahoo, basic shell for debugging if needed
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd -m -u 1000 stonktop

# Copy the stripped binary from builder
COPY --from=builder /app/target/release/stonktop /usr/local/bin/stonktop

# Make executable and switch user
RUN chmod +x /usr/local/bin/stonktop
USER stonktop

# Default to help if no args (TUI would fail in non-tty anyway)
ENTRYPOINT ["stonktop"]
CMD ["--help"]

# Labels for GH releases / OCI
LABEL org.opencontainers.image.title="stonktop"
LABEL org.opencontainers.image.description="A top-like terminal UI for monitoring stock and cryptocurrency prices"
LABEL org.opencontainers.image.source="https://github.com/somethingwithproof/stonktop"
LABEL org.opencontainers.image.licenses="MIT"
