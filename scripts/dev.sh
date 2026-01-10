#!/usr/bin/env bash
#
# Start Notive in development mode
#

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_ROOT"

# Install dependencies if needed
if [ ! -d "node_modules" ]; then
    echo "[Notive] Installing dependencies..."
    pnpm install
fi

# Start development server
echo "[Notive] Starting development mode..."
pnpm tauri dev
