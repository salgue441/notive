#!/usr/bin/env bash
#
# Build all package formats for Notive
#

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
DIST_DIR="$PROJECT_ROOT/dist"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log_info() { echo -e "${BLUE}[INFO]${NC} $*"; }
log_success() { echo -e "${GREEN}[SUCCESS]${NC} $*"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $*"; }
log_error() { echo -e "${RED}[ERROR]${NC} $*"; }

echo ""
echo "╔══════════════════════════════════════════════╗"
echo "║           Notive Build Script                ║"
echo "╚══════════════════════════════════════════════╝"
echo ""

# Create dist directory
mkdir -p "$DIST_DIR"

cd "$PROJECT_ROOT"

# Check dependencies
log_info "Checking dependencies..."

if ! command -v cargo &> /dev/null; then
    log_error "Rust/Cargo not found. Install from https://rustup.rs/"
    exit 1
fi

if ! command -v pnpm &> /dev/null; then
    log_error "pnpm not found. Install with: npm install -g pnpm"
    exit 1
fi

# Install frontend dependencies
log_info "Installing frontend dependencies..."
pnpm install --frozen-lockfile

# Build AppImage, deb, and rpm
log_info "Building packages (deb, rpm, appimage)..."
pnpm tauri build

# Copy artifacts to dist
log_info "Copying artifacts to dist/..."

find src-tauri/target/release/bundle -type f \( -name "*.deb" -o -name "*.rpm" -o -name "*.AppImage" \) -exec cp {} "$DIST_DIR/" \;

# Build Flatpak (if flatpak-builder is available)
if command -v flatpak-builder &> /dev/null; then
    log_info "Building Flatpak..."
    "$SCRIPT_DIR/build-flatpak.sh" || log_warn "Flatpak build failed (optional)"
else
    log_warn "flatpak-builder not found, skipping Flatpak build"
fi

# Build Snap (if snapcraft is available)
if command -v snapcraft &> /dev/null; then
    log_info "Building Snap..."
    "$SCRIPT_DIR/build-snap.sh" || log_warn "Snap build failed (optional)"
else
    log_warn "snapcraft not found, skipping Snap build"
fi

echo ""
log_success "Build complete!"
echo ""
echo "Artifacts available in: $DIST_DIR"
echo ""
ls -lh "$DIST_DIR"
