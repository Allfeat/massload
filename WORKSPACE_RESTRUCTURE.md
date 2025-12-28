# 🔄 Guide de Restructuration - Allfeat Apps Hub

Ce guide décrit les étapes concrètes pour transformer le projet `massload` actuel en **Allfeat Apps Hub**.

---

## 📁 Structure Actuelle vs Cible

### Actuel (massload)
```
massload/
├── backend/           # Axum server + AI transform
├── frontend/          # Leptos CSR
├── Cargo.toml         # Workspace 2 members
└── ...
```

### Cible (allfeat-apps)
```
allfeat-apps/
├── crates/
│   ├── core/         # Modèles + validation partagés
│   ├── ui/           # Composants Leptos partagés  
│   └── services/     # Services backend partagés
├── apps/
│   └── hub/          # Application principale
│       ├── backend/  # Axum unified API
│       └── frontend/ # Leptos app avec router
├── schemas/          # JSON Schemas MIDDS
└── Cargo.toml        # Workspace N members
```

---

## 🛠️ Étapes de Migration

### Étape 1: Préparer la nouvelle structure

```bash
# Depuis la racine du projet
mkdir -p crates/{core,ui,services}/src
mkdir -p apps/hub/{backend,frontend}/src
mkdir -p apps/hub/frontend/src/pages/{home,register,protect,massload,explorer,faucet}
mkdir -p schemas
```

### Étape 2: Déplacer les schemas

```bash
mv backend/schemas/*.json schemas/
```

### Étape 3: Créer le crate `core`

Extraire de `backend/src/`:
- `models/` → `crates/core/src/models/`
- `validation/` → `crates/core/src/validation/`
- `error.rs` → `crates/core/src/error.rs`

### Étape 4: Créer le crate `ui`

Extraire de `frontend/src/`:
- `components/header.rs` → `crates/ui/src/components/header.rs`
- `components/footer.rs` → `crates/ui/src/components/footer.rs`
- `components/wallet_modal.rs` → `crates/ui/src/components/wallet/modal.rs`
- `i18n.rs` → `crates/ui/src/i18n/`

### Étape 5: Créer le crate `services`

Extraire de `backend/src/`:
- `ai/` → `crates/services/src/ai/`
- `parser/` → `crates/services/src/parser/`
- `transform/` → `crates/services/src/transform/`
- `cache/` → `crates/services/src/cache/`

### Étape 6: Créer l'application Hub

L'app hub combine:
- Backend Axum servant l'API unifiée
- Frontend Leptos avec router multi-pages

---

## 📝 Fichiers de Configuration

### Nouveau `Cargo.toml` (racine)

```toml
[workspace]
resolver = "2"
members = [
    "crates/core",
    "crates/ui", 
    "crates/services",
    "apps/hub/backend",
    "apps/hub/frontend",
]

[workspace.package]
version = "0.2.0"
edition = "2021"
rust-version = "1.75"
authors = ["Allfeat <hello@allfeat.com>"]
license = "GPL-3.0"
repository = "https://github.com/allfeat/allfeat-apps"
homepage = "https://allfeat.org"

[workspace.dependencies]
# Core
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.11", features = ["v4", "serde"] }

# Validation
jsonschema = "0.37"
regex = "1.10"

# Async
tokio = { version = "1.0", features = ["rt-multi-thread", "macros", "fs"] }
futures = "0.3"

# Web Backend
axum = { version = "0.8", features = ["multipart"] }
tower-http = { version = "0.6", features = ["cors", "fs", "compression-gzip"] }
reqwest = { version = "0.12", features = ["json"] }

# Web Frontend (WASM)
leptos = { version = "0.7", features = ["csr"] }
leptos_router = { version = "0.7", features = ["csr"] }
leptos_meta = { version = "0.7", features = ["csr"] }
wasm-bindgen = { version = "0.2", features = ["serde-serialize"] }
wasm-bindgen-futures = "0.4"
web-sys = "0.3"
js-sys = "0.3"
gloo-net = "0.6"
serde-wasm-bindgen = "0.6"
console_log = "1.0"
log = "0.4"

# Internal crates
allfeat-core = { path = "crates/core" }
allfeat-ui = { path = "crates/ui" }
allfeat-services = { path = "crates/services" }
```

### `crates/core/Cargo.toml`

```toml
[package]
name = "allfeat-core"
version.workspace = true
edition.workspace = true
description = "Core models and validation for Allfeat ecosystem"
license.workspace = true

[dependencies]
serde.workspace = true
serde_json.workspace = true
thiserror.workspace = true
chrono.workspace = true
uuid.workspace = true
jsonschema.workspace = true
regex.workspace = true

[dev-dependencies]
```

### `crates/ui/Cargo.toml`

```toml
[package]
name = "allfeat-ui"
version.workspace = true
edition.workspace = true
description = "Shared UI components for Allfeat apps (Leptos)"
license.workspace = true

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
allfeat-core.workspace = true
leptos.workspace = true
leptos_router.workspace = true
wasm-bindgen.workspace = true
web-sys = { workspace = true, features = [
    "Window", "Document", "Element", "HtmlElement",
    "Storage", "Navigator"
] }
js-sys.workspace = true
serde.workspace = true
serde_json.workspace = true
log.workspace = true
```

### `crates/services/Cargo.toml`

```toml
[package]
name = "allfeat-services"
version.workspace = true
edition.workspace = true
description = "Backend services for Allfeat apps (AI, parsing, transformation)"
license.workspace = true

[dependencies]
allfeat-core.workspace = true
serde.workspace = true
serde_json.workspace = true
thiserror.workspace = true
tokio.workspace = true
reqwest.workspace = true
csv = "1.3"
chardet = "0.2"
encoding_rs = "0.8"
```

### `apps/hub/backend/Cargo.toml`

```toml
[package]
name = "allfeat-hub"
version.workspace = true
edition.workspace = true
description = "Allfeat Apps Hub - Unified server"
license.workspace = true

[[bin]]
name = "allfeat-hub"
path = "src/main.rs"

[dependencies]
allfeat-core.workspace = true
allfeat-services.workspace = true
axum.workspace = true
tower-http.workspace = true
tokio.workspace = true
serde.workspace = true
serde_json.workspace = true
dotenvy = "0.15"
tokio-stream = { version = "0.1", features = ["sync"] }
futures.workspace = true
```

### `apps/hub/frontend/Cargo.toml`

```toml
[package]
name = "allfeat-hub-frontend"
version.workspace = true
edition.workspace = true
description = "Allfeat Apps Hub - Frontend (Leptos CSR)"
license.workspace = true

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
allfeat-core.workspace = true
allfeat-ui.workspace = true
leptos.workspace = true
leptos_router.workspace = true
leptos_meta.workspace = true
wasm-bindgen.workspace = true
wasm-bindgen-futures.workspace = true
web-sys = { workspace = true, features = [
    "File", "FileList", "FormData", "Window", "Blob",
    "HtmlInputElement", "EventTarget", "Event", "Document"
] }
js-sys.workspace = true
serde.workspace = true
serde_json.workspace = true
serde-wasm-bindgen.workspace = true
gloo-net.workspace = true
console_log.workspace = true
log.workspace = true
console_error_panic_hook = "0.1"
getrandom = { version = "0.2", features = ["js"] }
chrono = { workspace = true, features = ["wasm-bindgen"] }
```

---

## 🔀 Routing Frontend

### Structure des Routes

```rust
// apps/hub/frontend/src/routes.rs

use leptos::*;
use leptos_router::*;

#[component]
pub fn AppRouter() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                // Home
                <Route path="/" view=HomePage/>
                
                // Register MIDDS
                <Route path="/register" view=RegisterHub/>
                <Route path="/register/musical-work" view=RegisterMusicalWork/>
                <Route path="/register/recording" view=RegisterRecording/>
                <Route path="/register/release" view=RegisterRelease/>
                <Route path="/register/artist" view=RegisterArtist/>
                <Route path="/register/legal-entity" view=RegisterLegalEntity/>
                
                // Protect (ATS)
                <Route path="/protect" view=ProtectHub/>
                <Route path="/protect/new" view=ProtectNew/>
                <Route path="/protect/verify" view=ProtectVerify/>
                
                // Massload
                <Route path="/massload" view=MassloadPage/>
                <Route path="/massload/templates" view=MassloadTemplates/>
                
                // Explorer
                <Route path="/explorer" view=ExplorerPage/>
                <Route path="/explorer/work/:iswc" view=ExplorerWorkDetail/>
                <Route path="/explorer/recording/:isrc" view=ExplorerRecordingDetail/>
                
                // Faucet
                <Route path="/faucet" view=FaucetPage/>
                
                // 404
                <Route path="/*any" view=NotFound/>
            </Routes>
        </Router>
    }
}
```

---

## 🎨 Composants Partagés à Créer

### Header avec Navigation

```rust
// crates/ui/src/components/header.rs

#[component]
pub fn AppHeader(
    wallet_connected: ReadSignal<bool>,
    wallet_address: ReadSignal<Option<String>>,
    on_connect: Callback<()>,
) -> impl IntoView {
    view! {
        <header class="app-header">
            <nav class="nav-container">
                <a href="/" class="logo">
                    <img src="/assets/logo.svg" alt="Allfeat"/>
                </a>
                
                <div class="nav-links">
                    <NavDropdown 
                        label="Register" 
                        items=vec![
                            ("Musical Work", "/register/musical-work"),
                            ("Recording", "/register/recording"),
                            ("Release", "/register/release"),
                            ("Artist", "/register/artist"),
                            ("Legal Entity", "/register/legal-entity"),
                        ]
                    />
                    
                    <NavDropdown 
                        label="Protect" 
                        items=vec![
                            ("New Timestamp", "/protect/new"),
                            ("Verify Proof", "/protect/verify"),
                        ]
                    />
                    
                    <a href="/massload" class="nav-link">"Massload"</a>
                    <a href="/explorer" class="nav-link">"Explorer"</a>
                    <a href="/faucet" class="nav-link">"Faucet"</a>
                </div>
                
                <WalletButton 
                    connected=wallet_connected
                    address=wallet_address
                    on_click=on_connect
                />
            </nav>
        </header>
    }
}
```

---

## 🏗️ Scripts de Build

### `build.sh`

```bash
#!/bin/bash
set -e

echo "🔨 Building Allfeat Apps Hub..."

# Build frontend (WASM)
echo "📦 Building frontend..."
cd apps/hub/frontend
trunk build --release
cd ../../..

# Build backend
echo "🦀 Building backend..."
cargo build --release -p allfeat-hub

echo "✅ Build complete!"
echo "   Frontend: apps/hub/frontend/dist/"
echo "   Backend:  target/release/allfeat-hub"
```

### `start.sh`

```bash
#!/bin/bash
set -e

# Load env
if [ -f .env ]; then
    export $(cat .env | grep -v '#' | xargs)
fi

PORT=${PORT:-3000}

echo "🚀 Starting Allfeat Apps Hub on port $PORT..."
./target/release/allfeat-hub serve --port $PORT
```

---

## 🐳 Dockerfile

```dockerfile
# Stage 1: Build frontend
FROM rust:1.75-bookworm AS frontend-builder
WORKDIR /app
RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown
COPY . .
WORKDIR /app/apps/hub/frontend
RUN trunk build --release

# Stage 2: Build backend
FROM rust:1.75-bookworm AS backend-builder
WORKDIR /app
COPY . .
RUN cargo build --release -p allfeat-hub

# Stage 3: Runtime
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy backend binary
COPY --from=backend-builder /app/target/release/allfeat-hub /app/

# Copy frontend dist
COPY --from=frontend-builder /app/apps/hub/frontend/dist /app/frontend/dist

ENV PORT=3000
EXPOSE 3000

CMD ["/app/allfeat-hub", "serve"]
```

---

## ✅ Checklist de Migration

- [ ] Créer la structure de dossiers
- [ ] Déplacer les JSON schemas
- [ ] Créer `crates/core` avec models + validation
- [ ] Créer `crates/ui` avec composants partagés
- [ ] Créer `crates/services` avec AI/parser/transform
- [ ] Créer `apps/hub/backend` avec API unifiée
- [ ] Créer `apps/hub/frontend` avec router
- [ ] Migrer la page Massload
- [ ] Créer les pages Register (depuis scratch, inspiré de l'actuel TS)
- [ ] Créer les pages Protect (depuis scratch, inspiré de l'actuel TS)
- [ ] Créer la page Explorer
- [ ] Créer la page Faucet
- [ ] Tests E2E
- [ ] Documentation
- [ ] Déploiement

---

*Guide créé pour la migration Allfeat Apps Hub*

