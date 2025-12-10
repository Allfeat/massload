# Massload

<div align="center">

**Bulk registration of musical works on Allfeat blockchain**

[![Build Status](https://img.shields.io/github/actions/workflow/status/allfeat/massload/ci.yml?style=flat-square)](https://github.com/allfeat/massload/actions)
[![License](https://img.shields.io/badge/license-GPL--3.0-blue?style=flat-square)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75+-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)

[Features](#features) • [Quick Start](#quick-start) • [Architecture](#architecture) • [Docker](#docker) • [Contributing](#contributing)

</div>

---

## Overview

Massload transforms CSV files from music industry sources (SACEM, ASCAP, GEMA, JASRAC, PRS, SGAE) into MIDDS format and registers them on the Allfeat blockchain using the `@allfeat/client` SDK.

```
┌─────────────────┐     ┌───────────────────────────────────┐     ┌─────────────────┐
│    CSV File     │────▶│   Massload Unified Server         │────▶│    Melodie      │
│  (any format)   │     │   (Axum + Leptos CSR)             │     │   Blockchain    │
│                 │     │ • AI Transform   • Web UI         │     │                 │
└─────────────────┘     └───────────────────────────────────┘     └─────────────────┘
```

## Architecture

**Unified Server** — Single deployable application combining backend and frontend:

```
┌─────────────────────────────────────────────────────────┐
│              Massload Unified Server                    │
│                 (Single Container)                      │
├─────────────────────────────────────────────────────────┤
│  Backend (Axum)                                         │
│  • POST /api/upload  → CSV → MIDDS JSON                │
│  • GET  /api/logs    → SSE real-time logs              │
│  • GET  /health      → Health check                    │
│  • Serves static frontend assets                       │
├─────────────────────────────────────────────────────────┤
│  Frontend (Leptos CSR - WASM)                          │
│  • Drag & drop CSV upload                              │
│  • Preview & validation                                │
│  • Wallet integration (@allfeat/client)                │
│  • Sign & submit to blockchain                         │
└─────────────────────────────────────────────────────────┘
```

**Key benefits:**
- 🐳 **Single Container** — One image for frontend + backend
- 🔌 **API Available** — REST endpoints accessible independently
- ⚡ **Stateless** — Horizontal scaling ready
- 📊 **Observable** — SSE logs for real-time monitoring
- 🚀 **Simple Deployment** — No orchestration needed

## Features

- **🔄 Smart Transformation** — AI-powered CSV to MIDDS conversion with template caching
- **🎯 Auto-Detection** — Encoding (UTF-8, ISO-8859-1) and delimiter detection
- **📋 Template Reuse** — Cache successful transformations for similar files
- **✅ Validation** — JSON Schema validation at every step
- **🔗 SDK Integration** — Direct `@allfeat/client` usage for blockchain submission
- **👛 Wallet Support** — SubWallet, Talisman, Polkadot.js extensions
- **📦 Batch Transactions** — Efficient multi-work registration via `batchAll`
- **📊 Real-time Logs** — SSE-based processing status updates

## Quick Start

### Prerequisites

- Rust 1.75+
- [Trunk](https://trunkrs.dev/) (`cargo install trunk`)
- A Polkadot-compatible wallet extension
- Anthropic API key (for AI transformation)

### Installation

```bash
# Clone the repository
git clone https://github.com/allfeat/massload.git
cd massload

# Build
cargo build --release
```

### Configuration

Copy and configure environment variables:

```bash
cp .env.example .env
# Then edit .env with your real values
```

Required variables in `.env`:
- `ANTHROPIC_API_KEY` : Claude API key (required)
- `RUST_LOG` : Log level (info, debug, warn, error)

### Running

**Option 1: Using scripts (recommended)**

```bash
# Build everything
./build.sh

# Start the unified server
./start.sh
```

**Option 2: Manual**

```bash
# Build frontend
cd frontend && trunk build --release && cd ..

# Build and run backend
cargo build --release --bin massload
./target/release/massload serve --port 3000
```

The app will open at `http://localhost:3000`.

### Usage

1. **Connect Wallet** — Click "Connect Wallet" and approve the connection
2. **Upload CSV** — Drag & drop your CSV file
3. **Review** — Check the transformed works in the preview
4. **Sign & Send** — Click to submit the batch transaction
5. **Confirm** — Approve in your wallet extension

## Project Structure

```
massload/
├── backend/                    # 🔧 Backend Logic (Axum)
│   ├── src/
│   │   ├── api/                # HTTP server + SSE logs
│   │   ├── ai/                 # Claude AI integration
│   │   ├── parser/             # CSV auto-parsing
│   │   ├── transform/          # DSL + grouper + pipeline
│   │   ├── validation/         # JSON Schema validators
│   │   └── cache/              # Template registry
│   └── schemas/                # MIDDS JSON schemas
│
├── frontend/                   # 🖥️ Frontend UI (Leptos CSR)
│   ├── src/
│   │   ├── components/         # UI components
│   │   ├── services/           # Wallet + blockchain
│   │   └── js/                 # @allfeat/client bindings
│   ├── style/                  # CSS styles
│   ├── public/                 # Static assets (favicon, etc.)
│   └── index.html              # Entry point
│
├── Dockerfile                  # 🐳 Multi-stage Docker build
├── docker-compose.yml          # Local testing
├── build.sh                    # Build frontend + backend
├── start.sh                    # Start unified server
├── DOCKER.md                   # K8s integration guide
└── Cargo.toml                  # Workspace
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
                                      Allfeat Blockchain (Melodie)
```

## API Reference

### Backend

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/health` | GET | Health check |
| `/api/upload` | POST | Upload CSV file |
| `/api/logs` | GET | SSE log stream |

### CLI

```bash
massload serve              # Start unified HTTP server
massload transform <csv>    # Transform CSV file (CLI only)
massload template list      # List cached templates
massload operations         # Show DSL operations
```

## Docker

### Local Development

```bash
# Build and run with docker-compose
docker-compose up

# Or manually
docker build -t massload:local .
docker run -p 3000:3000 --env-file .env massload:local
```

### Production Deployment

The project follows a **GitOps** workflow:
- Docker images are built automatically on push to `develop`/`main`
- Kubernetes manifests are managed in the `allfeat/infra-kube` repository
- ArgoCD/Flux handles deployment to the cluster

See [DOCKER.md](DOCKER.md) for detailed K8s integration guide.

## Configuration

### Environment Variables

| Variable | Description | Required |
|----------|-------------|----------|
| `ANTHROPIC_API_KEY` | Claude API key | Yes |
| `RUST_LOG` | Log level | No |

### Frontend Environment (Runtime)

| Variable | Default | Description |
|----------|---------|-------------|
| `BACKEND_URL` | `http://localhost:3000` | Backend API |
| `BLOCKCHAIN_RPC` | `wss://node-dev.allfeat.io` | Melodie node |

> 💡 Frontend config is **runtime** - pass via `docker run -e` or K8s ConfigMap.

## Contributing

Contributions are welcome! Please read our contributing guidelines before submitting a PR.

```bash
# Run tests
cargo test

# Run linter
cargo clippy --all -- -D warnings

# Format code
cargo fmt
```

## License

Massload is licensed under the [GNU General Public License v3.0](LICENSE).

---

<div align="center">

**[Website](https://allfeat.org)** • **[Discord](https://discord.allfeat.com)** • **[Twitter](https://twitter.com/allfeat_IP)**

Built with ❤️ by [Allfeat](https://github.com/allfeat)

</div>
