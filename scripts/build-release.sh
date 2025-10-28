#!/usr/bin/env bash
#
# Build Release Binaries for Multiple Platforms
#

set -e

VERSION=${1:-0.1.0}
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RELEASE_DIR="$PROJECT_ROOT/release"

echo "🔨 Building Wadah v${VERSION} Release Binaries"
echo ""

# Create release directory
mkdir -p "$RELEASE_DIR"

cd "$PROJECT_ROOT"

# Build for current platform
echo "📦 Building for current platform..."
cargo build --release -p wadah-cli

BINARY="target/release/wadah"
if [ ! -f "$BINARY" ]; then
    echo "❌ Build failed!"
    exit 1
fi

# Detect platform
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$OS" in
    darwin) OS_NAME="macos" ;;
    linux) OS_NAME="linux" ;;
    *) OS_NAME="$OS" ;;
esac

case "$ARCH" in
    x86_64) ARCH_NAME="amd64" ;;
    arm64|aarch64) ARCH_NAME="arm64" ;;
    *) ARCH_NAME="$ARCH" ;;
esac

PLATFORM="${OS_NAME}-${ARCH_NAME}"
TARBALL="wadah-${PLATFORM}.tar.gz"

echo "✓ Built for: $PLATFORM"
echo ""

# Create tarball
echo "📦 Creating release package..."
cd target/release
tar czf "$RELEASE_DIR/$TARBALL" wadah
cd -

echo "✓ Created: $RELEASE_DIR/$TARBALL"
echo ""

# Create SHA256 checksum
echo "🔐 Generating checksum..."
cd "$RELEASE_DIR"
shasum -a 256 "$TARBALL" > "${TARBALL}.sha256"
echo "✓ Checksum: ${TARBALL}.sha256"
cd -

echo ""
echo "═══════════════════════════════════"
echo "✅ Release build complete!"
echo "═══════════════════════════════════"
echo ""
echo "Binary: $RELEASE_DIR/$TARBALL"
echo "SHA256: $RELEASE_DIR/${TARBALL}.sha256"
echo ""
echo "To build for other platforms:"
echo "  1. Use cross: cross build --target <target> --release"
echo "  2. Use GitHub Actions (automatic)"
echo "  3. Use Docker buildx"
echo ""
echo "Upload to GitHub Release:"
echo "  gh release upload v${VERSION} $RELEASE_DIR/$TARBALL"
echo ""

