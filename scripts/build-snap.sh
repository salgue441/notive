#!/usr/bin/env bash
#
# Build Snap package
#

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
DIST_DIR="$PROJECT_ROOT/dist"

echo "[Notive] Building Snap..."

# Check for snapcraft
if ! command -v snapcraft &> /dev/null; then
    echo "[ERROR] snapcraft not found"
    echo "Install with: sudo snap install snapcraft --classic"
    exit 1
fi

cd "$PROJECT_ROOT"

# Build Snap
snapcraft

# Move to dist
mkdir -p "$DIST_DIR"
mv *.snap "$DIST_DIR/"

echo "[Notive] Snap built successfully!"
ls -lh "$DIST_DIR"/*.snap
