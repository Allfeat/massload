# MassLoad Frontend - Leptos/WASM
# ⚠️  IMPORTANT: Build from WORKSPACE ROOT, not from frontend/
#
# Correct usage:
#   docker build -f frontend/Dockerfile -t massload-frontend .
#
# Run:
#   docker run -e BACKEND_URL=https://api.example.com -p 8080:80 massload-frontend

FROM rust:1.83-slim-bookworm AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Install wasm target and trunk
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk --locked

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY frontend ./frontend

# Copy backend Cargo.toml (workspace member, not built)
COPY backend/Cargo.toml ./backend/Cargo.toml
RUN mkdir -p backend/src && echo "fn main() {}" > backend/src/main.rs && echo "" > backend/src/lib.rs

# Build WASM
WORKDIR /app/frontend
RUN trunk build --release

# Runtime image - nginx-unprivileged for K8s compatibility
FROM nginxinc/nginx-unprivileged:alpine

# Copy built files
COPY --from=builder /app/frontend/dist /usr/share/nginx/html

# Copy entrypoint script for runtime config generation
COPY frontend/docker-entrypoint.sh /docker-entrypoint.sh
USER root
RUN chmod +x /docker-entrypoint.sh && \
    chown nginx:nginx /usr/share/nginx/html
USER nginx

# Custom nginx config for SPA routing (port 8080 for unprivileged)
RUN echo 'server { \
    listen 8080; \
    root /usr/share/nginx/html; \
    index index.html; \
    location / { \
        try_files $uri $uri/ /index.html; \
    } \
    location ~* \.(js|css|wasm|png|jpg|jpeg|gif|ico|svg)$ { \
        expires 1y; \
        add_header Cache-Control "public, immutable"; \
    } \
    location = /config.js { \
        expires -1; \
        add_header Cache-Control "no-store, no-cache, must-revalidate"; \
    } \
    gzip on; \
    gzip_types text/plain text/css application/json application/javascript text/xml application/xml application/wasm; \
}' > /etc/nginx/conf.d/default.conf

EXPOSE 8080

# Use custom entrypoint to generate config.js at startup
ENTRYPOINT ["/docker-entrypoint.sh"]
