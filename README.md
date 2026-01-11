<div align="center">

<!-- Logo placeholder - replace with actual logo -->
<img src="assets/logo.svg" alt="Notive Logo" width="120" height="120">

# Notive

**A high-performance Notion desktop wrapper for Linux**

[![CI](https://github.com/YOUR_USERNAME/notive/actions/workflows/ci.yml/badge.svg)](https://github.com/YOUR_USERNAME/notive/actions/workflows/ci.yml)
[![Release](https://github.com/YOUR_USERNAME/notive/actions/workflows/release.yml/badge.svg)](https://github.com/YOUR_USERNAME/notive/actions/workflows/release.yml)
[![GitHub release](https://img.shields.io/github/v/release/YOUR_USERNAME/notive?style=flat-square)](https://github.com/YOUR_USERNAME/notive/releases/latest)
[![License](https://img.shields.io/github/license/YOUR_USERNAME/notive?style=flat-square)](LICENSE)
[![Downloads](https://img.shields.io/github/downloads/YOUR_USERNAME/notive/total?style=flat-square)](https://github.com/YOUR_USERNAME/notive/releases)

[Features](#features) · [Installation](#installation) · [Usage](#usage) · [Building](#building-from-source) · [Contributing](#contributing)

---

<img src="assets/screenshot.png" alt="Notive Screenshot" width="800">

</div>

## Why Notive?

Notion doesn't offer an official Linux client. Existing wrappers are often outdated Electron apps with poor performance and missing features. **Notive** changes that.

Built with [Tauri](https://tauri.app/) and Rust, Notive provides a native desktop experience with:

- **Minimal footprint** — ~15MB vs 150MB+ for Electron alternatives
- **Native performance** — Uses system WebKitGTK, not bundled Chromium
- **Full desktop integration** — Tray, notifications, shortcuts, auto-start
- **Always up-to-date** — Loads Notion's web app directly, no sync issues

## Features

| Feature | Description |
|---------|-------------|
| **System Tray** | Minimize to tray, quick access menu, notification badges |
| **Native Notifications** | Desktop notifications that respect your system settings |
| **Global Shortcuts** | Access Notion from anywhere with customizable hotkeys |
| **Auto-Start** | Launch Notive when you log in |
| **Auto-Updates** | Seamless background updates with changelog |
| **Zoom Controls** | Adjust page zoom to your preference |
| **Download Manager** | Native file download dialogs |
| **Multi-Account** | OAuth support for Google, Apple, Microsoft |

## Installation

### AppImage (Universal)

```bash
# Download latest AppImage
wget https://github.com/YOUR_USERNAME/notive/releases/latest/download/notive_amd64.AppImage

# Make executable and run
chmod +x notive_amd64.AppImage
./notive_amd64.AppImage
```

<details>
<summary><strong>Integrate with desktop</strong></summary>

```bash
# Optional: Install AppImageLauncher for better integration
# https://github.com/TheAssassin/AppImageLauncher
```

</details>

### Debian / Ubuntu (.deb)

```bash
# Download and install
wget https://github.com/YOUR_USERNAME/notive/releases/latest/download/notive_amd64.deb
sudo dpkg -i notive_amd64.deb

# If you encounter dependency issues
sudo apt-get install -f
```

### Fedora / RHEL (.rpm)

```bash
# Download and install
wget https://github.com/YOUR_USERNAME/notive/releases/latest/download/notive_x86_64.rpm
sudo rpm -i notive_x86_64.rpm

# Or with dnf
sudo dnf install ./notive_x86_64.rpm
```

### Flatpak

```bash
# Add Flathub if not already added
flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo

# Install Notive
flatpak install flathub io.github.notive.Notive

# Run
flatpak run io.github.notive.Notive
```

### Snap

```bash
sudo snap install notive
```

### Arch Linux (AUR)

```bash
# Using yay
yay -S notive-bin

# Or using paru
paru -S notive-bin
```

## Usage

### Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| <kbd>Ctrl</kbd>+<kbd>Shift</kbd>+<kbd>N</kbd> | Toggle window visibility |
| <kbd>Ctrl</kbd>+<kbd>Shift</kbd>+<kbd>C</kbd> | Quick capture |
| <kbd>Ctrl</kbd>+<kbd>R</kbd> | Reload page |
| <kbd>Ctrl</kbd>+<kbd>=</kbd> | Zoom in |
| <kbd>Ctrl</kbd>+<kbd>-</kbd> | Zoom out |
| <kbd>Ctrl</kbd>+<kbd>0</kbd> | Reset zoom |
| <kbd>F11</kbd> | Toggle fullscreen |

### Command Line Options

```bash
notive                    # Start normally
notive --minimized        # Start minimized to tray
notive --version          # Show version
notive --help             # Show help
```

### Tray Menu

Right-click the tray icon to access:
- Show/Hide window
- Settings
- Check for updates
- About
- Quit

## Building from Source

### Prerequisites

<details>
<summary><strong>Debian / Ubuntu</strong></summary>

```bash
sudo apt update
sudo apt install -y \
    libwebkit2gtk-4.1-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    libssl-dev \
    libsoup-3.0-dev \
    build-essential \
    curl \
    wget \
    file
```

</details>

<details>
<summary><strong>Fedora</strong></summary>

```bash
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
    file
```

</details>

<details>
<summary><strong>Arch Linux</strong></summary>

```bash
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
    file
```

</details>

### Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### Install Node.js and pnpm

```bash
# Using nvm (recommended)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.0/install.sh | bash
source ~/.bashrc
nvm install 22
npm install -g pnpm
```

### Build

```bash
# Clone the repository
git clone https://github.com/YOUR_USERNAME/notive.git
cd notive

# Install dependencies
pnpm install

# Development mode
pnpm tauri dev

# Production build
pnpm tauri build
```

Build artifacts will be in `src-tauri/target/release/bundle/`.

## Configuration

Settings are stored in `~/.config/notive/settings.json`:

```json
{
  "start_minimized": false,
  "minimize_to_tray": true,
  "close_to_tray": true,
  "notifications_enabled": true,
  "auto_update": true,
  "zoom_level": 1.0,
  "shortcuts": {
    "toggle_window": "Ctrl+Shift+N",
    "quick_capture": "Ctrl+Shift+C"
  }
}
```

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Quick Start

1. Fork the repository
2. Create a feature branch (`git checkout -b feat/amazing-feature`)
3. Commit your changes (`git commit -m 'feat: add amazing feature'`)
4. Push to the branch (`git push origin feat/amazing-feature`)
5. Open a Pull Request

### Development

```bash
# Run in development mode with hot reload
pnpm tauri dev

# Run linters
pnpm lint              # TypeScript/JavaScript
cd src-tauri && cargo clippy  # Rust

# Run tests
pnpm test              # Frontend
cd src-tauri && cargo test    # Backend

# Format code
pnpm format            # TypeScript/JavaScript
cd src-tauri && cargo fmt     # Rust
```

## Security

If you discover a security vulnerability, please see our [Security Policy](SECURITY.md) for responsible disclosure guidelines.

## Roadmap

### ✅ Completed Features

- [x] Custom CSS/themes support
- [x] Multiple workspaces
- [x] Offline indicator
- [x] Tab support
- [x] Integration with system calendar
- [x] Wayland native support improvements
- [x] Multi-account OAuth support
- [x] Shortcut customization UI
- [x] Periodic update checks
- [x] Enhanced download manager
- [x] Tray notification badges

### 🔮 Future Features

See [FUTURE_FEATURES.md](FUTURE_FEATURES.md) for a comprehensive list of potential enhancements including:
- Theme support (dark/light mode)
- Bookmarks and favorites
- Search integration
- Plugin system
- Mobile companion app
- And many more...

See the [open issues](https://github.com/YOUR_USERNAME/notive/issues) for a full list of proposed features and known issues.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Tauri](https://tauri.app/) - The framework that makes this possible
- [Notion](https://notion.so/) - The productivity tool we're wrapping
- All our [contributors](https://github.com/YOUR_USERNAME/notive/graphs/contributors)

---

<div align="center">

**[Report Bug](https://github.com/YOUR_USERNAME/notive/issues/new?template=bug_report.yml)** · **[Request Feature](https://github.com/YOUR_USERNAME/notive/issues/new?template=feature_request.yml)** · **[Discussions](https://github.com/YOUR_USERNAME/notive/discussions)**

Made with Rust and Tauri

</div>
