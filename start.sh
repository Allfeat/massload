#!/bin/bash
set -e

# Default port
PORT=${PORT:-3000}

# Check that binary exists
if [ ! -f target/release/massload ]; then
    echo "❌ Backend binary not found!"
    echo "   Run: ./build.sh"
    exit 1
fi

# Load .env if exists
if [ -f .env ]; then
    export $(cat .env | grep -v '^#' | xargs)
    echo "✅ Loaded .env"
fi

echo "🚀 Starting MassLoad on http://localhost:$PORT"
echo ""
./target/release/massload serve --port "$PORT"

