#!/bin/sh
# install.sh - sshp installer for macOS and Linux
# Usage: curl -sSf https://raw.githubusercontent.com/kaushiktadhani/sshp/main/install.sh | sh

set -e

REPO="kaushiktadhani/sshp"
BINARY_NAME="sshp"

# Detect OS
OS="$(uname -s)"
case "$OS" in
  Linux)  OS="unknown-linux-gnu" ;;
  Darwin) OS="apple-darwin" ;;
  *)
    echo "Unsupported operating system: $OS" >&2
    exit 1
    ;;
esac

# Detect architecture
ARCH="$(uname -m)"
case "$ARCH" in
  x86_64 | amd64) ARCH="x86_64" ;;
  aarch64 | arm64) ARCH="aarch64" ;;
  *)
    echo "Unsupported architecture: $ARCH" >&2
    exit 1
    ;;
esac

TARGET="${ARCH}-${OS}"

# Get the latest release tag from GitHub API
echo "Fetching latest release..."
LATEST_TAG="$(curl -sSf "https://api.github.com/repos/${REPO}/releases/latest" \
  | grep '"tag_name"' \
  | sed -E 's/.*"tag_name": *"([^"]+)".*/\1/')"

if [ -z "$LATEST_TAG" ]; then
  echo "Failed to determine the latest release tag." >&2
  exit 1
fi

echo "Installing ${BINARY_NAME} ${LATEST_TAG} for ${TARGET}..."

ARCHIVE="${BINARY_NAME}-${LATEST_TAG}-${TARGET}.tar.gz"
BASE_URL="https://github.com/${REPO}/releases/download/${LATEST_TAG}"

# Download to temp directory
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

echo "Downloading ${ARCHIVE}..."
curl -sSfL "${BASE_URL}/${ARCHIVE}" -o "${TMP_DIR}/${ARCHIVE}"
curl -sSfL "${BASE_URL}/${ARCHIVE}.sha256" -o "${TMP_DIR}/${ARCHIVE}.sha256"

# Verify checksum
echo "Verifying checksum..."
EXPECTED="$(cat "${TMP_DIR}/${ARCHIVE}.sha256" | awk '{print $1}')"
if command -v sha256sum >/dev/null 2>&1; then
  ACTUAL="$(sha256sum "${TMP_DIR}/${ARCHIVE}" | awk '{print $1}')"
elif command -v shasum >/dev/null 2>&1; then
  ACTUAL="$(shasum -a 256 "${TMP_DIR}/${ARCHIVE}" | awk '{print $1}')"
else
  echo "Warning: no sha256 utility found, skipping checksum verification." >&2
  ACTUAL="$EXPECTED"
fi

if [ "$ACTUAL" != "$EXPECTED" ]; then
  echo "Checksum mismatch! Expected: $EXPECTED  Got: $ACTUAL" >&2
  exit 1
fi
echo "Checksum verified."

# Extract binary
tar -xzf "${TMP_DIR}/${ARCHIVE}" -C "${TMP_DIR}"

# Determine install location
if [ -w "/usr/local/bin" ]; then
  INSTALL_DIR="/usr/local/bin"
elif [ -n "$HOME" ] && { [ -w "$HOME/.local/bin" ] 2>/dev/null || mkdir -p "$HOME/.local/bin" 2>/dev/null; }; then
  INSTALL_DIR="$HOME/.local/bin"
else
  echo "Cannot determine a writable install directory." >&2
  exit 1
fi

# Install binary
install -m 755 "${TMP_DIR}/${BINARY_NAME}" "${INSTALL_DIR}/${BINARY_NAME}"

echo ""
echo "sshp ${LATEST_TAG} installed to ${INSTALL_DIR}/${BINARY_NAME}"

# Warn if install dir is not in PATH
case ":${PATH}:" in
  *":${INSTALL_DIR}:"*) ;;
  *)
    echo ""
    echo "NOTE: ${INSTALL_DIR} is not in your PATH."
    echo "Add the following to your shell config (~/.bashrc or ~/.zshrc):"
    echo "  export PATH=\"${INSTALL_DIR}:\$PATH\""
    ;;
esac

# Add 's' shortcut alias to shell profile
SHELL_NAME="$(basename "$SHELL")"
case "$SHELL_NAME" in
  zsh)  SHELL_RC="$HOME/.zshrc" ;;
  bash) SHELL_RC="$HOME/.bashrc" ;;
  *)    SHELL_RC="" ;;
esac

if [ -n "$SHELL_RC" ]; then
  touch "$SHELL_RC"
  if grep -q "alias s='sshp'" "$SHELL_RC"; then
    echo "Shortcut 's' alias already exists in $SHELL_RC."
  else
    echo "" >> "$SHELL_RC"
    echo "alias s='sshp'" >> "$SHELL_RC"
    echo "Added 's' shortcut alias to $SHELL_RC. Type 's' to launch sshp."
  fi
fi

echo ""
echo "Restart your terminal, then run 'sshp' or 's' to get started!"
