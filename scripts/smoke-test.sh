#!/usr/bin/env bash
#
# Quick smoke test before release
#

set -e

echo "🧪 Wadah v0.1.0 Pre-Release Smoke Test"
echo ""

# Source Rust environment if needed
if [ -f "$HOME/.cargo/env" ]; then
    source "$HOME/.cargo/env"
fi

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT"

echo "1️⃣ Building project..."
cargo build --release -p wadah-cli
echo "✅ Build successful"
echo ""

echo "2️⃣ Running unit tests..."
cargo test --workspace --quiet
echo "✅ All tests passed"
echo ""

echo "3️⃣ Testing CLI commands..."

# Version check
echo "   Testing: wadah --version"
./target/release/wadah --version > /dev/null
echo "   ✓ Version check works"

# Help check
echo "   Testing: wadah --help"
./target/release/wadah --help > /dev/null
echo "   ✓ Help works"

# Init command
echo "   Testing: wadah init"
TEMP_DIR=$(mktemp -d)
cd "$TEMP_DIR"
"$PROJECT_ROOT/target/release/wadah" init test-agent --security minimal > /dev/null 2>&1
if [ -f "wadah.yaml" ]; then
    echo "   ✓ Init works"
else
    echo "   ✗ Init failed"
    exit 1
fi

# Pack command
echo "   Testing: wadah pack"
"$PROJECT_ROOT/target/release/wadah" pack --output test.wpkg > /dev/null 2>&1
if [ -f "test.wpkg" ]; then
    echo "   ✓ Pack works"
else
    echo "   ✗ Pack failed"
    exit 1
fi

# Verify command
echo "   Testing: wadah verify"
"$PROJECT_ROOT/target/release/wadah" verify test.wpkg > /dev/null 2>&1
echo "   ✓ Verify works"

# Cleanup
cd "$PROJECT_ROOT"
rm -rf "$TEMP_DIR"

echo ""
echo "4️⃣ Testing plugins command..."
./target/release/wadah plugins > /dev/null
echo "✅ Plugins command works"
echo ""

echo "5️⃣ Checking documentation..."
DOCS=(
    "README.md"
    "CHANGELOG.md"
    "RELEASE-v0.1.0.md"
    "docs/INDEX.md"
    "docs/Quickstart.md"
    "docs/WadahSpec-v0.1.md"
)

for doc in "${DOCS[@]}"; do
    if [ -f "$doc" ]; then
        echo "   ✓ $doc exists"
    else
        echo "   ✗ $doc missing"
        exit 1
    fi
done
echo "✅ All documentation present"
echo ""

echo "6️⃣ Checking templates..."
TEMPLATES=(
    "templates/hello-world"
    "templates/langchain-rag"
    "templates/devops-copilot"
    "templates/customer-support"
)

for template in "${TEMPLATES[@]}"; do
    if [ -d "$template" ] && [ -f "$template/wadah.yaml" ]; then
        echo "   ✓ $template OK"
    else
        echo "   ✗ $template incomplete"
        exit 1
    fi
done
echo "✅ All templates valid"
echo ""

echo "═══════════════════════════════════════"
echo "✅ ALL SMOKE TESTS PASSED!"
echo "═══════════════════════════════════════"
echo ""
echo "Wadah v0.1.0 is ready for release! 🚀"
echo ""
echo "Next steps:"
echo "  1. git tag -a v0.1.0 -m 'Release v0.1.0'"
echo "  2. git push origin v0.1.0"
echo "  3. ./scripts/build-release.sh"
echo "  4. Create GitHub release"
echo ""

