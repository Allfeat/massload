# Allfeat Apps Hub - Backend

<div align="center">

**🔧 Unified server — AI-powered CSV transformation + static file serving**

[![Rust](https://img.shields.io/badge/rust-1.75+-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![Axum](https://img.shields.io/badge/axum-0.7-blue?style=flat-square)](https://github.com/tokio-rs/axum)
[![License](https://img.shields.io/badge/license-GPL--3.0-blue?style=flat-square)](../../../LICENSE)

[API](#api-endpoints) • [Algorithm](#algorithm) • [Running](#running) • [Configuration](#configuration)

</div>

---

## Overview

A **unified Axum server** that:
1. **Serves the frontend** (CSR static files from `../frontend/dist`)
2. **Provides REST API** for CSV transformation (`/api/upload`)
3. **Streams real-time logs** via Server-Sent Events (`/api/logs`)

### Architecture

```
┌─────────────────────────────────────────────────────────┐
│                 Allfeat Apps Hub                        │
│                 (Unified Server)                        │
├─────────────────────────────────────────────────────────┤
│  Backend (Axum)                                         │
│  • POST /api/upload  → CSV → MIDDS JSON                │
│  • GET  /api/logs    → SSE real-time logs              │
│  • GET  /health      → Health check                    │
│  • /*                → Serve frontend static files      │
├─────────────────────────────────────────────────────────┤
│  Frontend (Leptos CSR)                                 │
│  • Compiled to WASM + JS                               │
│  • Served from apps/hub/frontend/dist                  │
└─────────────────────────────────────────────────────────┘
```

**Key Benefits:**
- 🚀 **Single deployment** — One binary/container for everything
- 🔌 **No CORS** — Frontend and API share the same origin
- ⚡ **Stateless** — Horizontal scaling ready
- 📊 **Observable** — SSE logs for monitoring

## Algorithm

### CSV → MIDDS Transformation Pipeline

```
CSV File
  │
  ├─1. Parse (auto-detect encoding & delimiter)
  │
  ├─2. Try Template Cache
  │   ├─✅ Match found → Reuse transformation
  │   └─❌ No match → Generate new matrix
  │
  ├─3. AI Matrix Generation (Claude)
  │   ├─ Analyze CSV structure
  │   ├─ Generate transformation DSL
  │   └─ Save to cache
  │
  ├─4. Apply DSL Transformation
  │   ├─ Map columns to MIDDS fields
  │   ├─ Split creators (Composer, Lyricist, etc.)
  │   └─ Group by ISWC
  │
  ├─5. Validation (JSON Schema)
  │   ├─ Validate each work
  │   └─ Report errors
  │
  └─6. Return MIDDS JSON
```

### Template Caching

The backend caches successful transformation matrices:
- **Key**: CSV column hash
- **Value**: Transformation DSL
- **Storage**: Local JSON files (`~/.cache/massload/templates/`)

**Benefits**:
- ⚡ **Instant transformation** for known CSV formats
- 💰 **Save AI costs** by reusing matrices
- 📈 **Improve over time** as more formats are cached

## API Endpoints

### Health Check

```bash
GET /health
```

**Response:**
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

### Upload CSV

```bash
POST /api/upload
Content-Type: multipart/form-data

file: <csv_file>
```

**Response:**
```json
{
  "jobId": "uuid",
  "status": "ready",
  "musicalWorks": [
    {
      "iswc": "T1234567890",
      "title": "Song Title",
      "creationYear": 2024,
      "instrumental": false,
      "language": "English",
      "creators": [
        {"id": {"Ipi": 123456789}, "role": "Composer"}
      ],
      "workType": "Original"
    }
  ],
  "metadata": {
    "totalWorks": 1,
    "estimatedCost": "0.05 AFT",
    "matrixId": "uuid",
    "cached": true,
    "csvInfo": {
      "encoding": "utf-8",
      "delimiter": ",",
      "rowCount": 10,
      "columns": ["ISWC", "Title", "Composer"]
    },
    "validation": {
      "valid": 1,
      "invalid": 0,
      "errors": []
    }
  }
}
```

### Real-time Logs (SSE)

```bash
GET /api/logs
Accept: text/event-stream
```

**Stream:**
```
data: {"level":"info","message":"📄 Parsing CSV..."}

data: {"level":"success","message":"✅ Transformation complete!"}
```

### Static Files (SPA Fallback)

All other routes serve the frontend:

```bash
GET /                 → index.html
GET /massload         → index.html (SPA routing)
GET /explore          → index.html (SPA routing)
GET /assets/*.js      → static JS/WASM files
GET /assets/*.css     → static CSS files
```

## Running

### Prerequisites

- Rust 1.75+
- `ANTHROPIC_API_KEY` environment variable

### Development

```bash
# From workspace root
cargo run --bin allfeat-hub -- serve --port 3000
```

The server will:
1. Look for frontend in `apps/hub/frontend/dist`
2. Serve it on `http://localhost:3000`
3. Expose API on `/api/*`

### Production

```bash
# Build release binary
cargo build --release --bin allfeat-hub

# Run
./target/release/allfeat-hub serve --port 3000
```

## Configuration

### Environment Variables

| Variable | Description | Required |
|----------|-------------|----------|
| `ANTHROPIC_API_KEY` | Claude API key | Yes |
| `RUST_LOG` | Log level (info, debug, warn, error) | No |
| `PORT` | Server port (default: 3000) | No |

### Frontend Path Detection

The server looks for frontend files in multiple locations:
1. `apps/hub/frontend/dist` (workspace root)
2. `frontend/dist` (legacy path)
3. `../frontend/dist` (when run from backend dir)

## File Structure

```
backend/
├── src/
│   ├── api/
│   │   ├── server.rs    # Axum server + routes
│   │   ├── types.rs     # API types
│   │   └── logs.rs      # SSE log broadcaster
│   ├── lib.rs           # Public API
│   └── main.rs          # CLI entry point
└── Cargo.toml
```

## CLI Usage

The backend also provides a CLI for offline transformation:

```bash
# Transform a CSV file
allfeat-hub transform input.csv --output works.json

# List cached templates
allfeat-hub template list

# Show DSL operations
allfeat-hub operations
```

See `cargo run --bin allfeat-hub -- --help` for all commands.

## Docker

The backend is built as part of the unified Docker image:

```dockerfile
# Multi-stage build
FROM rust:1.75-slim as builder

# ... build frontend and backend ...

# Runtime
FROM debian:bookworm-slim
COPY --from=builder /app/apps/hub/backend/target/release/allfeat-hub .
COPY --from=builder /app/apps/hub/frontend/dist ./apps/hub/frontend/dist

CMD ["./allfeat-hub", "serve", "--port", "3000"]
```

## Dependencies

Key Rust crates:

| Crate | Purpose |
|-------|---------|
| `axum` | Web framework |
| `tower-http` | Middleware (CORS, static files) |
| `serde_json` | JSON handling |
| `tokio` | Async runtime |
| `allfeat-services` | Transformation logic (workspace crate) |
| `allfeat-core` | MIDDS validation (workspace crate) |

## Error Handling

The API returns structured errors:

```json
{
  "error": "Failed to parse CSV",
  "details": "Invalid encoding: expected UTF-8"
}
```

HTTP status codes:
- `200` — Success
- `400` — Bad request (invalid CSV, no file, etc.)
- `500` — Internal server error (AI failure, etc.)

---

<div align="center">

Part of [Allfeat Apps Hub](../../../README.md) • Built with ❤️ by [Allfeat](https://allfeat.org)

</div>
