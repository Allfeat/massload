# Allfeat Apps Hub - Frontend

<div align="center">

**WebAssembly UI for Allfeat blockchain applications**

[![Rust](https://img.shields.io/badge/rust-1.75+-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![Leptos](https://img.shields.io/badge/leptos-0.6-purple?style=flat-square)](https://leptos.dev/)
[![License](https://img.shields.io/badge/license-GPL--3.0-blue?style=flat-square)](../../../LICENSE)

[Features](#features) • [Architecture](#architecture) • [Components](#components) • [Running](#running)

</div>

---

## Overview

The frontend is a **Leptos WebAssembly** application served by the unified Axum backend. It provides multiple applications for interacting with the Allfeat blockchain:

- **📤 Massload** — Bulk CSV upload and work registration
- **🔍 Explorer** — Browse on-chain musical works, recordings, and releases
- **📝 Register** — Single work registration (coming soon)
- **🛡️ Protect** — IP protection and certification (coming soon)

## Architecture

### Client-Side Rendering (CSR)

The frontend is compiled to WebAssembly and runs entirely in the browser. The backend serves static files (HTML, JS, WASM, CSS) and provides API endpoints.

```
┌─────────────────────────────────────────────────────────────────────┐
│                         Browser (WASM)                              │
├─────────────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌─────────┐ │
│  │   Massload   │  │   Explorer   │  │   Register   │  │  Header │ │
│  │ CSV Upload   │  │ Browse Works │  │ Single Work  │  │  Wallet │ │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘  └────┬────┘ │
│         │                 │                 │               │       │
│         └─────────────────┴─────────┬───────┴───────────────┘       │
│                                     │                                │
│                            ┌────────▼────────┐                       │
│                            │  @allfeat/client│                       │
│                            │   (dedot SDK)   │                       │
│                            └────────┬────────┘                       │
└─────────────────────────────────────┼──────────────────────────────┘
                                      │
                             ┌────────▼────────┐
                             │  Allfeat Node   │
                             │ wss://node-dev  │
                             └─────────────────┘
```

### Unified Server Benefits

- ✅ **No CORS issues** — Frontend and API share the same origin
- ✅ **Relative paths** — API calls use `/api/upload`, no hardcoded URLs
- ✅ **Single deployment** — One container for everything
- ✅ **Environment-agnostic** — Works on any domain/port automatically

## Features

### Massload
- **🔄 Drag & Drop** — Upload CSV files with visual feedback
- **📊 Real-time Logs** — SSE-based live processing logs from backend
- **📋 Transaction Preview** — Review works before blockchain submission
- **📦 Batch Transactions** — Efficient multi-work registration via `batch`
- **✅ Finalization Tracking** — Wait for `BestChainBlockIncluded` or `Finalized`

### Explorer
- **🔍 Browse Works** — View registered musical works on-chain
- **🎵 Recordings** — Explore recordings with performer metadata
- **💿 Releases** — View album and single releases
- **📊 Metrics** — Real-time blockchain statistics (works, recordings, releases count)

### Common Features
- **👛 Wallet Integration** — SubWallet, Talisman, Polkadot.js support
- **💰 Balance Display** — Show AFT balance in header
- **🌐 Multi-language** — EN, FR, ES, DE, JP, KR, GR
- **🎨 Themes** — Dark/Light mode with system preference detection

## File Structure

```
frontend/
├── src/
│   ├── components/
│   │   ├── header.rs          # Logo + wallet + balance
│   │   ├── sidebar.rs         # Navigation menu
│   │   ├── upload.rs          # CSV upload (Massload)
│   │   ├── preview.rs         # Transaction preview
│   │   ├── work_item.rs       # Preview list items
│   │   ├── confirm_dialog.rs  # Confirmation dialog
│   │   ├── success_message.rs # Success feedback
│   │   ├── language_switcher.rs # Language selector
│   │   └── midds_display/     # MIDDS format display
│   │       ├── work.rs         # Musical work display
│   │       ├── recording.rs    # Recording display
│   │       └── release.rs      # Release display
│   ├── pages/
│   │   ├── home.rs            # Home page
│   │   ├── massload.rs        # Massload page
│   │   ├── explore.rs         # Explorer page
│   │   └── how_it_works.rs    # Protocol explanation
│   ├── services/
│   │   ├── upload.rs          # Backend API calls (relative paths)
│   │   ├── wallet.rs          # Wallet connection
│   │   ├── blockchain.rs      # SDK bridge
│   │   └── explorer.rs        # Blockchain data fetching
│   ├── i18n/                  # Translations (modular)
│   │   ├── mod.rs
│   │   ├── nav.rs
│   │   ├── massload.rs
│   │   ├── preview.rs
│   │   └── how_it_works.rs
│   ├── js/
│   │   ├── blockchain.js      # @allfeat/client integration
│   │   └── wallet.js          # Polkadot extension bridge
│   ├── config.rs              # Configuration constants
│   ├── types.rs               # Shared types
│   ├── midds.rs               # MIDDS type definitions
│   ├── validation.rs          # Client-side validation
│   └── lib.rs                 # App entry point
├── style/
│   ├── main.css               # Main styles (dark/light themes)
│   └── accordion.css          # Preview accordion
├── public/
│   └── favicon.ico            # App icon
├── index.html                 # HTML template
└── Trunk.toml                 # Trunk build config
```

## Configuration

### `src/config.rs`

All configuration uses runtime environment detection:

```rust
/// Blockchain RPC endpoint (default: devnet)
pub fn blockchain_rpc() -> String {
    "wss://dev.rpc.allfeat.org".to_string()
}

/// App name for wallet popups
pub const APP_NAME: &str = "Allfeat Apps Hub";
```

**No backend URL needed** — All API calls use relative paths (`/api/upload`).

## Running

### Prerequisites
- Rust 1.75+
- [Trunk](https://trunkrs.dev/) (`cargo install trunk`)
- A Polkadot-compatible wallet extension

### Development

```bash
# Start frontend with hot reload
cd apps/hub/frontend
trunk serve --port 8080

# The frontend will proxy API calls to the backend
# Make sure the backend is running on port 3000
```

### Production Build

```bash
cd apps/hub/frontend
trunk build --release
# Output in dist/
```

The backend will automatically serve these static files.

## Wallet Support

| Wallet | Status |
|--------|--------|
| [SubWallet](https://subwallet.app/) | ✅ Recommended |
| [Talisman](https://talisman.xyz/) | ✅ Supported |
| [Polkadot.js](https://polkadot.js.org/extension/) | ✅ Supported |

### Connection Flow

1. User clicks "Connect Wallet"
2. Extension popup requests permission
3. User approves for "Allfeat Apps Hub"
4. First account is selected
5. Balance is fetched from chain

### Transaction Signing

1. User clicks "Sign & Send"
2. SDK builds `batch` transaction (non-atomic for resilience)
3. Wallet popup shows transaction details
4. User confirms
5. Frontend tracks status until finalization

## Data Flow (Massload)

```
1. User drops CSV file
              │
              ▼
2. Frontend sends to /api/upload (relative path)
              │
              ▼
3. Backend transforms → MIDDS JSON
              │
              ▼
4. Frontend displays preview with pagination
              │
              ▼
5. User reviews and clicks "Sign & Send"
              │
              ▼
6. blockchain.js converts IPI to BigInt
              │
              ▼
7. SDK builds batch transaction (utility.batch)
              │
              ▼
8. Wallet signs transaction
              │
              ▼
9. SDK submits and tracks finalization
              │
              ▼
10. Frontend shows success message with tx hash
```

## Shared Crates

The frontend uses workspace crates for code reuse:

- **`allfeat-core`** — MIDDS types, validation schemas
- **`allfeat-ui`** — Reusable UI components (icons, buttons)
- **`allfeat-services`** — Business logic (parsing, transformation)

## Internationalization (i18n)

The app supports 7 languages with a modular translation system:

```rust
// Usage in components
use crate::i18n::t;

view! {
    <h1>{move || t("massload.title")}</h1>
}
```

Translation files are organized by feature (`nav.rs`, `massload.rs`, `preview.rs`, etc.).

## Styling

The app uses CSS variables for theming:

```css
/* Dark theme (default) */
:root {
  --color-primary: #1BA794;
  --bg-primary: #0A0A0A;
  --text-primary: #FFFFFF;
}

/* Light theme */
[data-theme="light"] {
  --bg-primary: #E8E8E8;
  --text-primary: #151515;
  --header-bg: #FFFFFF;
}
```

Theme switches automatically based on system preference or user selection.

---

<div align="center">

Part of [Allfeat Apps Hub](../../../README.md) • Built with ❤️ by [Allfeat](https://allfeat.org)

</div>
