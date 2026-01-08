#!/bin/bash
# ghk installer - works on Linux and macOS
# Usage: curl -sSL https://raw.githubusercontent.com/bymehul/ghk/main/install.sh | bash

set -e

REPO="bymehul/ghk"
INSTALL_DIR="${HOME}/.local/bin"

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$ARCH" in
    x86_64) ARCH="x86_64" ;;
    aarch64|arm64) ARCH="aarch64" ;;
    *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
esac

case "$OS" in
    linux) TARGET="${ARCH}-unknown-linux-gnu" ;;
    darwin) TARGET="${ARCH}-apple-darwin" ;;
    *) echo "Unsupported OS: $OS"; exit 1 ;;
esac

# Get latest release
echo "Fetching latest release..."
RELEASE_URL=$(curl -sL "https://api.github.com/repos/${REPO}/releases/latest" | 
    grep "browser_download_url.*${TARGET}" | 
    cut -d '"' -f 4)

if [ -z "$RELEASE_URL" ]; then
    echo "Could not find release for ${TARGET}"
    exit 1
fi

# Create install directory
mkdir -p "$INSTALL_DIR"

# Download and install
echo "Downloading ghk..."
curl -sL "$RELEASE_URL" -o "${INSTALL_DIR}/ghk"
chmod +x "${INSTALL_DIR}/ghk"

echo ""
echo "✔ ghk installed to ${INSTALL_DIR}/ghk"
echo ""

# Check if in PATH
if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
    echo "Add this to your shell config (.bashrc, .zshrc, etc):"
    echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
    echo ""
fi

echo "Run 'ghk setup' to get started!"
