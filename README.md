# Allfeat Apps Hub

<div align="center">

**Decentralized music metadata management on Allfeat blockchain**

[![Build Status](https://img.shields.io/github/actions/workflow/status/allfeat/massload/deploy-dev.yml?style=flat-square)](https://github.com/allfeat/massload/actions)
[![License](https://img.shields.io/badge/license-GPL--3.0-blue?style=flat-square)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75+-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)

[Features](#features) • [Quick Start](#quick-start) • [Architecture](#architecture) • [Deployment](#deployment) • [Contributing](#contributing)

</div>

---

## Overview

Allfeat Apps Hub is a unified web application for interacting with the Allfeat blockchain. It includes:

- **📤 Massload** — Bulk CSV upload and registration of musical works
- **🔍 Explorer** — Browse on-chain musical works, recordings, and releases
- **📝 Register** — Single work registration (coming soon)
- **🛡️ Protect** — IP protection and certification (coming soon)

```
┌─────────────────┐     ┌───────────────────────────────────┐     ┌─────────────────┐
│    CSV File     │────▶│   Allfeat Apps Hub                │────▶│    Allfeat      │
│  (any format)   │     │   Unified Server (Axum + Leptos)  │     │   Blockchain    │
│                 │     │ • AI Transform   • Web UI         │     │   (Devnet)      │
└─────────────────┘     └───────────────────────────────────┘     └─────────────────┘
```

## Architecture

**Unified Server** — Single deployable application combining backend and frontend:

```
┌─────────────────────────────────────────────────────────┐
│              Allfeat Apps Hub                           │
│                 (Single Container)                      │
├─────────────────────────────────────────────────────────┤
│  Backend (Axum) - apps/hub/backend                     │
│  • POST /api/upload  → CSV → MIDDS JSON                │
│  • GET  /api/logs    → SSE real-time logs              │
│  • GET  /health      → Health check                    │
│  • Serves static frontend assets                       │
├─────────────────────────────────────────────────────────┤
│  Frontend (Leptos CSR - WASM) - apps/hub/frontend     │
│  • Drag & drop CSV upload (Massload)                   │
│  • Blockchain explorer (Works, Recordings, Releases)   │
│  • Wallet integration (@allfeat/client)                │
│  • Sign & submit to blockchain                         │
├─────────────────────────────────────────────────────────┤
│  Shared Crates                                         │
│  • allfeat-core     → MIDDS types & validation         │
│  • allfeat-ui       → Reusable UI components           │
│  • allfeat-services → Business logic & blockchain SDK  │
└─────────────────────────────────────────────────────────┘
```

**Key benefits:**
- 🐳 **Single Container** — One image for frontend + backend
- 🚀 **Unified Server** — No CORS issues, relative paths
- ⚡ **Stateless** — Horizontal scaling ready
- 📊 **Observable** — SSE logs for real-time monitoring
- 🔄 **Monorepo** — Shared crates for code reuse

## Features

### Massload
- **🔄 Smart Transformation** — AI-powered CSV to MIDDS conversion with template caching
- **🎯 Auto-Detection** — Encoding (UTF-8, ISO-8859-1) and delimiter detection
- **📋 Template Reuse** — Cache successful transformations for similar files
- **✅ Validation** — JSON Schema validation at every step
- **📦 Batch Transactions** — Efficient multi-work registration via `batch`

### Explorer
- **🔍 Browse Works** — View registered musical works on-chain
- **🎵 Recordings** — Explore recordings with performer metadata
- **💿 Releases** — View album and single releases
- **📊 Metrics** — Real-time blockchain statistics

### Common
- **🔗 SDK Integration** — Direct `@allfeat/client` usage for blockchain submission
- **👛 Wallet Support** — SubWallet, Talisman, Polkadot.js extensions
- **🌐 i18n** — Multi-language support (EN, FR, ES, DE, JP, KR, GR)
- **🎨 Themes** — Dark/Light mode

## Quick Start

### Prerequisites

- Rust 1.75+
- [Trunk](https://trunkrs.dev/) (`cargo install trunk`)
- A Polkadot-compatible wallet extension
- Anthropic API key (for AI transformation in Massload)

### Installation

```bash
# Clone the repository
git clone https://github.com/allfeat/massload.git
cd massload

# Install dependencies
cargo build --release
```

### Configuration

Create a `.env` file at the project root:

```bash
# Required for Massload AI transformation
ANTHROPIC_API_KEY=your_claude_api_key_here

# Optional - Logging
RUST_LOG=info
```

### Running Locally

**Option 1: Using Docker Compose (recommended)**

```bash
docker-compose up
```

**Option 2: Manual Build**

```bash
# Build frontend
cd apps/hub/frontend
trunk build --release
cd ../../..

# Build and run backend (serves frontend + API)
cargo run --bin allfeat-hub --release -- serve --port 3000
```

The app will open at `http://localhost:3000`.

### Usage

1. **Connect Wallet** — Click "Connect Wallet" and approve the connection
2. **Massload**: Upload CSV → Review → Sign & Send
3. **Explorer**: Browse on-chain works, recordings, and releases

## Project Structure

```
Allfeat_ecosystem/
├── apps/
│   └── hub/
│       ├── backend/              # 🔧 Backend Logic (Axum)
│       │   ├── src/
│       │   │   ├── api/          # HTTP server + routes
│       │   │   └── main.rs       # Entry point
│       │   └── Cargo.toml
│       │
│       └── frontend/             # 🖥️ Frontend UI (Leptos CSR)
│           ├── src/
│           │   ├── components/   # UI components
│           │   ├── pages/        # Route pages (Massload, Explorer)
│           │   ├── services/     # Wallet + blockchain
│           │   ├── i18n/         # Translations
│           │   └── js/           # @allfeat/client bindings
│           ├── style/            # CSS styles
│           ├── public/           # Static assets
│           └── index.html
│
├── crates/
│   ├── core/                     # MIDDS types, validation
│   ├── ui/                       # Reusable UI components
│   └── services/                 # Business logic, AI, parsers
│
├── schemas/                      # MIDDS JSON schemas
├── Dockerfile                    # 🐳 Multi-stage Docker build
├── docker-compose.yml            # Local testing
└── Cargo.toml                    # Workspace root
```

### Data Flow

```
                    Unified Server (Port 3000)
┌──────────────────────────────────────────────────────────────────┐
│                                                                  │
│  Backend (Axum)                      Frontend (Leptos CSR/WASM)  │
│  ┌────────────────────────────┐     ┌──────────────────────────┐│
│  │ CSV → Parse → AI → DSL     │────▶│ Preview → Sign → Submit  ││
│  │                            │     │                          ││
│  │ ┌──────┐ ┌───────┐ ┌────┐ │     │ ┌──────┐ ┌────────────┐ ││
│  │ │Cache │ │Claude │ │JSON│ │     │ │Wallet│ │@allfeat/SDK│ ││
│  │ └──────┘ └───────┘ └────┘ │     │ └──────┘ └────────────┘ ││
│  │                            │     │            ↓            ││
│  └────────────────────────────┘     └──────────────────────────┘│
│                                                  ↓               │
└──────────────────────────────────────────────────────────────────┘
                                                  ↓
                               Allfeat Blockchain (Devnet)
                            wss://dev.rpc.allfeat.org
```

## API Reference

### Backend Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/health` | GET | Health check |
| `/api/upload` | POST | Upload CSV file (multipart/form-data) |
| `/api/logs` | GET | SSE log stream |
| `/*` | GET | Serve frontend static files (SPA fallback) |

### Frontend Configuration (Runtime)

The frontend uses the following defaults (configurable via `src/config.rs`):

| Variable | Default | Description |
|----------|---------|-------------|
| `BLOCKCHAIN_RPC` | `wss://dev.rpc.allfeat.org` | Allfeat blockchain RPC endpoint |

All API calls use **relative paths** (`/api/upload`), so no backend URL configuration is needed.

## Deployment

### Docker

**Local Build:**

```bash
docker build -t allfeat-hub:local .
docker run -p 3000:3000 --env-file .env allfeat-hub:local
```

**Docker Compose:**

```bash
docker-compose up
```

### Production (Kubernetes)

The project follows a **GitOps** workflow:

1. Push to `develop` branch triggers GitHub Actions
2. Docker image is built and pushed to `ghcr.io/allfeat/massload:sha-*`
3. GitHub Actions updates `allfeat/infra-kube` repository with new image tag
4. Kubernetes cluster pulls and deploys the new image

**Environments:**
- **Dev**: `https://massload-dev.allfeat.org` (branch: `develop`)
- **Prod**: `https://protect.allfeat.org` (branch: `main`)

See `.github/workflows/deploy-dev.yml` for deployment workflow.

### Health Check

```bash
curl http://localhost:3000/health
```

Expected response:
```json
{
  "status": "ok",
  "service": "allfeat-hub",
  "version": "0.2.0",
  "endpoints": {
    "upload": "POST /api/upload",
    "logs": "GET /api/logs (SSE)"
  }
}
```

## Development

### Building

```bash
# Build all crates
cargo build --release

# Build only frontend
cd apps/hub/frontend && trunk build --release

# Build only backend
cargo build --release --bin allfeat-hub
```

### Testing

```bash
# Run all tests
cargo test

# Run specific crate tests
cargo test -p allfeat-core
cargo test -p allfeat-services
```

### Linting

```bash
# Run clippy
cargo clippy --all -- -D warnings

# Format code
cargo fmt --all
```

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/my-feature`)
3. Commit your changes (`git commit -m 'Add my feature'`)
4. Push to the branch (`git push origin feature/my-feature`)
5. Open a Pull Request

**Code is law. Les bugs sont l'ennemi commun.**

## Documentation

- **[How it Works](apps/hub/frontend/src/pages/how_it_works.rs)** — Explanation of Proof of Metadata (PoM)
- **[White Paper](https://docsend.com/view/2w37ijcv57qvvbjw)** — Full protocol documentation
- **[Allfeat Docs](https://docs.allfeat.org)** — Official documentation
- **[MIDDS Specification](https://docs.allfeat.org/learn/metadata/)** — Music Industry Decentralized Data Structures

## License

Allfeat Apps Hub is licensed under the [GNU General Public License v3.0](LICENSE).

---

<div align="center">

**[Website](https://allfeat.org)** • **[Discord](https://discord.allfeat.com)** • **[Twitter](https://twitter.com/allfeat_IP)**

Built with ❤️ by [Allfeat](https://github.com/allfeat)

</div>
