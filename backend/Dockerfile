# MassLoad Backend - Rust Microservice
# Build from workspace root: docker build -f backend/Dockerfile -t massload-backend .
# Run: docker run -p 3000:3000 -e ANTHROPIC_API_KEY=xxx massload-backend

FROM rust:1.83-slim-bookworm AS builder

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY backend ./backend
# Copy frontend Cargo.toml (workspace member, not built)
COPY frontend/Cargo.toml ./frontend/Cargo.toml
RUN mkdir -p frontend/src && echo "fn main() {}" > frontend/src/main.rs

# Build release
WORKDIR /app/backend
RUN cargo build --release

# Runtime image
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Copy binary
COPY --from=builder /app/target/release/massload /app/massload

# Create cache directory
RUN mkdir -p /app/.massload

# Environment
ENV RUST_LOG=info

# Expose port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s \
    CMD curl -f http://localhost:3000/health || exit 1

# Run
CMD ["/app/massload", "serve", "--port", "3000"]
