# MassLoad - Deployment Guide

## Architecture

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│    Frontend     │────▶│     Backend     │────▶│  Claude API     │
│  (Leptos/WASM)  │     │  (Axum/Rust)    │     │  (Anthropic)    │
│    nginx:80     │     │     :3000       │     │                 │
└─────────────────┘     └─────────────────┘     └─────────────────┘
         │                      │
         ▼                      ▼
┌─────────────────┐     ┌─────────────────┐
│  Allfeat Chain  │     │   CSV/Schema    │
│   (WebSocket)   │     │   Processing    │
└─────────────────┘     └─────────────────┘
```

## Docker

### Build Images

```bash
# Backend
docker build -f backend/Dockerfile -t massload-backend .

# Frontend (single image for all environments)
docker build -f frontend/Dockerfile -t massload-frontend .
```

### Run Locally

```bash
# With docker-compose (recommended for local dev)
export ANTHROPIC_API_KEY="sk-ant-..."
docker-compose up --build

# Or manually
docker run -d -p 3000:3000 -e ANTHROPIC_API_KEY="sk-ant-..." massload-backend

# Frontend with custom backend URL (runtime config)
docker run -d -p 8080:80 \
  -e BACKEND_URL=http://localhost:3000 \
  -e BLOCKCHAIN_RPC=wss://node-dev.allfeat.io \
  massload-frontend
```

### Push to Registry

```bash
# Tag and push to GitHub Container Registry
docker tag massload-backend ghcr.io/allfeat/massload-backend:latest
docker tag massload-frontend ghcr.io/allfeat/massload-frontend:latest

docker push ghcr.io/allfeat/massload-backend:latest
docker push ghcr.io/allfeat/massload-frontend:latest
```

## Kubernetes

### Prerequisites

- `kubectl` configured with cluster access
- Ingress controller (nginx-ingress recommended)
- Optional: cert-manager for TLS

### Quick Deploy

```bash
# 1. Create the secret with your API key (don't commit this!)
kubectl create namespace massload
kubectl create secret generic massload-secrets \
  --from-literal=ANTHROPIC_API_KEY=sk-ant-xxx \
  -n massload

# 2. Apply all manifests
kubectl apply -k deploy/k8s/

# 3. Check status
kubectl get pods -n massload
kubectl get ingress -n massload
```

### Manual Deploy (without Kustomize)

```bash
kubectl apply -f deploy/k8s/namespace.yaml
kubectl apply -f deploy/k8s/configmap.yaml
# Create secret manually (see above)
kubectl apply -f deploy/k8s/backend-deployment.yaml
kubectl apply -f deploy/k8s/backend-service.yaml
kubectl apply -f deploy/k8s/frontend-deployment.yaml
kubectl apply -f deploy/k8s/frontend-service.yaml
kubectl apply -f deploy/k8s/ingress.yaml
```

### Configuration

| Variable | Location | Description |
|----------|----------|-------------|
| `ANTHROPIC_API_KEY` | Secret | Claude API key (required) |
| `RUST_LOG` | ConfigMap | Log level (info, debug, trace) |
| `BACKEND_URL` | ConfigMap/Env | Backend API URL (runtime config) |
| `BLOCKCHAIN_RPC` | ConfigMap/Env | Allfeat node WebSocket URL (runtime config) |

### Update Deployment

```bash
# After pushing new images
kubectl rollout restart deployment/massload-backend -n massload
kubectl rollout restart deployment/massload-frontend -n massload

# Check rollout status
kubectl rollout status deployment/massload-backend -n massload
```

### Troubleshooting

```bash
# View logs
kubectl logs -f deployment/massload-backend -n massload
kubectl logs -f deployment/massload-frontend -n massload

# Describe pod for events
kubectl describe pod -l app.kubernetes.io/component=backend -n massload

# Port-forward for local testing
kubectl port-forward svc/massload-backend 3000:3000 -n massload
kubectl port-forward svc/massload-frontend 8080:80 -n massload
```

## Environment Variables

### Backend

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `ANTHROPIC_API_KEY` | Yes | - | Claude API key |
| `RUST_LOG` | No | `info` | Log verbosity |

### Frontend (Runtime)

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `BACKEND_URL` | No | `http://localhost:3000` | Backend API endpoint |
| `BLOCKCHAIN_RPC` | No | `wss://node-dev.allfeat.io` | Allfeat node WS |

✅ Frontend variables are **runtime** - configured via environment variables when starting the container. The same Docker image works for all environments (dev, staging, prod).

## CI/CD (GitHub Actions)

Example workflow for automatic builds:

```yaml
# .github/workflows/docker.yml
name: Build & Push Docker

on:
  push:
    branches: [master]
    tags: ['v*']

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Login to GHCR
        uses: docker/login-action@v3
        with:
          registry: ghcr.io
          username: ${{ github.actor }}
          password: ${{ secrets.GITHUB_TOKEN }}
      
      - name: Build & Push Backend
        uses: docker/build-push-action@v5
        with:
          context: .
          file: backend/Dockerfile
          push: true
          tags: ghcr.io/allfeat/massload-backend:${{ github.sha }}
      
      - name: Build & Push Frontend
        uses: docker/build-push-action@v5
        with:
          context: .
          file: frontend/Dockerfile
          push: true
          tags: ghcr.io/allfeat/massload-frontend:${{ github.sha }}
```

