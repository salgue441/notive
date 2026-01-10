#!/usr/bin/env bash
#
# Build AppImage (universal Linux package)
#

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
DIST_DIR="$PROJECT_ROOT/dist"

echo "[Notive] Building AppImage..."

cd "$PROJECT_ROOT"

# Install dependencies
pnpm install --frozen-lockfile

# Build
pnpm tauri build --bundles appimage

# Copy to dist
mkdir -p "$DIST_DIR"
cp src-tauri/target/release/bundle/appimage/*.AppImage "$DIST_DIR/"

echo "[Notive] AppImage built successfully!"
ls -lh "$DIST_DIR"/*.AppImage
