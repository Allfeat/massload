#!/bin/bash
set -e

echo "🔨 Building MassLoad..."
echo ""

# Build frontend (WASM)
echo "📦 Building frontend (WASM)..."
cd frontend
unset NO_COLOR  # Trunk expects true/false, not 1/0
trunk build --release
cd ..
echo "✅ Frontend built"

# Build backend
echo ""
echo "🦀 Building backend..."
cargo build --release --bin massload
echo "✅ Backend built"

echo ""
echo "🎉 Build complete!"
echo ""
echo "To start the server:"
echo "  ./target/release/massload serve --port 3000"
echo ""
echo "Or use: ./start.sh"

