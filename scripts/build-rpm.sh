#!/usr/bin/env bash
#
# Build .rpm package for Fedora/RHEL
#

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
DIST_DIR="$PROJECT_ROOT/dist"

echo "[Notive] Building .rpm package..."

cd "$PROJECT_ROOT"

# Install dependencies
pnpm install --frozen-lockfile

# Build
pnpm tauri build --bundles rpm

# Copy to dist
mkdir -p "$DIST_DIR"
cp src-tauri/target/release/bundle/rpm/*.rpm "$DIST_DIR/"

echo "[Notive] .rpm package built successfully!"
ls -lh "$DIST_DIR"/*.rpm
