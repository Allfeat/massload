# MassLoad Frontend - Leptos/WASM
# Build from workspace root: docker build -f frontend/Dockerfile --build-arg BACKEND_URL=https://api.example.com -t massload-frontend .
# Run: docker run -p 8080:80 massload-frontend

FROM rust:1.83-slim-bookworm AS builder

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y \
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
RUN mkdir -p backend/src && echo "fn main() {}" > backend/src/main.rs && echo "pub fn lib() {}" > backend/src/lib.rs

# Build-time configuration (can be overridden with --build-arg)
ARG BACKEND_URL=http://localhost:3000
ARG BLOCKCHAIN_RPC=wss://node-dev.allfeat.io

ENV BACKEND_URL=${BACKEND_URL}
ENV BLOCKCHAIN_RPC=${BLOCKCHAIN_RPC}

# Build WASM
WORKDIR /app/frontend
RUN trunk build --release

# Runtime image - nginx to serve static files
FROM nginx:alpine

# Copy built files
COPY --from=builder /app/frontend/dist /usr/share/nginx/html

# Custom nginx config for SPA routing
RUN echo 'server { \
    listen 80; \
    root /usr/share/nginx/html; \
    index index.html; \
    location / { \
        try_files $uri $uri/ /index.html; \
    } \
    location ~* \.(js|css|wasm|png|jpg|jpeg|gif|ico|svg)$ { \
        expires 1y; \
        add_header Cache-Control "public, immutable"; \
    } \
    gzip on; \
    gzip_types text/plain text/css application/json application/javascript text/xml application/xml application/wasm; \
}' > /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
