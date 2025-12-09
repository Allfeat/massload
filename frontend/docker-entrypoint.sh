#!/bin/sh
# MassLoad Frontend Entrypoint
# Generates runtime config.js from environment variables
# Compatible with nginx-unprivileged (non-root container)

set -e

# Default values
BACKEND_URL="${BACKEND_URL:-http://localhost:3000}"
BLOCKCHAIN_RPC="${BLOCKCHAIN_RPC:-wss://node-dev.allfeat.io}"

# Generate config.js from template
echo "[MassLoad] Generating runtime config..."
echo "  BACKEND_URL: $BACKEND_URL"
echo "  BLOCKCHAIN_RPC: $BLOCKCHAIN_RPC"

cat > /usr/share/nginx/html/config.js << EOF
// Runtime configuration - Generated at container startup
// DO NOT EDIT - This file is generated from environment variables
window.MASSLOAD_CONFIG = {
    BACKEND_URL: "$BACKEND_URL",
    BLOCKCHAIN_RPC: "$BLOCKCHAIN_RPC"
};
console.log("[MassLoad] Runtime config loaded:", window.MASSLOAD_CONFIG);
EOF

echo "[MassLoad] Config generated at /usr/share/nginx/html/config.js"
echo "[MassLoad] Starting nginx..."

# Execute the command passed as arguments (typically: nginx -g "daemon off;")
exec "$@"

