#!/bin/sh
# Docker entrypoint - generates config.js from environment variables

set -e

# Default values
BACKEND_URL="${BACKEND_URL:-http://localhost:3000}"
BLOCKCHAIN_RPC="${BLOCKCHAIN_RPC:-wss://node-dev.allfeat.io}"

# Generate config.js from template
echo "Generating runtime config..."
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

echo "Config generated at /usr/share/nginx/html/config.js"

# Start nginx
exec nginx -g "daemon off;"

