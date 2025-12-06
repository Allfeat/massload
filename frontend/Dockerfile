# MassLoad Frontend - Leptos/WASM
# Build: docker build --build-arg BACKEND_URL=https://api.massload.io -t massload-frontend .
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

# Copy project files
COPY Cargo.toml Cargo.lock ./
COPY Trunk.toml ./
COPY index.html ./
COPY src ./src
COPY style ./style
COPY package.json ./

# Build-time configuration (can be overridden with --build-arg)
ARG BACKEND_URL=http://localhost:3000
ARG BLOCKCHAIN_RPC=wss://node-dev.allfeat.io

ENV BACKEND_URL=${BACKEND_URL}
ENV BLOCKCHAIN_RPC=${BLOCKCHAIN_RPC}

# Build WASM
RUN trunk build --release

# Runtime image - nginx to serve static files
FROM nginx:alpine

# Copy built files
COPY --from=builder /app/dist /usr/share/nginx/html

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

