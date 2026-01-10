# Notive Build Environment
# Multi-stage Dockerfile for reproducible builds

# ===========================================
# Stage 1: Build Environment
# ===========================================
FROM rust:1.83-bookworm AS builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
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
    patchelf \
    squashfs-tools \
    rpm \
    && rm -rf /var/lib/apt/lists/*

# Install Node.js 22 and pnpm
RUN curl -fsSL https://deb.nodesource.com/setup_22.x | bash - \
    && apt-get install -y nodejs \
    && npm install -g pnpm@9 \
    && rm -rf /var/lib/apt/lists/*

# Install Tauri CLI
RUN cargo install tauri-cli --version "^2.9" --locked

# Set working directory
WORKDIR /app

# Copy dependency files first (for Docker layer caching)
COPY package.json pnpm-lock.yaml ./
COPY src-tauri/Cargo.toml src-tauri/Cargo.lock ./src-tauri/

# Install frontend dependencies
RUN pnpm install --frozen-lockfile

# Create dummy Rust source for dependency compilation
RUN mkdir -p src-tauri/src \
    && echo "fn main() {}" > src-tauri/src/main.rs \
    && echo "pub fn run() {}" > src-tauri/src/lib.rs

# Pre-build Rust dependencies
WORKDIR /app/src-tauri
RUN cargo build --release
RUN rm -rf src

# Copy full source
WORKDIR /app
COPY . .

# Build the application
RUN pnpm tauri build

# ===========================================
# Stage 2: Artifacts Export
# ===========================================
FROM scratch AS artifacts

# Copy build artifacts
COPY --from=builder /app/src-tauri/target/release/bundle/deb/*.deb /
COPY --from=builder /app/src-tauri/target/release/bundle/rpm/*.rpm /
COPY --from=builder /app/src-tauri/target/release/bundle/appimage/*.AppImage /

# ===========================================
# Stage 3: Development Environment
# ===========================================
FROM rust:1.83-bookworm AS dev

# Install system dependencies
RUN apt-get update && apt-get install -y \
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
    git \
    && rm -rf /var/lib/apt/lists/*

# Install Node.js 22 and pnpm
RUN curl -fsSL https://deb.nodesource.com/setup_22.x | bash - \
    && apt-get install -y nodejs \
    && npm install -g pnpm@9 \
    && rm -rf /var/lib/apt/lists/*

# Install Rust components
RUN rustup component add rustfmt clippy

# Install Tauri CLI
RUN cargo install tauri-cli --version "^2.9" --locked

# Set working directory
WORKDIR /app

# Default command
CMD ["bash"]
