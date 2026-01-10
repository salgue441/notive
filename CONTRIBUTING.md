# Contributing to Notive

First off, thank you for considering contributing to Notive! It's people like you that make Notive such a great tool for the Linux community.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [How Can I Contribute?](#how-can-i-contribute)
- [Style Guidelines](#style-guidelines)
- [Commit Messages](#commit-messages)
- [Pull Request Process](#pull-request-process)
- [Community](#community)

## Code of Conduct

This project and everyone participating in it is governed by our [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code. Please report unacceptable behavior to [INSERT EMAIL].

## Getting Started

### Prerequisites

Before you begin, ensure you have the following installed:

- **Rust** (1.77.2 or later) - [Install Rust](https://rustup.rs/)
- **Node.js** (22 or later) - [Install Node.js](https://nodejs.org/)
- **pnpm** (9 or later) - `npm install -g pnpm`
- **Linux development libraries** (see below)

#### Linux Dependencies

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
    @development-tools
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
    base-devel
```

</details>

## Development Setup

1. **Fork the repository** on GitHub

2. **Clone your fork**
   ```bash
   git clone https://github.com/YOUR_USERNAME/notive.git
   cd notive
   ```

3. **Add upstream remote**
   ```bash
   git remote add upstream https://github.com/ORIGINAL_OWNER/notive.git
   ```

4. **Install dependencies**
   ```bash
   pnpm install
   ```

5. **Start development server**
   ```bash
   pnpm tauri dev
   ```

### Useful Commands

| Command | Description |
|---------|-------------|
| `pnpm tauri dev` | Start development mode with hot reload |
| `pnpm tauri build` | Create production build |
| `pnpm lint` | Run frontend linter (Biome) |
| `pnpm lint:check` | Check frontend without fixing |
| `pnpm format` | Format frontend code |
| `pnpm test` | Run frontend tests |
| `pnpm typecheck` | Run TypeScript type checking |
| `cd src-tauri && cargo fmt` | Format Rust code |
| `cd src-tauri && cargo clippy` | Run Rust linter |
| `cd src-tauri && cargo test` | Run Rust tests |

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check the [existing issues](https://github.com/YOUR_USERNAME/notive/issues) to avoid duplicates.

When creating a bug report, please use the [bug report template](.github/ISSUE_TEMPLATE/bug_report.yml) and include:

- A clear, descriptive title
- Steps to reproduce the behavior
- Expected vs actual behavior
- Your environment (distro, desktop environment, package format)
- Screenshots if applicable
- Logs (`RUST_LOG=debug notive` for verbose output)

### Suggesting Features

Feature requests are welcome! Please use the [feature request template](.github/ISSUE_TEMPLATE/feature_request.yml) and include:

- A clear description of the problem you're trying to solve
- Your proposed solution
- Alternative solutions you've considered
- How important this feature is to your workflow

### Your First Code Contribution

Not sure where to start? Look for issues labeled:

- [`good first issue`](https://github.com/YOUR_USERNAME/notive/labels/good%20first%20issue) - Simple issues for newcomers
- [`help wanted`](https://github.com/YOUR_USERNAME/notive/labels/help%20wanted) - Issues where we need community help

### Pull Requests

1. **Create a branch** from `main`
   ```bash
   git checkout -b feat/my-feature
   # or
   git checkout -b fix/my-bugfix
   ```

2. **Make your changes** following our [style guidelines](#style-guidelines)

3. **Test your changes**
   ```bash
   pnpm test
   cd src-tauri && cargo test
   ```

4. **Lint your code**
   ```bash
   pnpm lint
   cd src-tauri && cargo fmt && cargo clippy
   ```

5. **Commit your changes** following our [commit message guidelines](#commit-messages)

6. **Push to your fork**
   ```bash
   git push origin feat/my-feature
   ```

7. **Open a Pull Request** using our [PR template](.github/PULL_REQUEST_TEMPLATE.md)

## Style Guidelines

### Rust Code

- Follow the [Rust Style Guide](https://doc.rust-lang.org/nightly/style-guide/)
- Use `cargo fmt` before committing
- Address all `cargo clippy` warnings
- Write documentation for public APIs
- Keep functions focused and small

```rust
// Good: Clear, documented, focused
/// Toggles the main window visibility.
///
/// If the window is visible, it will be hidden.
/// If hidden, it will be shown and focused.
pub fn toggle_window<R: Runtime>(app: &AppHandle<R>) -> Result<(), Error> {
    let window = app.get_webview_window("main")
        .ok_or(Error::WindowNotFound)?;

    if window.is_visible()? {
        window.hide()?;
    } else {
        window.show()?;
        window.set_focus()?;
    }

    Ok(())
}
```

### TypeScript Code

- Use Biome for formatting and linting
- Prefer `const` over `let`
- Use TypeScript strict mode
- Avoid `any` types

```typescript
// Good: Typed, const, clear naming
const showNotification = async (title: string, body: string): Promise<void> => {
  await invoke('show_notification', { notification: { title, body } });
};
```

### File Organization

```
src-tauri/src/
├── commands/       # IPC command handlers
├── config/         # Settings and configuration
├── handlers/       # Event handlers (downloads, navigation)
├── notifications/  # Notification system
├── shortcuts/      # Keyboard shortcuts
├── tray/           # System tray
├── updater/        # Auto-update logic
├── utils/          # Shared utilities
├── lib.rs          # Library entry point
└── main.rs         # Application entry point
```

## Commit Messages

We follow [Conventional Commits](https://www.conventionalcommits.org/). This enables automatic changelog generation and semantic versioning.

### Format

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

### Types

| Type | Description |
|------|-------------|
| `feat` | New feature |
| `fix` | Bug fix |
| `docs` | Documentation only |
| `style` | Code style (formatting, semicolons) |
| `refactor` | Code change that neither fixes a bug nor adds a feature |
| `perf` | Performance improvement |
| `test` | Adding or fixing tests |
| `build` | Build system or dependencies |
| `ci` | CI/CD configuration |
| `chore` | Other changes (maintenance) |
| `revert` | Revert a previous commit |

### Examples

```bash
# Feature
feat(tray): add notification badge count

# Bug fix
fix(shortcuts): resolve global shortcut not registering on Wayland

# Documentation
docs: update installation instructions for Fedora

# Breaking change (note the !)
feat(config)!: change settings file format to TOML

BREAKING CHANGE: Settings are now stored in settings.toml instead of settings.json.
Users will need to migrate their settings manually.
```

### Scope (optional)

Common scopes: `tray`, `shortcuts`, `notifications`, `config`, `updater`, `build`, `ci`, `deps`

## Pull Request Process

1. **Ensure CI passes** - All checks must be green
2. **Update documentation** - If you change behavior, update docs
3. **Add tests** - For new features or bug fixes
4. **Request review** - Tag maintainers if needed
5. **Address feedback** - Respond to review comments
6. **Squash if needed** - Keep history clean

### PR Title Format

Use the same format as commit messages:

```
feat(tray): add notification badge count
fix(shortcuts): resolve registration issue on Wayland
```

### Review Process

- PRs require at least one approval from a maintainer
- CI must pass before merging
- Maintainers may request changes or suggest improvements
- Be patient - we review PRs as quickly as possible

## Community

- **Discussions**: [GitHub Discussions](https://github.com/YOUR_USERNAME/notive/discussions)
- **Issues**: [GitHub Issues](https://github.com/YOUR_USERNAME/notive/issues)

### Recognition

Contributors are recognized in:
- The [README.md](README.md) acknowledgments
- The [GitHub contributors page](https://github.com/YOUR_USERNAME/notive/graphs/contributors)
- Release notes for significant contributions

---

Thank you for contributing to Notive! Your efforts help make Notion accessible to the Linux community.
