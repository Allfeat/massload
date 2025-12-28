# 🎵 Allfeat Apps Hub - Architecture Proposal

## Executive Summary

Cette proposition vise à créer un **Layer Métier Unifié** pour l'écosystème Allfeat, inspiré du modèle [apps.tanssi.network](https://apps.tanssi.network/). L'objectif est de consolider les applications existantes (Massload, Register, Protect) dans une plateforme cohérente tout en permettant leur évolution indépendante.

---

## 📊 Analyse de l'Existant

### Applications Actuelles

| App | URL | Stack | Fonction |
|-----|-----|-------|----------|
| **Massload** | (interne) | Rust (Axum + Leptos CSR) | Import CSV bulk → MIDDS → Blockchain |
| **Register** | register.allfeat.org | TypeScript (Next.js?) | Enregistrement unitaire MIDDS |
| **Protect** | protect.allfeat.org | TypeScript (Next.js?) | Allfeat Time Stamp (ATS) |
| **Faucet** | faucet.allfeat.org | ? | Distribution tokens testnet |

### Stack Massload Actuelle

```
┌─────────────────────────────────────────────────────────────┐
│              Massload Unified Server                        │
├─────────────────────────────────────────────────────────────┤
│  Backend (Axum - Rust)                                      │
│  • POST /api/upload   → CSV → AI Transform → MIDDS JSON    │
│  • GET  /api/logs     → SSE real-time logs                 │
│  • GET  /health       → Health check                       │
│  • Serves static frontend                                   │
├─────────────────────────────────────────────────────────────┤
│  Frontend (Leptos CSR - WASM)                              │
│  • Drag & drop upload                                       │
│  • Wallet integration (@allfeat/client)                    │
│  • Sign & submit via SDK                                    │
└─────────────────────────────────────────────────────────────┘
```

### Points Forts à Conserver
- ✅ Architecture monolithique simple (single container)
- ✅ SDK @allfeat/client pour blockchain
- ✅ Support wallets Polkadot (SubWallet, Talisman, Polkadot.js)
- ✅ MIDDS schemas bien définis
- ✅ Leptos CSR performant en WASM

---

## 🏗️ Architecture Proposée : Allfeat Apps Hub

### Inspiration Tanssi

Tanssi organise son portail en **catégories fonctionnelles** :
- Home (Dashboard global)
- Stake (Staking center)
- Bridge (Cross-chain)
- Ecosystem (Appchains explorer)
- Build Appchains (Tools)
- Operate Network (Operators)
- Learn (Documentation)

### Transposition Allfeat

```
┌──────────────────────────────────────────────────────────────────────┐
│                      apps.allfeat.org                                │
│                    (Allfeat Apps Hub)                                │
├──────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐ │
│  │   Home      │  │  Register   │  │   Protect   │  │  Massload   │ │
│  │  Dashboard  │  │   MIDDS     │  │    ATS      │  │   Bulk      │ │
│  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘ │
│                                                                      │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐ │
│  │  Explorer   │  │   Verify    │  │   Faucet    │  │   Learn     │ │
│  │  Catalog    │  │   Proof     │  │   Tokens    │  │   Docs      │ │
│  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘ │
│                                                                      │
├──────────────────────────────────────────────────────────────────────┤
│                     Shared Components                                │
│  • Header + Navigation     • Wallet Connection     • Footer         │
│  • Theme (light/dark)      • i18n (FR/EN)          • Notifications  │
└──────────────────────────────────────────────────────────────────────┘
```

---

## 🔧 Architecture Technique Détaillée

### Monorepo Workspace Structure

```
allfeat-apps/
├── Cargo.toml                     # Workspace root
├── rust-toolchain.toml
│
├── crates/
│   │
│   ├── core/                      # 🧱 Shared Core Library
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── models/            # Domain models (MIDDS, Party, Work, Recording, Release)
│   │       │   ├── mod.rs
│   │       │   ├── party.rs       # PartyId, CreatorRole, Artist, LegalEntity
│   │       │   ├── musical_work.rs
│   │       │   ├── recording.rs
│   │       │   ├── release.rs
│   │       │   └── timestamp.rs   # ATS (Allstamp)
│   │       ├── validation/        # JSON Schema validators
│   │       ├── blockchain/        # @allfeat/client bindings (WASM)
│   │       └── error.rs
│   │
│   ├── ui/                        # 🎨 Shared UI Components (Leptos)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── components/
│   │       │   ├── header.rs      # App header + navigation
│   │       │   ├── footer.rs
│   │       │   ├── wallet/        # Wallet connection modal & status
│   │       │   ├── forms/         # Form components (input, select, file upload)
│   │       │   ├── cards/         # Card layouts
│   │       │   ├── tables/        # Data tables
│   │       │   └── notifications/ # Toast notifications
│   │       ├── hooks/             # Reactive hooks
│   │       ├── i18n/              # Internationalization
│   │       └── theme/             # Theme system (dark/light)
│   │
│   ├── services/                  # 🔌 Backend Services (Axum)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── ai/                # Claude AI integration
│   │       ├── parser/            # CSV parsing
│   │       ├── transform/         # DSL transformation
│   │       ├── cache/             # Template caching
│   │       └── timestamp/         # Hash generation for ATS
│   │
│   └── cli/                       # 🖥️ CLI Tools
│       ├── Cargo.toml
│       └── src/
│           └── main.rs
│
├── apps/
│   │
│   ├── hub/                       # 🏠 Main Hub Application
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── main.rs           # Axum server (API + static serving)
│   │   │   ├── api/              # Unified API routes
│   │   │   └── routes.rs         # Route definitions
│   │   └── frontend/
│   │       ├── Cargo.toml        # Leptos frontend
│   │       ├── Trunk.toml
│   │       ├── index.html
│   │       ├── src/
│   │       │   ├── lib.rs
│   │       │   ├── app.rs        # Main app with router
│   │       │   └── pages/
│   │       │       ├── home.rs          # Dashboard
│   │       │       ├── register/        # MIDDS Registration
│   │       │       │   ├── mod.rs
│   │       │       │   ├── musical_work.rs
│   │       │       │   ├── recording.rs
│   │       │       │   ├── release.rs
│   │       │       │   ├── artist.rs
│   │       │       │   └── legal_entity.rs
│   │       │       ├── protect/         # ATS (Allstamp)
│   │       │       │   ├── mod.rs
│   │       │       │   ├── protect.rs
│   │       │       │   └── verify.rs
│   │       │       ├── massload/        # Bulk import
│   │       │       │   ├── mod.rs
│   │       │       │   └── upload.rs
│   │       │       ├── explorer/        # Catalog explorer
│   │       │       ├── faucet/          # Token faucet
│   │       │       └── learn/           # Documentation links
│   │       └── style/
│   │           └── main.css
│   │
│   └── standalone/               # 📦 Standalone builds (optional)
│       ├── massload/             # Standalone massload (current)
│       ├── register/             # Standalone register
│       └── protect/              # Standalone protect
│
├── schemas/                      # 📋 JSON Schemas
│   ├── midds-musical-work.json
│   ├── midds-recording.json
│   ├── midds-release.json
│   ├── midds-artist.json
│   ├── midds-legal-entity.json
│   └── timestamp-proof.json
│
├── docker/
│   ├── Dockerfile.hub           # Production image
│   └── docker-compose.yml       # Local development
│
└── deploy/
    ├── k8s/                     # Kubernetes manifests
    └── terraform/               # Infrastructure as Code
```

---

## 🎯 Mapping des Features par Module

### 1. Home Dashboard (`/`)

```rust
// apps/hub/frontend/src/pages/home.rs
- Statistiques globales blockchain (works registered, timestamps, etc.)
- Quick actions (cards vers Register, Protect, Massload)
- Recent activity feed
- Network status
```

### 2. Register MIDDS (`/register/*`)

| Route | Fonction | Actuel |
|-------|----------|--------|
| `/register` | Hub registration avec cards | register.allfeat.org |
| `/register/musical-work` | Formulaire œuvre musicale | register.allfeat.org/fr/musical-work |
| `/register/recording` | Formulaire recording | register.allfeat.org/fr/recording |
| `/register/release` | Formulaire release | register.allfeat.org/fr/release |
| `/register/artist` | Formulaire artiste | register.allfeat.org/fr/coming-soon |
| `/register/legal-entity` | Formulaire entité légale | register.allfeat.org/fr/coming-soon |

### 3. Protect ATS (`/protect/*`)

| Route | Fonction | Actuel |
|-------|----------|--------|
| `/protect` | Hub protection | protect.allfeat.org |
| `/protect/new` | Créer un timestamp | protect.allfeat.org/fr/protect |
| `/protect/verify` | Vérifier un proof | protect.allfeat.org/fr/proof |

### 4. Massload Bulk (`/massload`)

| Route | Fonction |
|-------|----------|
| `/massload` | Upload CSV → Transform → Register batch |
| `/massload/templates` | Gestion templates de transformation |

### 5. Explorer (`/explorer`)

| Route | Fonction |
|-------|----------|
| `/explorer` | Recherche dans le catalogue |
| `/explorer/work/:iswc` | Détail œuvre |
| `/explorer/recording/:isrc` | Détail recording |
| `/explorer/party/:id` | Détail party (artist/entity) |

### 6. Faucet (`/faucet`)

| Route | Fonction |
|-------|----------|
| `/faucet` | Demander des tokens testnet |

---

## 🔄 Data Flow Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                           Browser (WASM)                            │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │                    Leptos Frontend                           │   │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐       │   │
│  │  │ Register │ │ Protect  │ │ Massload │ │ Explorer │       │   │
│  │  └────┬─────┘ └────┬─────┘ └────┬─────┘ └────┬─────┘       │   │
│  │       │            │            │            │              │   │
│  │  ┌────▼────────────▼────────────▼────────────▼─────┐       │   │
│  │  │              UI Components (crates/ui)           │       │   │
│  │  │  • Wallet Modal  • Forms  • Cards  • i18n       │       │   │
│  │  └────┬────────────────────────────────────────────┘       │   │
│  │       │                                                     │   │
│  │  ┌────▼─────────────────────────────────────────────────┐  │   │
│  │  │              Core (crates/core)                       │  │   │
│  │  │  • Models (MIDDS)  • Validation  • Blockchain bindings│  │   │
│  │  └────┬──────────────────────────────────┬──────────────┘  │   │
│  │       │                                  │                  │   │
│  │       │ JS Interop                       │ Direct          │   │
│  │       ▼                                  ▼                  │   │
│  │  ┌─────────────────┐              ┌──────────────────┐     │   │
│  │  │ @allfeat/client │              │ Backend API      │     │   │
│  │  │ (SDK via ESM)   │              │ (Axum)           │     │   │
│  │  └────────┬────────┘              └────────┬─────────┘     │   │
│  └───────────┼────────────────────────────────┼───────────────┘   │
└──────────────┼────────────────────────────────┼───────────────────┘
               │                                │
               ▼                                ▼
┌──────────────────────────┐     ┌──────────────────────────────────┐
│   Allfeat Blockchain     │     │   Backend Services               │
│   (Melodie Runtime)      │     │   • AI Transform (Claude)        │
│   • MusicalWorks         │     │   • CSV Parsing                  │
│   • Recordings           │     │   • Template Cache               │
│   • Releases             │     │   • Timestamp generation         │
│   • Parties              │     └──────────────────────────────────┘
│   • Timestamps           │
└──────────────────────────┘
```

---

## 🎨 Design System

### Theme Variables

```css
:root {
  /* Allfeat Brand Colors */
  --color-primary: #6366f1;      /* Indigo */
  --color-secondary: #8b5cf6;    /* Violet */
  --color-accent: #f59e0b;       /* Amber */
  
  /* Semantic */
  --color-success: #10b981;
  --color-warning: #f59e0b;
  --color-error: #ef4444;
  --color-info: #3b82f6;
  
  /* Dark theme */
  --bg-primary: #0f172a;
  --bg-secondary: #1e293b;
  --bg-card: #334155;
  --text-primary: #f8fafc;
  --text-secondary: #94a3b8;
  
  /* Light theme */
  --bg-primary-light: #ffffff;
  --bg-secondary-light: #f1f5f9;
  --text-primary-light: #0f172a;
}
```

### Navigation Structure

```
┌─────────────────────────────────────────────────────────────────────┐
│ [Logo Allfeat]  Register  Protect  Massload  Explorer  [Wallet]    │
│                    ↓         ↓                                      │
│              ┌─────────┐ ┌─────────┐                               │
│              │ Musical │ │ New     │                               │
│              │ Work    │ │ Stamp   │                               │
│              │ Record  │ │ Verify  │                               │
│              │ Release │ └─────────┘                               │
│              │ Artist  │                                           │
│              │ Entity  │                                           │
│              └─────────┘                                           │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 🚀 Migration Plan

### Phase 1: Core Extraction (Week 1-2)
1. Extraire les models de `massload/backend/src/models/` → `crates/core/`
2. Extraire les validations → `crates/core/validation/`
3. Créer les bindings blockchain → `crates/core/blockchain/`

### Phase 2: UI Components (Week 2-3)
1. Extraire les composants Leptos → `crates/ui/`
2. Créer le système de thème
3. Implémenter i18n unifié

### Phase 3: Hub Application (Week 3-5)
1. Créer `apps/hub/` avec Axum + Leptos
2. Migrer Massload comme page `/massload`
3. Recréer Register comme pages `/register/*`
4. Recréer Protect comme pages `/protect/*`

### Phase 4: Polish & Deploy (Week 5-6)
1. Explorer (lecture blockchain)
2. Faucet integration
3. Tests E2E
4. Déploiement Kubernetes

---

## 📦 Dependencies

### Core Crate
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
jsonschema = "0.37"
thiserror = "1.0"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.11", features = ["v4", "serde"] }
```

### UI Crate (WASM)
```toml
[dependencies]
leptos = { version = "0.7", features = ["csr"] }
leptos_router = { version = "0.7", features = ["csr"] }
wasm-bindgen = { version = "0.2", features = ["serde-serialize"] }
web-sys = "0.3"
js-sys = "0.3"
gloo-net = "0.6"
serde-wasm-bindgen = "0.6"
```

### Hub Backend
```toml
[dependencies]
axum = { version = "0.8", features = ["multipart"] }
tokio = { version = "1.0", features = ["full"] }
tower-http = { version = "0.6", features = ["cors", "fs", "compression-gzip"] }
reqwest = { version = "0.12", features = ["json"] }
```

---

## 🔗 API Endpoints (Unified)

| Method | Path | Description |
|--------|------|-------------|
| GET | `/health` | Health check |
| GET | `/api/v1/stats` | Dashboard statistics |
| POST | `/api/v1/upload` | Massload CSV upload |
| GET | `/api/v1/logs` | SSE logs stream |
| POST | `/api/v1/timestamp/create` | Create ATS proof |
| POST | `/api/v1/timestamp/verify` | Verify ATS proof |
| GET | `/api/v1/templates` | List transformation templates |
| GET | `/api/v1/explorer/works` | Search works |
| GET | `/api/v1/explorer/work/:iswc` | Get work by ISWC |
| `*` | `/*` | Serve frontend (fallback) |

---

## 🎯 Benefits

1. **Unified Experience** - Single entry point for all Allfeat tools
2. **Code Reuse** - Shared components, models, validation
3. **Consistent UI** - Same design system across all features
4. **Single Deployment** - One container, one domain
5. **Pure Rust** - Replace TypeScript with Rust/WASM for consistency
6. **Scalable** - Add new features as pages without new repos
7. **SEO/Routing** - Client-side routing with Leptos Router

---

## 📚 References

- [Tanssi Apps Portal](https://apps.tanssi.network/) - Architecture inspiration
- [Allfeat Documentation](https://docs.allfeat.org/) - Technical specs
- [Allfeat SDK](https://github.com/Allfeat/client) - @allfeat/client
- [Leptos Framework](https://leptos.dev/) - Rust frontend
- [Axum Framework](https://docs.rs/axum) - Rust backend

---

*Proposal by AI Assistant - December 2024*

