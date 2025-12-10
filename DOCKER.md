# 🐳 Docker - Notes pour Lois

## Image Docker

Le Dockerfile suit le même pattern que `faucet` :

```dockerfile
# Multi-stage build:
# 1. Builder: compile frontend (Trunk) + backend (Cargo)
# 2. Runtime: Debian slim avec binaire + frontend/dist/
```

### Build local

```bash
docker build -t ghcr.io/allfeat/massload:local .
```

### Test local

```bash
# Avec docker-compose
docker-compose up

# Ou directement
docker run -p 3000:3000 --env-file .env ghcr.io/allfeat/massload:local
```

## Variables d'environnement

Voir `.env.example` pour la liste complète.

**Obligatoire :**
- `ANTHROPIC_API_KEY` : Clé API Claude

**Optionnel :**
- `RUST_LOG=info` : Niveau de logs
- `PORT=3000` : Port du serveur

## Pour infra-kube

### Structure attendue

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

### Deployment hints

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

### ConfigMap pour frontend

Le frontend attend un fichier `/config.js` optionnel :

```yaml
# configmap.yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: massload-config
data:
  config.js: |
    window.MASSLOAD_CONFIG = {
      BACKEND_URL: "",  // Empty = même origine (serveur unifié)
      BLOCKCHAIN_RPC: "wss://node-dev.allfeat.io"
    };
```

Monter en volume dans le deployment :
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

### Health check

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
# Dans base/ ou overlay/
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
    - massload-dev.allfeat.io  # ou massload.allfeat.io en prod
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

## Workflows GitHub Actions

Les workflows peuvent suivre le même pattern que `faucet` :

1. **deploy-dev.yml** : Push sur `develop` → Build image → Update infra-kube/dev
2. **deploy-prod.yml** : Push sur `main` → Build image → Update infra-kube/prod

Voir `faucet/.github/workflows/` pour référence.

## Notes

- **Port** : 3000 (comme faucet)
- **Architecture** : Serveur unifié (backend sert le frontend CSR)
- **User** : `massload` (non-root, comme faucet)
- **Health** : `/health` endpoint
- **Stateless** : Pas de volumes persistants nécessaires

