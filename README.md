# Massload

<div align="center">

**Bulk registration of musical works on Allfeat blockchain**

[![Build Status](https://img.shields.io/github/actions/workflow/status/allfeat/massload/ci.yml?style=flat-square)](https://github.com/allfeat/massload/actions)
[![License](https://img.shields.io/badge/license-GPL--3.0-blue?style=flat-square)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75+-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)

[Features](#features) • [Quick Start](#quick-start) • [Architecture](#architecture) • [Microservice](#microservice) • [Contributing](#contributing)

</div>

---

## Overview

Massload transforms CSV files from music industry sources (SACEM, ASCAP, GEMA, JASRAC, PRS, SGAE) into MIDDS format and registers them on the Allfeat blockchain using the `@allfeat/client` SDK.

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│    CSV File     │────▶│   Microservice  │────▶│    Frontend     │────▶│    Melodie      │
│  (any format)   │     │   (Transform)   │     │  (Sign & Send)  │     │   Blockchain    │
└─────────────────┘     └─────────────────┘     └─────────────────┘     └─────────────────┘
```

## Microservice Architecture

The **backend is a standalone microservice** that can be deployed independently and consumed by any client:

```
                    ┌─────────────────────────────────────────┐
                    │        Massload Microservice            │
                    │         (Stateless REST API)            │
                    ├─────────────────────────────────────────┤
                    │  POST /api/upload → MIDDS JSON          │
                    │  GET  /api/logs   → SSE stream          │
                    │  GET  /health     → Health check        │
                    └─────────────────────────────────────────┘
                                      │
              ┌───────────────────────┼───────────────────────┐
              ▼                       ▼                       ▼
      ┌──────────────┐       ┌──────────────┐       ┌──────────────┐
      │  Leptos UI   │       │  Other App   │       │   CLI Tool   │
      │  (included)  │       │  (custom)    │       │   (curl)     │
      └──────────────┘       └──────────────┘       └──────────────┘
```

**Key microservice benefits:**
- 🐳 **Container-ready** — Deploy as Docker container
- 🔌 **API-first** — REST endpoints, no frontend coupling
- ⚡ **Stateless** — Horizontal scaling ready
- 📊 **Observable** — SSE logs for real-time monitoring

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

Create `backend/.env`:

```bash
ANTHROPIC_API_KEY=sk-ant-api03-...
```

### Running

```bash
# Terminal 1: Start backend
cd backend
cargo run --release -- serve

# Terminal 2: Start frontend
cd frontend
trunk serve --open
```

The app will open at `http://localhost:8080`.

### Usage

1. **Connect Wallet** — Click "Connect Wallet" and approve the connection
2. **Upload CSV** — Drag & drop your CSV file
3. **Review** — Check the transformed works in the preview
4. **Sign & Send** — Click to submit the batch transaction
5. **Confirm** — Approve in your wallet extension

## Documentation

| Document | Description |
|----------|-------------|
| [Backend README](backend/README.md) | API, CLI, transformation algorithm |
| [Frontend README](frontend/README.md) | Components, SDK integration, wallet support |

## Architecture

```
massload/
├── backend/                    # 🔧 Microservice (Rust)
│   ├── src/
│   │   ├── api/                # HTTP server + SSE logs
│   │   ├── ai/                 # Claude AI integration
│   │   ├── parser/             # CSV auto-parsing
│   │   ├── transform/          # DSL + grouper + pipeline
│   │   ├── validation/         # JSON Schema validators
│   │   └── cache/              # Template registry
│   └── schemas/                # MIDDS JSON schemas
│
├── frontend/                   # 🖥️ Leptos WASM UI
│   ├── src/
│   │   ├── components/         # UI components
│   │   ├── services/           # Backend + blockchain
│   │   └── js/                 # @allfeat/client bindings
│   └── style/                  # CSS
│
└── Cargo.toml                  # Workspace
```

### Data Flow

```
                              Backend                                Frontend
┌──────────────────────────────────────────────────────┐   ┌─────────────────────────────┐
│                                                      │   │                             │
│  CSV → Parse → Template/AI → DSL → Validate → Group  │──▶│  Preview → Sign → Submit    │
│                                                      │   │                             │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐              │   │  ┌─────────┐  ┌──────────┐  │
│  │  Cache  │  │ Claude  │  │ Schema  │              │   │  │ Wallet  │  │   SDK    │  │
│  └─────────┘  └─────────┘  └─────────┘              │   │  └─────────┘  └──────────┘  │
│                                                      │   │                             │
└──────────────────────────────────────────────────────┘   └─────────────────────────────┘
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
massload serve              # Start HTTP server
massload transform <csv>    # Transform CSV file
massload template list      # List cached templates
massload operations         # Show DSL operations
```

## Configuration

### Backend Environment

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
