# 🐳 Docker & Kubernetes Integration

## Docker Image

The Dockerfile follows the same pattern as `faucet`:

```dockerfile
# Multi-stage build:
# 1. Builder: compile frontend (Trunk) + backend (Cargo)
# 2. Runtime: Debian slim with binary + frontend/dist/
```

### Local Build

```bash
docker build -t ghcr.io/allfeat/massload:local .
```

### Local Testing

```bash
# With docker-compose
docker-compose up

# Or directly
docker run -p 3000:3000 --env-file .env ghcr.io/allfeat/massload:local
```

## Environment Variables

See `.env.example` for the complete list.

**Required:**
- `ANTHROPIC_API_KEY` : Claude API key

**Optional:**
- `RUST_LOG=info` : Log level
- `PORT=3000` : Server port

## Architecture Notes

**Unified Server:**
- The backend (Axum) serves the frontend (Leptos WASM) as static files
- Frontend uses **relative paths** for API calls (`/api/upload`, `/api/logs`)
- No `BACKEND_URL` configuration needed (unlike older microservice architectures)
- Same pattern as `faucet` project

## For infra-kube Integration

### Expected Structure

```
infra-kube/
└── kubernetes/
    └── apps/
        └── massload/
            ├── base/
            │   ├── deployment.yaml
            │   ├── service.yaml
            │   ├── configmap.yaml
            │   ├── secret.yaml (sealed)
            │   └── kustomization.yaml
            └── overlays/
                ├── dev/
                │   └── kustomization.yaml
                └── prod/
                    └── kustomization.yaml
```

### Deployment Configuration

```yaml
# deployment.yaml
spec:
  template:
    spec:
      containers:
      - name: massload
        image: IMAGE_PLACEHOLDER  # Kustomize edit set image
        ports:
        - containerPort: 3000
        env:
        - name: ANTHROPIC_API_KEY
          valueFrom:
            secretKeyRef:
              name: massload-secret
              key: ANTHROPIC_API_KEY
        - name: RUST_LOG
          value: "info"
```

### ConfigMap for Frontend (Optional)

The frontend uses **relative paths** for API calls (no `BACKEND_URL` needed).  
Only external services like blockchain RPC need configuration:

```yaml
# configmap.yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: massload-config
data:
  config.js: |
    window.MASSLOAD_CONFIG = {
      BLOCKCHAIN_RPC: "wss://node-dev.allfeat.io"
    };
```

**Note:** If you don't mount `config.js`, the frontend will use default values (devnet RPC).

Mount as volume in deployment:
```yaml
volumeMounts:
- name: config-js
  mountPath: /app/frontend/dist/config.js
  subPath: config.js
volumes:
- name: config-js
  configMap:
    name: massload-config
```

### Health Check

```yaml
livenessProbe:
  httpGet:
    path: /health
    port: 3000
  initialDelaySeconds: 10
readinessProbe:
  httpGet:
    path: /health
    port: 3000
  initialDelaySeconds: 5
```

### Service

```yaml
# service.yaml
apiVersion: v1
kind: Service
metadata:
  name: massload
spec:
  type: ClusterIP
  ports:
  - port: 80
    targetPort: 3000
  selector:
    app: massload
```

### Ingress

```yaml
# In base/ or overlay/
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: massload
  annotations:
    cert-manager.io/cluster-issuer: "letsencrypt-prod"
spec:
  ingressClassName: nginx
  tls:
  - hosts:
    - massload-dev.allfeat.io  # or massload.allfeat.io in prod
    secretName: massload-tls
  rules:
  - host: massload-dev.allfeat.io
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: massload
            port:
              number: 80
```

## GitHub Actions Workflows

Workflows can follow the same pattern as `faucet`:

1. **deploy-dev.yml** : Push to `develop` → Build image → Update infra-kube/dev
2. **deploy-prod.yml** : Push to `main` → Build image → Update infra-kube/prod

See `faucet/.github/workflows/` for reference.

## Notes

- **Port**: 3000 (same as faucet)
- **Architecture**: Unified server (backend serves frontend CSR)
- **User**: `massload` (non-root, like faucet)
- **Health**: `/health` endpoint
- **Stateless**: No persistent volumes required
