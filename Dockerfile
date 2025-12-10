# Highly optimized multi-stage build for Allfeat MassLoad
FROM rust:1.89-slim AS builder

# Install system dependencies in one layer
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Install Rust toolchain and tools in separate layer for caching
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk --version 0.21.5

WORKDIR /app

# Copy manifests first (best cache layer)
COPY Cargo.toml Cargo.lock ./
COPY backend/Cargo.toml ./backend/
COPY frontend/Cargo.toml ./frontend/

# Copy actual source code
COPY backend/src/ ./backend/src/
COPY backend/schemas/ ./backend/schemas/
COPY frontend/src/ ./frontend/src/
COPY frontend/index.html ./frontend/
COPY frontend/style/ ./frontend/style/
COPY frontend/public/ ./frontend/public/
COPY frontend/package*.json ./frontend/

# Build frontend
WORKDIR /app/frontend
RUN trunk build --release

# Build backend
WORKDIR /app
RUN cargo build --release --bin massload

# Runtime stage
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy artifacts
COPY --from=builder /app/target/release/massload ./massload
COPY --from=builder /app/frontend/dist ./frontend/dist
COPY --from=builder /app/frontend/public ./frontend/public

# Security: non-root user
RUN useradd -r -s /bin/false massload \
    && chown -R massload:massload /app

USER massload

EXPOSE 3000

HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:3000/health || exit 1

CMD ["./massload", "serve", "--port", "3000"]

