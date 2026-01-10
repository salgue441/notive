#!/usr/bin/env bash
#
# Build .deb package for Debian/Ubuntu
#

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
DIST_DIR="$PROJECT_ROOT/dist"

echo "[Notive] Building .deb package..."

cd "$PROJECT_ROOT"

# Install dependencies
pnpm install --frozen-lockfile

# Build
pnpm tauri build --bundles deb

# Copy to dist
mkdir -p "$DIST_DIR"
cp src-tauri/target/release/bundle/deb/*.deb "$DIST_DIR/"

echo "[Notive] .deb package built successfully!"
ls -lh "$DIST_DIR"/*.deb
