#!/usr/bin/env bash
#
# Integration Test Suite for Wadah CLI
# Tests the complete agent lifecycle: init → pack → verify → run
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
WADAH_BIN="$PROJECT_ROOT/target/release/wadah"
TEST_DIR="/tmp/wadah-integration-test-$$"

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${GREEN}✓${NC} $1"
}

log_error() {
    echo -e "${RED}✗${NC} $1"
}

log_test() {
    echo -e "${YELLOW}→${NC} $1"
}

cleanup() {
    if [ -d "$TEST_DIR" ]; then
        rm -rf "$TEST_DIR"
    fi
}

trap cleanup EXIT

# Build the binary first
if [ ! -f "$WADAH_BIN" ]; then
    echo "Building wadah CLI..."
    cd "$PROJECT_ROOT"
    cargo build --release -p wadah-cli
fi

# Create test directory
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

echo "================================"
echo "Wadah Integration Test Suite"
echo "================================"
echo ""

# Test 1: Help command
log_test "Test 1: Help command"
if "$WADAH_BIN" --help > /dev/null 2>&1; then
    log_info "Help command works"
else
    log_error "Help command failed"
    exit 1
fi

# Test 2: Version command
log_test "Test 2: Version command"
if "$WADAH_BIN" --version > /dev/null 2>&1; then
    log_info "Version command works"
else
    log_error "Version command failed"
    exit 1
fi

# Test 3: Init minimal agent
log_test "Test 3: Init minimal agent"
"$WADAH_BIN" init test-minimal --security minimal > /dev/null 2>&1
if [ -f "wadah.yaml" ] && [ -d "prompts" ] && [ -d "build" ]; then
    log_info "Minimal agent initialized successfully"
else
    log_error "Minimal agent init failed"
    exit 1
fi

# Test 4: Pack minimal agent
log_test "Test 4: Pack minimal agent"
"$WADAH_BIN" pack -m wadah.yaml -o test-minimal.wpkg > /dev/null 2>&1
if [ -f "test-minimal.wpkg" ]; then
    SIZE=$(stat -f%z "test-minimal.wpkg" 2>/dev/null || stat -c%s "test-minimal.wpkg" 2>/dev/null)
    log_info "Package created successfully ($SIZE bytes)"
else
    log_error "Pack command failed"
    exit 1
fi

# Test 5: Verify package
log_test "Test 5: Verify package integrity"
if "$WADAH_BIN" verify test-minimal.wpkg > /dev/null 2>&1; then
    log_info "Package verification passed"
else
    log_error "Package verification failed"
    exit 1
fi

# Test 6: Init standard agent
log_test "Test 6: Init standard agent"
rm -rf wadah.yaml prompts build README.md .gitignore
"$WADAH_BIN" init test-standard --security standard > /dev/null 2>&1
if [ -f "wadah.yaml" ] && grep -q "security.budgets" wadah.yaml; then
    log_info "Standard agent initialized with security plugins"
else
    log_error "Standard agent init failed"
    exit 1
fi

# Test 7: Init strict agent
log_test "Test 7: Init strict agent"
rm -rf wadah.yaml prompts build README.md .gitignore tools code
"$WADAH_BIN" init test-strict --security strict > /dev/null 2>&1
if [ -f "wadah.yaml" ] && [ -f "ToolCaps.json" ] && [ -d "tools" ]; then
    log_info "Strict agent initialized with full security"
else
    log_error "Strict agent init failed"
    exit 1
fi

# Test 8: Pack strict agent
log_test "Test 8: Pack strict agent with artifacts"
"$WADAH_BIN" pack -m wadah.yaml -o test-strict.wpkg > /dev/null 2>&1
if [ -f "test-strict.wpkg" ]; then
    log_info "Strict agent packaged successfully"
else
    log_error "Strict agent pack failed"
    exit 1
fi

# Test 9: Plugins command
log_test "Test 9: List security plugins"
if "$WADAH_BIN" plugins > /dev/null 2>&1; then
    log_info "Plugins command works"
else
    log_error "Plugins command failed"
    exit 1
fi

# Test 10: Invalid package verification (should fail gracefully)
log_test "Test 10: Invalid package handling"
echo "not a valid package" > invalid.wpkg
if ! "$WADAH_BIN" verify invalid.wpkg > /dev/null 2>&1; then
    log_info "Invalid package correctly rejected"
else
    log_error "Invalid package was accepted (should have failed)"
    exit 1
fi

echo ""
echo "================================"
echo -e "${GREEN}All integration tests passed!${NC}"
echo "================================"

exit 0

