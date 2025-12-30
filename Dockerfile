# Highly optimized multi-stage build for Allfeat Hub
FROM rust:1.89-slim AS builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Install Rust toolchain and build tools
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk --version 0.21.5
RUN cargo install wasm-bindgen-cli --version 0.2.106
RUN cargo install wasm-opt --locked

WORKDIR /app

# Copy workspace manifests for ALL members
COPY Cargo.toml Cargo.lock ./
COPY apps/hub/backend/Cargo.toml ./apps/hub/backend/
COPY apps/hub/frontend/Cargo.toml ./apps/hub/frontend/
COPY crates/core/Cargo.toml ./crates/core/
COPY crates/ui/Cargo.toml ./crates/ui/
COPY crates/services/Cargo.toml ./crates/services/

# Copy all source code
COPY apps/ ./apps/
COPY crates/ ./crates/
COPY schemas/ ./schemas/

# Build frontend with Trunk
WORKDIR /app/apps/hub/frontend
RUN trunk build --release

# Build backend
WORKDIR /app
RUN cargo build --release --bin allfeat-hub

# ============================================================================
# Runtime stage
# ============================================================================
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy compiled artifacts
COPY --from=builder /app/target/release/allfeat-hub ./allfeat-hub
COPY --from=builder /app/apps/hub/frontend/dist ./apps/hub/frontend/dist
COPY --from=builder /app/apps/hub/frontend/public ./apps/hub/frontend/public

# Create non-root user for security
RUN useradd -r -u 1000 -s /bin/false allfeat && \
    chown -R allfeat:allfeat /app

USER allfeat

EXPOSE 3000

HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:3000/health || exit 1

CMD ["./allfeat-hub", "serve", "--port", "3000"]
