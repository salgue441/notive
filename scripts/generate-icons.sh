#!/usr/bin/env bash
#
# Generate PNG icons from SVG source
#
# Requires: inkscape or rsvg-convert (librsvg2-bin)
#

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
ICONS_DIR="$PROJECT_ROOT/src-tauri/icons"
SVG_SOURCE="$ICONS_DIR/icon.svg"

echo "[Notive] Generating icons from SVG..."

# Check for SVG source
if [ ! -f "$SVG_SOURCE" ]; then
    echo "[ERROR] SVG source not found: $SVG_SOURCE"
    exit 1
fi

# Determine which tool to use
if command -v rsvg-convert &> /dev/null; then
    CONVERTER="rsvg-convert"
elif command -v inkscape &> /dev/null; then
    CONVERTER="inkscape"
else
    echo "[ERROR] No SVG converter found"
    echo "Install one of:"
    echo "  Debian/Ubuntu: sudo apt install librsvg2-bin"
    echo "  Fedora: sudo dnf install librsvg2-tools"
    echo "  Or install Inkscape"
    exit 1
fi

echo "[Notive] Using converter: $CONVERTER"

# Generate icons
generate_png() {
    local size=$1
    local output=$2

    if [ "$CONVERTER" = "rsvg-convert" ]; then
        rsvg-convert -w "$size" -h "$size" "$SVG_SOURCE" -o "$output"
    else
        inkscape "$SVG_SOURCE" -w "$size" -h "$size" -o "$output"
    fi

    echo "  Generated: $output (${size}x${size})"
}

# Standard sizes for Tauri
generate_png 32 "$ICONS_DIR/32x32.png"
generate_png 128 "$ICONS_DIR/128x128.png"
generate_png 256 "$ICONS_DIR/128x128@2x.png"

# Also create icon.png (used by tray)
generate_png 256 "$ICONS_DIR/icon.png"

# Generate .ico for Windows (if running on Windows or using tools)
if command -v convert &> /dev/null; then
    echo "[Notive] Generating Windows .ico..."
    convert "$ICONS_DIR/32x32.png" "$ICONS_DIR/128x128.png" "$ICONS_DIR/128x128@2x.png" "$ICONS_DIR/icon.ico"
    echo "  Generated: $ICONS_DIR/icon.ico"
fi

# Generate .icns for macOS (if running on macOS)
if [ "$(uname)" = "Darwin" ] && command -v iconutil &> /dev/null; then
    echo "[Notive] Generating macOS .icns..."
    ICONSET="$ICONS_DIR/icon.iconset"
    mkdir -p "$ICONSET"

    sips -z 16 16 "$ICONS_DIR/icon.png" --out "$ICONSET/icon_16x16.png"
    sips -z 32 32 "$ICONS_DIR/icon.png" --out "$ICONSET/icon_16x16@2x.png"
    sips -z 32 32 "$ICONS_DIR/icon.png" --out "$ICONSET/icon_32x32.png"
    sips -z 64 64 "$ICONS_DIR/icon.png" --out "$ICONSET/icon_32x32@2x.png"
    sips -z 128 128 "$ICONS_DIR/icon.png" --out "$ICONSET/icon_128x128.png"
    sips -z 256 256 "$ICONS_DIR/icon.png" --out "$ICONSET/icon_128x128@2x.png"
    sips -z 256 256 "$ICONS_DIR/icon.png" --out "$ICONSET/icon_256x256.png"
    sips -z 512 512 "$ICONS_DIR/icon.png" --out "$ICONSET/icon_256x256@2x.png"
    sips -z 512 512 "$ICONS_DIR/icon.png" --out "$ICONSET/icon_512x512.png"
    sips -z 1024 1024 "$ICONS_DIR/icon.png" --out "$ICONSET/icon_512x512@2x.png"

    iconutil -c icns "$ICONSET" -o "$ICONS_DIR/icon.icns"
    rm -rf "$ICONSET"
    echo "  Generated: $ICONS_DIR/icon.icns"
fi

echo ""
echo "[Notive] Icon generation complete!"
ls -la "$ICONS_DIR"
