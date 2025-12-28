# Massload Frontend

<div align="center">

**WebAssembly UI for bulk musical works registration**

[![Rust](https://img.shields.io/badge/rust-1.75+-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![Leptos](https://img.shields.io/badge/leptos-0.6-purple?style=flat-square)](https://leptos.dev/)
[![License](https://img.shields.io/badge/license-GPL--3.0-blue?style=flat-square)](../LICENSE)

[Features](#features) • [Architecture](#architecture) • [Components](#components) • [Configuration](#configuration)

</div>

---

## Overview

The frontend is a Leptos WebAssembly application that provides a user interface for uploading CSV files and registering musical works on the Allfeat blockchain via the `@allfeat/client` SDK.

```
┌─────────────────────────────────────────────────────────────────────┐
│                         Browser (WASM)                              │
├─────────────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌────────────┐ │
│  │   Upload    │  │    Logs     │  │   Preview   │  │   Header   │ │
│  │  Drag&Drop  │  │    SSE      │  │   Works     │  │   Wallet   │ │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘  └──────┬─────┘ │
│         │                │                │                │       │
│         └────────────────┴────────┬───────┴────────────────┘       │
│                                   │                                 │
│                          ┌────────▼────────┐                        │
│                          │  BlockchainJS   │                        │
│                          │ @allfeat/client │                        │
│                          └────────┬────────┘                        │
└───────────────────────────────────┼─────────────────────────────────┘
                                    │
                           ┌────────▼────────┐
                           │  Melodie Node   │
                           │   (WebSocket)   │
                           └─────────────────┘
```

## Features

| Feature | Description |
|---------|-------------|
| **Drag & Drop** | Upload CSV files with visual feedback |
| **Real-time Logs** | SSE-based live processing logs from backend |
| **Transaction Preview** | Review works before blockchain submission |
| **Wallet Integration** | SubWallet, Talisman, Polkadot.js support |
| **Balance Display** | Show MEL balance in header |
| **Batch Transactions** | Efficient multi-work registration via `batchAll` |
| **Finalization Tracking** | Wait for `BestChainBlockIncluded` or `Finalized` |

## Architecture

### Rust/WASM Layer
- **Leptos**: Reactive UI framework
- **Services**: Backend communication, wallet integration
- **Components**: Modular UI components

### JavaScript Bridge
- **blockchain.js**: `@allfeat/client` SDK integration
- **wallet.js**: Polkadot wallet extension bridge

### SDK Integration

The frontend uses `@allfeat/client` (based on dedot) for blockchain interactions:

```javascript
// Connection
const provider = new AllfeatProvider('wss://node-dev.allfeat.io');
const client = await MelodieClient.create(provider);

// Transaction
const calls = works.map(work => client.tx.musicalWorks.register(work).call);
const batch = client.tx.utility.batchAll(calls);

// Sign & Send (with wallet signer)
batch.signAndSend(address, { signer }, (result) => {
    if (result.status?.type === 'Finalized') {
        // Success!
    }
});
```

## Components

### Header
- Allfeat logo
- MEL balance display (fetched on wallet connect)
- Wallet connection status

### Upload Section
- Drag & drop zone
- File type validation
- Backend upload trigger

### Logs Panel
- Real-time SSE logs
- Color-coded by level (info, success, warning, error)
- Auto-scroll

### Preview Section
- Expandable work list
- Creator details
- Sign & Send button
- Cancel option

## File Structure

```
frontend/
├── src/
│   ├── components/
│   │   ├── header.rs      # Logo + wallet + balance
│   │   ├── hero.rs        # Title section
│   │   ├── upload.rs      # Drag & drop upload
│   │   ├── logs.rs        # SSE log display
│   │   ├── preview.rs     # Transaction preview
│   │   └── footer.rs      # Footer
│   ├── services/
│   │   ├── upload.rs      # Backend API calls
│   │   ├── wallet.rs      # Wallet connection
│   │   └── blockchain.rs  # SDK bridge
│   ├── js/
│   │   ├── blockchain.js  # @allfeat/client integration
│   │   └── wallet.js      # Polkadot extension bridge
│   ├── config.rs          # Configuration constants
│   ├── types.rs           # Shared types
│   └── lib.rs             # App entry point
├── style/
│   ├── main.css           # Main styles
│   └── accordion.css      # Preview accordion
├── index.html             # HTML template
└── Trunk.toml             # Trunk build config
```

## Configuration

Edit `src/config.rs`:

```rust
/// Backend API URL
pub const BACKEND_URL: &str = "http://localhost:3000";

/// Blockchain RPC endpoint
pub const BLOCKCHAIN_RPC: &str = "wss://node-dev.allfeat.io";

/// App name for wallet popups
pub const APP_NAME: &str = "Mass Load";
```

## Running

### Prerequisites
- Rust 1.75+
- [Trunk](https://trunkrs.dev/) (`cargo install trunk`)
- A Polkadot-compatible wallet extension

### Development

```bash
# Start frontend (hot reload)
trunk serve --port 8080

# Or with auto-open browser
trunk serve --open
```

### Production Build

```bash
trunk build --release
# Output in dist/
```

## Wallet Support

The frontend supports any Polkadot.js-compatible wallet:

| Wallet | Status |
|--------|--------|
| [SubWallet](https://subwallet.app/) | ✅ Recommended |
| [Talisman](https://talisman.xyz/) | ✅ Supported |
| [Polkadot.js](https://polkadot.js.org/extension/) | ✅ Supported |

### Connection Flow

1. User clicks "Connect Wallet"
2. Extension popup requests permission
3. User approves for "Mass Load"
4. First account is selected
5. Balance is fetched from chain

### Transaction Signing

1. User clicks "Sign & Send"
2. SDK builds `batchAll` transaction
3. Wallet popup shows transaction details
4. User confirms
5. Frontend tracks status until finalization

## Data Flow

```
1. User drops CSV file
              │
              ▼
2. Frontend sends to backend (/api/upload)
              │
              ▼
3. Backend transforms → MIDDS JSON
              │
              ▼
4. Frontend displays preview
              │
              ▼
5. User clicks "Sign & Send"
              │
              ▼
6. blockchain.js converts IPI to BigInt
              │
              ▼
7. SDK builds batchAll transaction
              │
              ▼
8. Wallet signs transaction
              │
              ▼
9. SDK submits and tracks finalization
              │
              ▼
10. Frontend shows success/error
```

---

<div align="center">

Part of [Massload](../README.md) • Built with ❤️ by [Allfeat](https://allfeat.org)

</div>
