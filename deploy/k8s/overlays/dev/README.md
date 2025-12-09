# Dev Environment Deployment

## Issue: Mixed Content Error

The frontend was showing **Mixed Content** errors:

```
Blocage du chargement du contenu mixte actif (mixed active content)
« http://massload-backend-service:3000/api/upload »
```

**Cause**: Frontend (HTTPS) calling backend with HTTP internal service URL.

**Solution**: Use public HTTPS backend URL via Kustomize overlay.

## Deploy to Dev

```bash
kubectl apply -k deploy/k8s/overlays/dev/
```

This will:
- Set `BACKEND_URL=https://api.massload-dev.allfeat.org`
- Configure ingress hosts for dev environment
- Use dev blockchain node

## After Deployment

Restart frontend to pick up new ConfigMap:

```bash
kubectl rollout restart deployment/massload-frontend -n massload
```

Verify the configuration:

```bash
# Check config.js
curl https://massload-dev.allfeat.org/config.js

# Should show:
# window.MASSLOAD_CONFIG = {
#     BACKEND_URL: "https://api.massload-dev.allfeat.org",
#     ...
# }

# Test backend API
curl https://api.massload-dev.allfeat.org/health
```

## Requirements

- Ingress must route `api.massload-dev.allfeat.org` to backend service
- TLS certificate must cover both domains
- Backend service must be reachable via ingress

## Troubleshooting

If Mixed Content errors persist:
1. Check ConfigMap: `kubectl get configmap massload-config -n massload -o yaml`
2. Check pods picked up new config: `kubectl rollout status deployment/massload-frontend -n massload`
3. Verify ingress: `kubectl get ingress massload-ingress -n massload`
4. Test backend directly: `curl https://api.massload-dev.allfeat.org/health`

