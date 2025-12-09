# 🔍 DEBUG: MassLoad K8s Deployment

**Date**: 2025-12-09  
**Commit actuel**: 172223c (fix: proper nginx-unprivileged configuration for K8s)  
**Image attendue**: `ghcr.io/allfeat/massload-frontend:sha-172223c4c4123ac44e738f47e5a451b758eac281`  
**Workflow**: ✅ SUCCESS (14:38:38 UTC)  
**Problème**: Site affiche toujours l'ancien design malgré workflow réussi

---

## ⚠️ SYMPTÔMES

1. Workflow GitHub Actions réussit
2. Image Docker buildée et pushée sur GHCR
3. Repo `allfeat/infra-kube` mis à jour (commit df28a8d puis autres)
4. MAIS: https://massload-dev.allfeat.org/ affiche toujours l'ancien design
5. Crash silencieux possible dans K8s
6. **ConfigMap modifiée mais pod pas redémarré** ⚠️ CAUSE PROBABLE

---

## 🎯 PROBLÈME PRINCIPAL: CONFIGMAP

**⚠️ CAUSE PROBABLE DU PROBLÈME**

Les variables d'environnement injectées depuis une ConfigMap sont:
- ✅ Lues **AU DÉMARRAGE** du pod
- ❌ **PAS mises à jour** si la ConfigMap change après

**Si la ConfigMap a été modifiée:**
```yaml
# deploy/k8s/configmap.yaml
data:
  BACKEND_URL: "https://api.massload-dev.allfeat.org"  # ← Modifié ?
  BLOCKCHAIN_RPC: "wss://node-dev.allfeat.io"
```

**Et que le pod n'a pas été redémarré:**
- Le pod utilise toujours l'**ancienne** ConfigMap
- Le `docker-entrypoint.sh` génère `config.js` avec les anciennes valeurs
- Même si l'image Docker est nouvelle, la config est obsolète

**SOLUTION IMMÉDIATE:**
```bash
kubectl rollout restart deployment/massload-frontend -n massload
```

Cela force la recréation des pods qui liront la ConfigMap actuelle.

---

## 🔍 COMMANDES DE DIAGNOSTIC (pour admin K8s)

### 1️⃣ Vérifier le status du pod

```bash
# Context K8s
kubectl config use-context <cluster-dev>

# Status du deployment
kubectl get deployment -n massload massload-frontend

# Status des pods
kubectl get pods -n massload -l app=massload-frontend

# Détails du deployment
kubectl describe deployment -n massload massload-frontend
```

**À vérifier:**
- [ ] READY: 1/1 (si 0/1 = pod crash)
- [ ] STATUS: Running (si CrashLoopBackOff = problème)
- [ ] RESTARTS: 0 (si >0 = pod restart en boucle)

---

### 2️⃣ Vérifier l'image déployée

```bash
# Image actuellement déployée
kubectl get deployment massload-frontend -n massload \
  -o jsonpath='{.spec.template.spec.containers[0].image}'

# Doit afficher:
# ghcr.io/allfeat/massload-frontend:sha-172223c4c4123ac44e738f47e5a451b758eac281
```

**Si l'image est différente:**
- ArgoCD n'a pas synchronisé
- Kustomize n'a pas appliqué le changement
- Le repo infra-kube a un problème

---

### 3️⃣ Voir les LOGS du pod

```bash
# Logs en temps réel
kubectl logs -n massload -l app=massload-frontend -f --tail=50

# Logs du pod actuel
POD=$(kubectl get pods -n massload -l app=massload-frontend -o jsonpath='{.items[0].metadata.name}')
kubectl logs -n massload $POD

# Logs du pod PRÉCÉDENT (si crash)
kubectl logs -n massload $POD --previous
```

**À chercher:**
- `[MassLoad] Generating runtime config...` (bon signe)
- `[MassLoad] Starting nginx...` (bon signe)
- `nginx: [emerg]` (problème de config)
- `chown(...) failed (1: Operation not permitted)` (problème de permissions)
- `Permission denied` (problème de permissions)

---

### 4️⃣ Vérifier les événements K8s

```bash
# Événements récents
kubectl get events -n massload --sort-by='.lastTimestamp' | grep -i massload-frontend | tail -20
```

**À chercher:**
- `Failed to pull image` (problème GHCR)
- `Back-off restarting failed container` (crash loop)
- `Liveness probe failed` (healthcheck fail)
- `ImagePullBackOff` (image introuvable)

---

### 5️⃣ Tester depuis l'intérieur du pod

```bash
# Shell dans le pod
kubectl exec -it -n massload $POD -- sh

# Une fois dans le pod:
# 1. Vérifier que nginx écoute sur 8080
netstat -tulpn | grep 8080
# ou
curl -s http://localhost:8080/ | head -20

# 2. Vérifier config.js
cat /usr/share/nginx/html/config.js

# 3. Vérifier les fichiers statiques
ls -lah /usr/share/nginx/html/
ls -lah /usr/share/nginx/html/public/

# 4. Vérifier les permissions
ls -ld /usr/share/nginx/html/
id  # Doit afficher: uid=101(nginx) gid=101(nginx)

# 5. Vérifier le logo
ls -lah /usr/share/nginx/html/public/logo-*.png
```

---

### 6️⃣ Vérifier la ConfigMap

```bash
# Voir le contenu de la ConfigMap actuelle
kubectl get configmap massload-config -n massload -o yaml

# Vérifier quand elle a été modifiée
kubectl describe configmap massload-config -n massload

# Comparer avec ce qui est attendu
kubectl get configmap massload-config -n massload \
  -o jsonpath='{.data.BACKEND_URL}{"\n"}{.data.BLOCKCHAIN_RPC}{"\n"}'

# Doit afficher:
# https://api.massload-dev.allfeat.org
# wss://node-dev.allfeat.io
```

**Si la ConfigMap a été modifiée récemment:**

Le pod doit être redémarré pour prendre en compte les changements !

```bash
# Forcer le restart du deployment
kubectl rollout restart deployment/massload-frontend -n massload

# Suivre le status du rollout
kubectl rollout status deployment/massload-frontend -n massload
```

---

### 7️⃣ Vérifier ArgoCD

#### Via UI

Aller sur: https://argocd.allfeat.io (ou URL de votre ArgoCD)

1. Chercher l'app: `massload-frontend`
2. Vérifier le status:
   - ✅ **Synced** + **Healthy** = OK
   - ⚠️ **Out of Sync** = Pas synchronisé → Cliquer **SYNC**
   - ❌ **Degraded** = Problème → Voir les détails

#### Via CLI

```bash
# Lister les apps
argocd app list | grep massload

# Détails de l'app
argocd app get massload-frontend

# Forcer le sync
argocd app sync massload-frontend

# Logs via ArgoCD
argocd app logs massload-frontend
```

---

### 8️⃣ Forcer un redéploiement

**⚠️ NÉCESSAIRE si la ConfigMap a été modifiée !**

Si tout semble OK mais le site n'est pas à jour:

```bash
# Option 1: Rollout restart (force recreation des pods)
kubectl rollout restart deployment/massload-frontend -n massload
kubectl rollout status deployment/massload-frontend -n massload

# Option 2: Scaler à 0 puis à 1
kubectl scale deployment/massload-frontend -n massload --replicas=0
sleep 5
kubectl scale deployment/massload-frontend -n massload --replicas=1

# Option 3: Delete le pod (K8s le recréera)
kubectl delete pod -n massload -l app=massload-frontend
```

---

## 🔍 CHECKLIST DE DIAGNOSTIC

- [ ] Workflow GitHub Actions: SUCCESS
- [ ] Image Docker existe sur GHCR: `docker manifest inspect ghcr.io/allfeat/massload-frontend:sha-172223c...`
- [ ] Repo infra-kube mis à jour: Vérifier le dernier commit sur master
- [ ] ArgoCD status: Synced + Healthy
- [ ] Deployment status: READY 1/1
- [ ] Pod status: Running (pas CrashLoopBackOff)
- [ ] Pod restarts: 0 (ou très peu)
- [ ] Image dans deployment: `sha-172223c...`
- [ ] Logs pod: Pas d'erreur, nginx démarré
- [ ] Test interne: `curl http://localhost:8080/` fonctionne
- [ ] Service: Pointe vers le bon pod
- [ ] Ingress: Pointe vers le bon service
- [ ] Cache navigateur: Vidé (Ctrl+Shift+R)

---

## 🎯 SOLUTIONS POSSIBLES

### Problème 0: ConfigMap modifiée mais pod pas redémarré ⚠️ PRINCIPAL

**Symptôme**: 
- La ConfigMap `massload-config` a été modifiée
- Le pod n'a PAS été redémarré après la modification
- Le site affiche l'ancien design ou a une config obsolète

**Diagnostic**:
```bash
# Vérifier quand la ConfigMap a été modifiée
kubectl describe configmap massload-config -n massload | grep "Events:"

# Vérifier l'âge du pod
kubectl get pods -n massload -l app.kubernetes.io/component=frontend

# Si le pod est PLUS VIEUX que la dernière modification de la ConfigMap
# → Le pod utilise l'ancienne config !
```

**Solution**:
```bash
# Forcer le restart (RECOMMANDÉ)
kubectl rollout restart deployment/massload-frontend -n massload

# Ou delete le pod (il sera recréé)
kubectl delete pod -n massload -l app.kubernetes.io/component=frontend

# Vérifier que le nouveau pod démarre
kubectl get pods -n massload -w

# Vérifier les logs du nouveau pod
kubectl logs -n massload -l app.kubernetes.io/component=frontend --tail=20
```

**Prévention future**:

Installer **Reloader** de Stakater pour auto-restart les pods quand la ConfigMap change:
```bash
# Installer Reloader
kubectl apply -f https://raw.githubusercontent.com/stakater/Reloader/master/deployments/kubernetes/reloader.yaml

# Annoter le deployment
kubectl patch deployment massload-frontend -n massload \
  -p '{"spec":{"template":{"metadata":{"annotations":{"reloader.stakater.com/auto":"true"}}}}}'
```

---

### Problème 1: ArgoCD en mode manual

**Solution**: Forcer le sync dans ArgoCD UI ou CLI

### Problème 2: Image pas pullée

**Solution**:
```bash
kubectl delete pod -n massload -l app=massload-frontend
# K8s va recréer le pod et pull la nouvelle image
```

### Problème 3: Permission denied dans le pod

**Symptôme**: Logs montrent `chown(...) failed` ou `Permission denied`

**Solution**: Vérifier que le Dockerfile utilise bien:
- `FROM nginxinc/nginx-unprivileged:alpine`
- `USER nginx` après les opérations root
- Port 8080 (pas 80)
- **DÉJÀ CORRIGÉ dans commit 172223c** ✅

### Problème 4: Config.js pas généré

**Symptôme**: Site charge mais backend_url est undefined

**Solution**: Vérifier les variables d'environnement dans le deployment K8s:
```yaml
env:
  - name: BACKEND_URL
    value: "http://massload-backend-service:3000"
  - name: BLOCKCHAIN_RPC
    value: "wss://node-dev.allfeat.io"
```

### Problème 5: Ingress cache

**Solution**: Purger le cache Cloudflare ou Nginx Ingress Controller

---

## 📞 CONTACT

Si le problème persiste après ces vérifications:

1. Demander à Loïs de:
   - Vérifier les logs du pod
   - Forcer un `kubectl rollout restart`
   - Vérifier ArgoCD

2. Vérifier si https://massload-dev.allfeat.org/ est derrière un cache:
   - Cloudflare
   - CDN
   - Varnish
   - → Purger le cache

3. Vérifier les DNS et le certificat SSL:
   ```bash
   dig massload-dev.allfeat.org
   curl -vI https://massload-dev.allfeat.org/
   ```

---

## ✅ VALIDATION

Une fois corrigé, le site devrait afficher:

- ✅ Logo PNG Allfeat (light/dark selon thème)
- ✅ Sélecteur de langue (🌐 FR/EN/ES/DE/JA/KO/EL)
- ✅ Wallet Modal avec 3 wallets (SubWallet/Talisman/Polkadot.js)
- ✅ Traductions fonctionnelles
- ✅ Design aligné avec register.allfeat.org

**Logs attendus dans le pod:**
```
[MassLoad] Generating runtime config...
  BACKEND_URL: http://massload-backend-service:3000
  BLOCKCHAIN_RPC: wss://node-dev.allfeat.io
[MassLoad] Config generated at /usr/share/nginx/html/config.js
[MassLoad] Starting nginx...
nginx: starting worker processes
```


