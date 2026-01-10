#!/usr/bin/env bash
#
# Install build dependencies for Notive
#

set -euo pipefail

echo "[Notive] Installing build dependencies..."

# Detect distribution
if [ -f /etc/os-release ]; then
    . /etc/os-release
    DISTRO=$ID
else
    echo "[ERROR] Cannot detect Linux distribution"
    exit 1
fi

case "$DISTRO" in
    ubuntu|debian|linuxmint|pop)
        echo "[Notive] Detected Debian-based distribution"
        sudo apt-get update
        sudo apt-get install -y \
            libwebkit2gtk-4.1-dev \
            libgtk-3-dev \
            libayatana-appindicator3-dev \
            librsvg2-dev \
            libssl-dev \
            libsoup-3.0-dev \
            build-essential \
            curl \
            wget \
            file \
            patchelf
        ;;
    fedora|rhel|centos|rocky|almalinux)
        echo "[Notive] Detected Red Hat-based distribution"
        sudo dnf install -y \
            webkit2gtk4.1-devel \
            gtk3-devel \
            libayatana-appindicator-gtk3-devel \
            librsvg2-devel \
            openssl-devel \
            libsoup3-devel \
            @development-tools \
            curl \
            wget \
            file \
            patchelf
        ;;
    arch|manjaro|endeavouros)
        echo "[Notive] Detected Arch-based distribution"
        sudo pacman -S --needed \
            webkit2gtk-4.1 \
            gtk3 \
            libayatana-appindicator \
            librsvg \
            openssl \
            libsoup3 \
            base-devel \
            curl \
            wget \
            file \
            patchelf
        ;;
    opensuse*|suse)
        echo "[Notive] Detected openSUSE"
        sudo zypper install -y \
            webkit2gtk3-devel \
            gtk3-devel \
            libayatana-appindicator3-1 \
            librsvg-devel \
            libopenssl-devel \
            libsoup-devel \
            -t pattern devel_basis \
            curl \
            wget \
            file \
            patchelf
        ;;
    *)
        echo "[ERROR] Unsupported distribution: $DISTRO"
        echo "Please install the following packages manually:"
        echo "  - WebKitGTK 4.1 development files"
        echo "  - GTK3 development files"
        echo "  - libayatana-appindicator development files"
        echo "  - librsvg development files"
        echo "  - OpenSSL development files"
        echo "  - libsoup3 development files"
        echo "  - Build essentials (gcc, make, etc.)"
        exit 1
        ;;
esac

# Install Rust if not present
if ! command -v cargo &> /dev/null; then
    echo "[Notive] Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

# Install Node.js if not present
if ! command -v node &> /dev/null; then
    echo "[Notive] Installing Node.js via nvm..."
    curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.0/install.sh | bash
    export NVM_DIR="$HOME/.nvm"
    [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
    nvm install 22
fi

# Install pnpm if not present
if ! command -v pnpm &> /dev/null; then
    echo "[Notive] Installing pnpm..."
    npm install -g pnpm
fi

echo ""
echo "[Notive] Dependencies installed successfully!"
echo ""
echo "You can now build Notive with:"
echo "  pnpm install"
echo "  pnpm tauri build"
