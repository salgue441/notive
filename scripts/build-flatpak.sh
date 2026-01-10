#!/usr/bin/env bash
#
# Build Flatpak package
#

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
DIST_DIR="$PROJECT_ROOT/dist"
FLATPAK_DIR="$PROJECT_ROOT/flatpak"

echo "[Notive] Building Flatpak..."

# Check for flatpak-builder
if ! command -v flatpak-builder &> /dev/null; then
    echo "[ERROR] flatpak-builder not found"
    echo "Install with:"
    echo "  Debian/Ubuntu: sudo apt install flatpak-builder"
    echo "  Fedora: sudo dnf install flatpak-builder"
    echo "  Arch: sudo pacman -S flatpak-builder"
    exit 1
fi

cd "$PROJECT_ROOT"

# Ensure Flatpak SDKs are installed
echo "[Notive] Ensuring Flatpak SDKs are installed..."
flatpak install -y flathub org.gnome.Platform//47 org.gnome.Sdk//47 \
    org.freedesktop.Sdk.Extension.rust-stable//24.08 \
    org.freedesktop.Sdk.Extension.node22//24.08 || true

# Build Flatpak
mkdir -p "$FLATPAK_DIR/build" "$FLATPAK_DIR/repo"

flatpak-builder \
    --force-clean \
    --repo="$FLATPAK_DIR/repo" \
    --install-deps-from=flathub \
    "$FLATPAK_DIR/build" \
    "$FLATPAK_DIR/io.github.notive.Notive.yml"

# Create bundle
mkdir -p "$DIST_DIR"
flatpak build-bundle \
    "$FLATPAK_DIR/repo" \
    "$DIST_DIR/notive.flatpak" \
    io.github.notive.Notive

echo "[Notive] Flatpak built successfully!"
ls -lh "$DIST_DIR"/*.flatpak
