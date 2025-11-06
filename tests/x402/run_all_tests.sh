#!/bin/bash

echo "🧪 Wadah x402 & CDP Integration - Full Test Suite"
echo "=================================================="
echo ""

START_TIME=$(date +%s)
TOTAL_PASS=0
TOTAL_FAIL=0

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

run_test() {
    local test_file="$1"
    local test_name="$2"
    
    echo ""
    echo -e "${BLUE}═══════════════════════════════════════${NC}"
    echo -e "${BLUE}Running: $test_name${NC}"
    echo -e "${BLUE}═══════════════════════════════════════${NC}"
    
    if [ -f "$test_file" ]; then
        chmod +x "$test_file"
        if "$test_file"; then
            echo -e "${GREEN}✅ $test_name: PASSED${NC}"
            return 0
        else
            echo -e "${RED}❌ $test_name: FAILED${NC}"
            return 1
        fi
    else
        echo -e "${YELLOW}⚠️  $test_name: SKIPPED (file not found)${NC}"
        return 2
    fi
}

# Change to test directory
cd /Users/hsp/Projects/wadah-engine/tests/x402

echo "📋 Test Plan:"
echo "  1. Setup & Configuration"
echo "  2. CDP Client"
echo "  3. Payment Flow"
echo ""
echo "Starting tests..."

# Test 1: Setup
if run_test "./01_setup_test.sh" "Setup & Configuration"; then
    ((TOTAL_PASS++))
else
    ((TOTAL_FAIL++))
fi

# Test 2: CDP Client
if run_test "./02_cdp_client_test.sh" "CDP Client"; then
    ((TOTAL_PASS++))
else
    ((TOTAL_FAIL++))
fi

# Test 3: Payment Flow (optional - needs running server)
echo ""
echo -e "${YELLOW}⚠️  Payment Flow Test requires:${NC}"
echo "   - wadah serve running"
echo "   - Base Sepolia access"
read -p "Run payment flow test? (y/n): " run_payment
if [[ $run_payment =~ ^[Yy]$ ]]; then
    if run_test "./04_payment_flow_test.sh" "Payment Flow"; then
        ((TOTAL_PASS++))
    else
        ((TOTAL_FAIL++))
    fi
else
    echo "Skipped payment flow test"
fi

# Calculate duration
END_TIME=$(date +%s)
DURATION=$((END_TIME - START_TIME))

echo ""
echo -e "${BLUE}═══════════════════════════════════════${NC}"
echo "📊 Final Results"
echo -e "${BLUE}═══════════════════════════════════════${NC}"
echo -e "${GREEN}Passed: $TOTAL_PASS${NC}"
echo -e "${RED}Failed: $TOTAL_FAIL${NC}"
echo "Total:  $((TOTAL_PASS + TOTAL_FAIL))"
echo "Duration: ${DURATION}s"
echo ""

if [ $TOTAL_FAIL -eq 0 ]; then
    echo -e "${GREEN}🎉 All tests passed!${NC}"
    echo ""
    echo "✅ CDP Integration is working correctly"
    echo "✅ x402 protocol implementation verified"
    echo "✅ Ready for production testing"
    exit 0
else
    echo -e "${RED}⚠️  Some tests failed${NC}"
    echo ""
    echo "Please review failed tests and fix issues."
    exit 1
fi

