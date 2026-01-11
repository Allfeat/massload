# 🏛️ Allfeat Apps Hub - Architecture Diagrams

## 1. Vue d'Ensemble Haut Niveau

```
                                    ┌─────────────────────────────────────┐
                                    │         apps.allfeat.org            │
                                    │      (Allfeat Apps Hub)             │
                                    └─────────────────────────────────────┘
                                                     │
                     ┌───────────────────────────────┼───────────────────────────────┐
                     │                               │                               │
           ┌─────────▼─────────┐         ┌──────────▼──────────┐         ┌──────────▼──────────┐
           │                   │         │                     │         │                     │
           │   REGISTRATION    │         │     PROTECTION      │         │       TOOLS         │
           │      MIDDS        │         │        ATS          │         │                     │
           │                   │         │                     │         │                     │
           │  • Musical Work   │         │  • Create Stamp     │         │  • Massload Bulk    │
           │  • Recording      │         │  • Verify Proof     │         │  • Explorer         │
           │  • Release        │         │                     │         │  • Faucet           │
           │  • Artist         │         └─────────────────────┘         │                     │
           │  • Legal Entity   │                                         └─────────────────────┘
           │                   │
           └───────────────────┘


                                    ┌─────────────────────────────────────┐
                                    │        Allfeat Blockchain           │
                                    │          (Melodie)                  │
                                    │                                     │
                                    │  • MusicalWorks Pallet              │
                                    │  • Recordings Pallet                │
                                    │  • Releases Pallet                  │
                                    │  • Parties Pallet                   │
                                    │  • Timestamps Pallet                │
                                    │                                     │
                                    └─────────────────────────────────────┘
```

---

## 2. Architecture Technique Détaillée

```
┌──────────────────────────────────────────────────────────────────────────────────────────┐
│                                      BROWSER                                              │
│                                                                                          │
│  ┌────────────────────────────────────────────────────────────────────────────────────┐ │
│  │                           Leptos Frontend (WASM)                                    │ │
│  │                                                                                     │ │
│  │  ┌─────────────────────────────────────────────────────────────────────────────┐  │ │
│  │  │                              Router                                          │  │ │
│  │  │   /           /register/*    /protect/*    /massload    /explorer    /faucet │  │ │
│  │  └────────┬─────────────┬─────────────┬───────────┬───────────┬───────────┬────┘  │ │
│  │           │             │             │           │           │           │        │ │
│  │  ┌────────▼────┐ ┌──────▼─────┐ ┌─────▼────┐ ┌────▼─────┐ ┌──▼────┐ ┌────▼────┐  │ │
│  │  │    Home     │ │  Register  │ │ Protect  │ │ Massload │ │Explorer│ │ Faucet  │  │ │
│  │  │  Dashboard  │ │   Pages    │ │  Pages   │ │  Page    │ │ Page   │ │  Page   │  │ │
│  │  └─────────────┘ └────────────┘ └──────────┘ └──────────┘ └────────┘ └─────────┘  │ │
│  │                                                                                     │ │
│  │  ┌─────────────────────────────────────────────────────────────────────────────┐  │ │
│  │  │                         allfeat-ui (Shared Components)                       │  │ │
│  │  │                                                                              │  │ │
│  │  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐           │  │ │
│  │  │  │  Header  │ │  Footer  │ │  Wallet  │ │  Forms   │ │  Cards   │           │  │ │
│  │  │  │          │ │          │ │  Modal   │ │          │ │          │           │  │ │
│  │  │  └──────────┘ └──────────┘ └──────────┘ └──────────┘ └──────────┘           │  │ │
│  │  │                                                                              │  │ │
│  │  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐                        │  │ │
│  │  │  │  Tables  │ │  Toasts  │ │  Theme   │ │   i18n   │                        │  │ │
│  │  │  │          │ │          │ │ Switcher │ │          │                        │  │ │
│  │  │  └──────────┘ └──────────┘ └──────────┘ └──────────┘                        │  │ │
│  │  └─────────────────────────────────────────────────────────────────────────────┘  │ │
│  │                                                                                     │ │
│  │  ┌─────────────────────────────────────────────────────────────────────────────┐  │ │
│  │  │                         allfeat-core (Domain)                                │  │ │
│  │  │                                                                              │  │ │
│  │  │  ┌─────────────────────────────┐  ┌─────────────────────────────────────┐  │  │ │
│  │  │  │          Models             │  │           Validation                 │  │  │ │
│  │  │  │  • PartyId (IPI/ISNI)       │  │  • JSON Schema validators           │  │  │ │
│  │  │  │  • CreatorRole              │  │  • ISWC/ISRC format                 │  │  │ │
│  │  │  │  • MusicalWork              │  │  • IPI/ISNI format                  │  │  │ │
│  │  │  │  • Recording                │  │                                      │  │  │ │
│  │  │  │  • Release                  │  └─────────────────────────────────────┘  │  │ │
│  │  │  │  • Artist                   │                                           │  │ │
│  │  │  │  • LegalEntity              │                                           │  │ │
│  │  │  │  • Timestamp                │                                           │  │ │
│  │  │  └─────────────────────────────┘                                           │  │ │
│  │  └─────────────────────────────────────────────────────────────────────────────┘  │ │
│  │                                                                                     │ │
│  │  ┌─────────────────────────────────────────────────────────────────────────────┐  │ │
│  │  │                    JS Interop (@allfeat/client SDK)                          │  │ │
│  │  │                                                                              │  │ │
│  │  │  ┌──────────────────────┐  ┌──────────────────────┐  ┌────────────────────┐ │  │ │
│  │  │  │    wallet.js         │  │   blockchain.js      │  │    balance.js      │ │  │ │
│  │  │  │  • connectWallet()   │  │  • submitWork()      │  │  • getBalance()    │ │  │ │
│  │  │  │  • getAccounts()     │  │  • submitBatch()     │  │                    │ │  │ │
│  │  │  └──────────────────────┘  └──────────────────────┘  └────────────────────┘ │  │ │
│  │  └─────────────────────────────────────────────────────────────────────────────┘  │ │
│  │                                                                                     │ │
│  └────────────────────────────────────────────────────────────────────────────────────┘ │
│                                                                                          │
└────────────────────────────────────────────────────────────────────────────────┬─────────┘
                                                                                 │
                                                HTTP + WebSocket                 │
                                                                                 │
┌────────────────────────────────────────────────────────────────────────────────▼─────────┐
│                                      SERVER                                               │
│                                                                                          │
│  ┌────────────────────────────────────────────────────────────────────────────────────┐ │
│  │                           Axum Backend (Rust)                                       │ │
│  │                                                                                     │ │
│  │  ┌──────────────────────────────────────────────────────────────────────────────┐ │ │
│  │  │                              API Routes                                       │ │ │
│  │  │                                                                               │ │ │
│  │  │  GET  /health                    → Health check                              │ │ │
│  │  │  GET  /api/v1/stats              → Dashboard stats                           │ │ │
│  │  │  POST /api/v1/upload             → Massload CSV upload                       │ │ │
│  │  │  GET  /api/v1/logs               → SSE real-time logs                        │ │ │
│  │  │  POST /api/v1/timestamp/create   → Create ATS hash                           │ │ │
│  │  │  POST /api/v1/timestamp/verify   → Verify ATS proof                          │ │ │
│  │  │  GET  /api/v1/templates          → List transformation templates             │ │ │
│  │  │  GET  /*                         → Serve frontend static files               │ │ │
│  │  │                                                                               │ │ │
│  │  └──────────────────────────────────────────────────────────────────────────────┘ │ │
│  │                                                                                     │ │
│  │  ┌─────────────────────────────────────────────────────────────────────────────┐  │ │
│  │  │                       allfeat-services (Backend Logic)                       │  │ │
│  │  │                                                                              │  │ │
│  │  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌─────────────────┐  │  │ │
│  │  │  │      AI      │  │    Parser    │  │   Transform  │  │      Cache      │  │  │ │
│  │  │  │   (Claude)   │  │    (CSV)     │  │    (DSL)     │  │   (Templates)   │  │  │ │
│  │  │  │              │  │              │  │              │  │                 │  │  │ │
│  │  │  │ • Generate   │  │ • Auto-enc   │  │ • Execute    │  │ • Store matrix  │  │  │ │
│  │  │  │   matrix     │  │ • Delimiter  │  │   matrix     │  │ • Match columns │  │  │ │
│  │  │  │              │  │ • Parse CSV  │  │ • Group by   │  │ • Cache hits    │  │  │ │
│  │  │  │              │  │              │  │   ISWC       │  │                 │  │  │ │
│  │  │  └──────────────┘  └──────────────┘  └──────────────┘  └─────────────────┘  │  │ │
│  │  │                                                                              │  │ │
│  │  │  ┌──────────────────────────────────────────────────────────────────────┐   │  │ │
│  │  │  │                        Timestamp Service                              │   │  │ │
│  │  │  │  • File hash (SHA256/BLAKE3)                                          │   │  │ │
│  │  │  │  • Proof generation                                                   │   │  │ │
│  │  │  │  • Verification                                                       │   │  │ │
│  │  │  └──────────────────────────────────────────────────────────────────────┘   │  │ │
│  │  └─────────────────────────────────────────────────────────────────────────────┘  │ │
│  │                                                                                     │ │
│  └────────────────────────────────────────────────────────────────────────────────────┘ │
│                                                                                          │
└──────────────────────────────────────────────────────────────────────────────────────────┘
                                           │
                                           │ WebSocket (RPC)
                                           │
┌──────────────────────────────────────────▼───────────────────────────────────────────────┐
│                                                                                          │
│                              ALLFEAT BLOCKCHAIN (Melodie)                                │
│                                                                                          │
│  ┌──────────────────────────────────────────────────────────────────────────────────┐  │
│  │                              Substrate Pallets                                    │  │
│  │                                                                                   │  │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌──────────────────────┐  │  │
│  │  │  pallet-     │  │  pallet-     │  │  pallet-     │  │  pallet-             │  │  │
│  │  │  musical-    │  │  recordings  │  │  releases    │  │  parties             │  │  │
│  │  │  works       │  │              │  │              │  │                      │  │  │
│  │  │              │  │              │  │              │  │  • Artists           │  │  │
│  │  │  • register  │  │  • register  │  │  • register  │  │  • Legal Entities    │  │  │
│  │  │  • update    │  │  • link      │  │  • link      │  │  • IPI/ISNI          │  │  │
│  │  │  • query     │  │  • query     │  │  • query     │  │                      │  │  │
│  │  └──────────────┘  └──────────────┘  └──────────────┘  └──────────────────────┘  │  │
│  │                                                                                   │  │
│  │  ┌──────────────────────────────────────────────────────────────────────────┐   │  │
│  │  │                          pallet-timestamps (ATS)                          │   │  │
│  │  │                                                                           │   │  │
│  │  │  • stamp(hash, metadata)  → Store timestamped proof on-chain             │   │  │
│  │  │  • verify(hash)           → Check if hash exists & get block info        │   │  │
│  │  │                                                                           │   │  │
│  │  └──────────────────────────────────────────────────────────────────────────┘   │  │
│  │                                                                                   │  │
│  │  ┌──────────────────────────────────────────────────────────────────────────┐   │  │
│  │  │                    Proof of Metadata (PoM) Consensus                      │   │  │
│  │  │                                                                           │   │  │
│  │  │  • Submit MIDDS with collateral                                          │   │  │
│  │  │  • Trusters vote & certify                                               │   │  │
│  │  │  • Challenge period                                                       │   │  │
│  │  │  • Rewards distribution                                                   │   │  │
│  │  └──────────────────────────────────────────────────────────────────────────┘   │  │
│  └──────────────────────────────────────────────────────────────────────────────────┘  │
│                                                                                          │
│                          Node: wss://dev.rpc.allfeat.org                                │
│                                                                                          │
└──────────────────────────────────────────────────────────────────────────────────────────┘
```

---

## 3. Workspace Cargo Structure

```
allfeat-apps/
│
├── Cargo.toml              ─────────────────────────────────────────────────────┐
│   [workspace]                                                                   │
│   members = [                                                                   │
│       "crates/core",                                                            │
│       "crates/ui",                                                              │
│       "crates/services",                                                        │
│       "apps/hub/backend",                                                       │
│       "apps/hub/frontend",                                                      │
│   ]                                                                             │
│                                                                                 │
├── crates/                                                                       │
│   │                                                                             │
│   ├── core/               ◄── Shared domain models + validation                │
│   │   ├── Cargo.toml           (used by frontend + backend)                    │
│   │   └── src/                                                                  │
│   │       ├── lib.rs                                                            │
│   │       ├── models/                                                           │
│   │       │   ├── mod.rs                                                        │
│   │       │   ├── party.rs         # PartyId, CreatorRole, Artist, LegalEntity │
│   │       │   ├── musical_work.rs  # MusicalWork, GroupedWork                  │
│   │       │   ├── recording.rs     # Recording, ISRC                           │
│   │       │   ├── release.rs       # Release, Album, Single                    │
│   │       │   └── timestamp.rs     # TimestampProof, ATS                       │
│   │       ├── validation/                                                       │
│   │       │   ├── mod.rs                                                        │
│   │       │   ├── schemas.rs       # Embedded JSON schemas                     │
│   │       │   └── validators.rs    # Validation functions                      │
│   │       └── error.rs             # Error types                               │
│   │                                                                             │
│   ├── ui/                 ◄── Shared Leptos components (WASM)                  │
│   │   ├── Cargo.toml                                                            │
│   │   └── src/                                                                  │
│   │       ├── lib.rs                                                            │
│   │       ├── components/                                                       │
│   │       │   ├── mod.rs                                                        │
│   │       │   ├── header.rs        # App header with navigation                │
│   │       │   ├── footer.rs        # App footer                                │
│   │       │   ├── wallet/          # Wallet connection components              │
│   │       │   │   ├── mod.rs                                                    │
│   │       │   │   ├── modal.rs     # Wallet selection modal                    │
│   │       │   │   ├── button.rs    # Connect/status button                     │
│   │       │   │   └── balance.rs   # Balance display                           │
│   │       │   ├── forms/           # Reusable form components                  │
│   │       │   │   ├── mod.rs                                                    │
│   │       │   │   ├── input.rs     # Text input                                │
│   │       │   │   ├── select.rs    # Dropdown select                           │
│   │       │   │   ├── file.rs      # File upload                               │
│   │       │   │   └── submit.rs    # Submit button                             │
│   │       │   ├── cards/           # Card layouts                              │
│   │       │   │   ├── mod.rs                                                    │
│   │       │   │   ├── feature.rs   # Feature card (home page)                  │
│   │       │   │   └── preview.rs   # Data preview card                         │
│   │       │   └── notifications/   # Toast system                              │
│   │       ├── hooks/               # Reactive utilities                        │
│   │       │   ├── mod.rs                                                        │
│   │       │   ├── use_wallet.rs    # Wallet state hook                         │
│   │       │   └── use_theme.rs     # Theme toggle hook                         │
│   │       ├── i18n/                # Internationalization                      │
│   │       │   ├── mod.rs                                                        │
│   │       │   ├── en.rs            # English translations                      │
│   │       │   └── fr.rs            # French translations                       │
│   │       └── theme/               # Theme system                              │
│   │           ├── mod.rs                                                        │
│   │           ├── dark.rs                                                       │
│   │           └── light.rs                                                      │
│   │                                                                             │
│   └── services/           ◄── Backend-only services                            │
│       ├── Cargo.toml                                                            │
│       └── src/                                                                  │
│           ├── lib.rs                                                            │
│           ├── ai/                  # AI transformation (Claude)                │
│           │   ├── mod.rs                                                        │
│           │   ├── client.rs        # Anthropic API client                      │
│           │   └── prompt.rs        # Matrix generation prompts                 │
│           ├── parser/              # CSV parsing                               │
│           │   ├── mod.rs                                                        │
│           │   ├── encoding.rs      # Auto-encoding detection                   │
│           │   └── csv.rs           # CSV to JSON                               │
│           ├── transform/           # DSL transformation                        │
│           │   ├── mod.rs                                                        │
│           │   ├── dsl.rs           # Transformation DSL                        │
│           │   ├── grouper.rs       # Group by ISWC                             │
│           │   └── pipeline.rs      # Full transformation pipeline             │
│           ├── cache/               # Template caching                          │
│           │   ├── mod.rs                                                        │
│           │   └── registry.rs      # Matrix template registry                  │
│           └── timestamp/           # ATS service                               │
│               ├── mod.rs                                                        │
│               ├── hash.rs          # File hashing (SHA256/BLAKE3)              │
│               └── proof.rs         # Proof generation                          │
│                                                                                 │
├── apps/                                                                         │
│   │                                                                             │
│   └── hub/                ◄── Main application                                 │
│       │                                                                         │
│       ├── backend/        ◄── Axum unified API server                          │
│       │   ├── Cargo.toml                                                        │
│       │   └── src/                                                              │
│       │       ├── main.rs          # Entry point                               │
│       │       ├── api/             # API handlers                              │
│       │       │   ├── mod.rs                                                    │
│       │       │   ├── upload.rs    # POST /api/v1/upload                       │
│       │       │   ├── logs.rs      # GET /api/v1/logs (SSE)                    │
│       │       │   ├── timestamp.rs # POST /api/v1/timestamp/*                  │
│       │       │   ├── templates.rs # GET /api/v1/templates                     │
│       │       │   └── stats.rs     # GET /api/v1/stats                         │
│       │       ├── state.rs         # App state                                 │
│       │       └── server.rs        # Server startup                            │
│       │                                                                         │
│       └── frontend/       ◄── Leptos CSR application                           │
│           ├── Cargo.toml                                                        │
│           ├── Trunk.toml                                                        │
│           ├── index.html                                                        │
│           ├── src/                                                              │
│           │   ├── lib.rs           # WASM entry point                          │
│           │   ├── app.rs           # Main app component                        │
│           │   ├── routes.rs        # Router configuration                      │
│           │   └── pages/           # Page components                           │
│           │       ├── mod.rs                                                    │
│           │       ├── home.rs              # / (Dashboard)                     │
│           │       ├── register/            # /register/*                       │
│           │       │   ├── mod.rs                                                │
│           │       │   ├── hub.rs           # /register                         │
│           │       │   ├── musical_work.rs  # /register/musical-work            │
│           │       │   ├── recording.rs     # /register/recording               │
│           │       │   ├── release.rs       # /register/release                 │
│           │       │   ├── artist.rs        # /register/artist                  │
│           │       │   └── legal_entity.rs  # /register/legal-entity            │
│           │       ├── protect/             # /protect/*                        │
│           │       │   ├── mod.rs                                                │
│           │       │   ├── hub.rs           # /protect                          │
│           │       │   ├── create.rs        # /protect/new                      │
│           │       │   └── verify.rs        # /protect/verify                   │
│           │       ├── massload/            # /massload                         │
│           │       │   ├── mod.rs                                                │
│           │       │   ├── upload.rs        # Upload page                       │
│           │       │   └── templates.rs     # Templates management              │
│           │       ├── explorer/            # /explorer                         │
│           │       │   ├── mod.rs                                                │
│           │       │   ├── search.rs        # Search page                       │
│           │       │   ├── work.rs          # Work detail                       │
│           │       │   └── recording.rs     # Recording detail                  │
│           │       ├── faucet.rs            # /faucet                           │
│           │       └── not_found.rs         # 404 page                          │
│           ├── style/                                                            │
│           │   └── main.css         # Global styles                             │
│           ├── js/                  # JavaScript interop                        │
│           │   ├── wallet.js        # Wallet connection                         │
│           │   └── blockchain.js    # @allfeat/client SDK                       │
│           └── public/              # Static assets                             │
│               ├── favicon.ico                                                   │
│               └── assets/                                                       │
│                   └── logo.svg                                                  │
│                                                                                 │
├── schemas/                ◄── JSON Schemas (source of truth)                   │
│   ├── midds-musical-work.json                                                   │
│   ├── midds-recording.json                                                      │
│   ├── midds-release.json                                                        │
│   ├── midds-artist.json                                                         │
│   ├── midds-legal-entity.json                                                   │
│   ├── timestamp-proof.json                                                      │
│   └── transformation-matrix.json                                                │
│                                                                                 │
├── docker/                                                                       │
│   ├── Dockerfile           # Multi-stage production build                      │
│   └── docker-compose.yml   # Local development                                 │
│                                                                                 │
└── .github/                                                                      │
    └── workflows/                                                                │
        ├── ci.yml           # Test + lint                                        │
        └── deploy.yml       # Build + push image                                 │
```

---

## 4. Comparaison avec Tanssi

| Aspect | Tanssi | Allfeat (Proposé) |
|--------|--------|-------------------|
| **URL** | apps.tanssi.network | apps.allfeat.org |
| **Framework** | React/Next.js | Leptos (Rust/WASM) |
| **Navigation** | Home, Stake, Bridge, Ecosystem, Build, Operate, Learn | Home, Register, Protect, Massload, Explorer, Faucet |
| **Wallet** | Polkadot.js, Talisman, SubWallet | Same |
| **Backend** | API (probablement Node.js) | Axum (Rust) |
| **SDK** | @polkadot/api | @allfeat/client |
| **Deployment** | Single SPA | Single SPA (unified server) |

---

## 5. Flow d'une Transaction

### Register Musical Work Flow

```
┌────────────────┐     ┌────────────────┐     ┌────────────────┐
│                │     │                │     │                │
│  1. User fills │────▶│  2. Validate   │────▶│  3. Sign tx    │
│     form       │     │     locally    │     │     (wallet)   │
│                │     │                │     │                │
└────────────────┘     └────────────────┘     └────────────────┘
        │                      │                      │
        ▼                      ▼                      ▼
┌────────────────┐     ┌────────────────┐     ┌────────────────┐
│  • ISWC        │     │  • JSON Schema │     │  • SubWallet   │
│  • Title       │     │  • ISWC format │     │  • Talisman    │
│  • Creators    │     │  • IPI/ISNI    │     │  • Polkadot.js │
│  • Role        │     │    format      │     │                │
└────────────────┘     └────────────────┘     └────────────────┘
                                                      │
                                                      ▼
                               ┌────────────────────────────────┐
                               │                                │
                               │  4. Submit to Blockchain       │
                               │     via @allfeat/client        │
                               │                                │
                               │  client.tx.musicalWorks        │
                               │       .register(work)          │
                               │       .signAndSend(...)        │
                               │                                │
                               └────────────────────────────────┘
                                              │
                                              ▼
                               ┌────────────────────────────────┐
                               │                                │
                               │  5. Melodie Blockchain         │
                               │                                │
                               │  • pallet-musical-works        │
                               │  • PoM consensus               │
                               │  • Finalization                │
                               │                                │
                               └────────────────────────────────┘
                                              │
                                              ▼
                               ┌────────────────────────────────┐
                               │                                │
                               │  6. Confirmation               │
                               │                                │
                               │  • Block hash                  │
                               │  • Success toast               │
                               │  • Update UI                   │
                               │                                │
                               └────────────────────────────────┘
```

---

*Architecture diagrams for Allfeat Apps Hub*

