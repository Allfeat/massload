# Massload Backend

<div align="center">

**🔧 Microservice — AI-powered CSV to MIDDS transformation**

[![Rust](https://img.shields.io/badge/rust-1.75+-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-GPL--3.0-blue?style=flat-square)](../LICENSE)
[![Docker](https://img.shields.io/badge/docker-ready-2496ED?style=flat-square&logo=docker)](Dockerfile)

[API](#api-endpoints) • [Algorithm](#algorithm) • [CLI](#cli-usage) • [Docker](#docker) • [Configuration](#configuration)

</div>

---

## Overview

A **standalone REST microservice** that transforms CSV files from various music industry sources (SACEM, ASCAP, GEMA, JASRAC, PRS, SGAE) into MIDDS format compatible with the `@allfeat/client` SDK.

### Microservice Features

- ⚡ **Stateless** — No session, horizontal scaling ready
- 🔌 **API-first** — REST + SSE, consumed by any client
- 🐳 **Container-ready** — Single binary, minimal dependencies
- 📊 **Observable** — Real-time SSE logs for monitoring

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│    CSV File     │────▶│    Massload     │────▶│   MIDDS JSON    │
│  (any format)   │     │    Backend      │     │  (SDK-ready)    │
└─────────────────┘     └────────┬────────┘     └─────────────────┘
                                 │
                    ┌────────────┴────────────┐
                    │                         │
               ┌────▼────┐              ┌─────▼─────┐
               │ Template│              │  Claude   │
               │  Cache  │              │    AI     │
               └─────────┘              └───────────┘
```

## Algorithm

The transformation pipeline uses a smart fallback strategy:

### Step 1: CSV Parsing
- Auto-detect encoding (UTF-8, ISO-8859-1, Windows-1252)
- Auto-detect delimiter (`,` `;` `|` `\t`)
- Extract headers and records

### Step 2: Template Matching
```
For each cached template (sorted by success rate):
    1. Check column compatibility
    2. Execute transformation
    3. Validate results
    4. If valid → Use this template
    5. If invalid → Try next template
```

### Step 3: AI Fallback
If all cached templates fail:
1. Send preview rows + unique values to Claude
2. Claude analyzes the data structure
3. Generates a transformation matrix (DSL)
4. Matrix is cached for future use

### Step 4: Transformation DSL
Available operations:
| Operation | Description | Example |
|-----------|-------------|---------|
| `copy` | Direct copy | `"Title" → title` |
| `normalize` | Clean ISWC format | `T-123.456.789-0 → T1234567890` |
| `map` | Value mapping | `CA → Composer` |
| `concat` | Merge fields | `First + Last → fullName` |
| `split` | Split field | `"A, B" → [A, B]` |
| `constant` | Fixed value | `→ "Original"` |

### Step 5: Validation
- **Flat validation**: Each record against MIDDS schema
- **Grouped validation**: Final SDK format before output

### Step 6: Grouping
```
Flat rows (one per creator)     →    Grouped works (SDK format)
┌─────────────────────────────┐     ┌──────────────────────────────┐
│ ISWC: T123, Creator: Alice  │     │ ISWC: T123                   │
│ ISWC: T123, Creator: Bob    │ ──▶ │ Creators: [Alice, Bob]       │
│ ISWC: T456, Creator: Carol  │     │ Participants: []             │
└─────────────────────────────┘     ├──────────────────────────────┤
                                    │ ISWC: T456                   │
                                    │ Creators: [Carol]            │
                                    └──────────────────────────────┘
```

## Output Format

The output is directly compatible with `@allfeat/client` SDK (dedot):

```json
{
  "iswc": "T1234567890",
  "title": "My Song",
  "creationYear": 2024,
  "creators": [
    { "id": { "type": "Ipi", "value": 123456789 }, "role": "Composer" },
    { "id": { "type": "Both", "value": { "ipi": 987654321, "isni": "0000000412345678" }}, "role": "Author" }
  ],
  "participants": []
}
```

> **Note**: Optional fields are omitted when null (SDK requirement). The `participants` field is required by the Melodie runtime.

## API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/health` | Health check |
| `POST` | `/api/upload` | Upload CSV for transformation |
| `GET` | `/api/logs` | SSE stream for real-time logs |

### POST /api/upload

Upload a CSV file and receive transformed MIDDS JSON.

**Request:**
```bash
curl -X POST http://localhost:3000/api/upload \
  -F "file=@catalog.csv"
```

**Response:**
```json
{
  "status": "success",
  "jobId": "abc123",
  "musicalWorks": [...],
  "metadata": {
    "totalWorks": 150,
    "estimatedCost": "7.50 AFT",
    "cached": true,
    "matrixId": "template-123"
  }
}
```

### GET /api/logs

Server-Sent Events stream for real-time processing logs.

```javascript
const events = new EventSource('/api/logs');
events.onmessage = (e) => console.log(JSON.parse(e.data));
```

## CLI Usage

```bash
# Start HTTP server
massload serve --port 3000

# Transform a CSV file
massload transform input.csv --output output.json --grouped grouped.json

# Use a specific transformation matrix
massload transform input.csv --matrix custom-matrix.json

# List cached templates
massload template list

# Show template details
massload template show <id>

# Delete a template
massload template delete <id>

# Show available DSL operations
massload operations

# Show example transformation matrix
massload example-matrix
```

## Configuration

### Environment Variables

| Variable | Description | Required |
|----------|-------------|----------|
| `ANTHROPIC_API_KEY` | Claude API key for AI matrix generation | Yes |
| `RUST_LOG` | Log level (trace, debug, info, warn, error) | No |

### Example `.env`

```bash
ANTHROPIC_API_KEY=sk-ant-api03-...
RUST_LOG=info
```

## Project Structure

```
backend/
├── src/
│   ├── ai/              # Claude AI integration
│   │   ├── mod.rs       # API client
│   │   └── prompt.rs    # System & user prompts
│   ├── api/             # HTTP API layer
│   │   ├── mod.rs       # Module exports
│   │   ├── server.rs    # HTTP server (Axum)
│   │   ├── types.rs     # Request/Response DTOs
│   │   └── logs.rs      # SSE log broadcaster
│   ├── cache/           # Template caching
│   │   └── mod.rs       # Matrix registry
│   ├── parser/          # CSV parsing
│   │   └── mod.rs       # Auto-detect encoding/delimiter
│   ├── transform/       # Transformation engine
│   │   ├── mod.rs       # Module exports
│   │   ├── dsl/         # DSL engine
│   │   │   ├── mod.rs
│   │   │   ├── matrix.rs     # Matrix structure
│   │   │   ├── operations.rs # Transform operations
│   │   │   └── executor.rs   # DSL executor
│   │   ├── grouper.rs   # Flat → Grouped transformation
│   │   └── pipeline.rs  # Main transformation pipeline
│   ├── validation/      # JSON Schema validation
│   │   └── mod.rs       # Schema validators
│   ├── models/          # Domain models
│   │   └── mod.rs       # Creator, PartyId, GroupedWork
│   ├── error.rs         # Hierarchical error types
│   ├── lib.rs           # Library exports & re-exports
│   └── main.rs          # CLI entry point
├── schemas/
│   ├── midds-musical-work-flat.json    # Flat record schema
│   ├── midds-musical-work-grouped.json # SDK-compatible schema
│   └── transformation-matrix-schema.json
└── .massload/           # Runtime data (gitignored)
    └── matrices/        # Cached transformation templates
```

## Building

```bash
# Development
cargo build

# Release
cargo build --release

# Run tests
cargo test
```

---

<div align="center">

Part of [Massload](../README.md) • Built with ❤️ by [Allfeat](https://allfeat.org)

</div>
