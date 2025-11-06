#!/bin/bash

echo "🧪 Test 02: CDP Client Functionality"
echo "====================================="
echo ""

PASS=0
FAIL=0

test_result() {
    if [ $1 -eq 0 ]; then
        echo "  ✅ PASS: $2"
        ((PASS++))
    else
        echo "  ❌ FAIL: $2"
        ((FAIL++))
    fi
}

# Test 1: Compile CDP client
echo "Test 2.1: CDP Client Compilation"
cd /Users/hsp/Projects/wadah-engine
if cargo build -p wadah-payment --lib > /dev/null 2>&1; then
    test_result 0 "CDP client compiles successfully"
else
    test_result 1 "CDP client compilation failed"
fi

# Test 2: Run CDP client unit tests
echo "Test 2.2: CDP Client Unit Tests"
if cargo test -p wadah-payment --lib cdp_client > /dev/null 2>&1; then
    test_result 0 "CDP client unit tests pass"
else
    test_result 1 "CDP client unit tests failed"
fi

# Test 3: Test CDP config parsing
echo "Test 2.3: CDP Config JSON Parsing"
if cargo test -p wadah-payment test_cdp_config_parsing -- --nocapture 2>&1 | grep -q "test result: ok"; then
    test_result 0 "CDP config parsing works"
else
    test_result 1 "CDP config parsing failed"
fi

# Test 4: Check CDPClient is exported
echo "Test 2.4: CDPClient Export"
if grep -q "pub use cdp_client::" /Users/hsp/Projects/wadah-engine/crates/payment/src/lib.rs; then
    test_result 0 "CDPClient is exported"
else
    test_result 1 "CDPClient not exported"
fi

# Test 5: Verify HTTP client dependency
echo "Test 2.5: HTTP Client Dependency"
if grep -q "reqwest" /Users/hsp/Projects/wadah-engine/crates/payment/Cargo.toml; then
    test_result 0 "reqwest dependency present"
else
    test_result 1 "reqwest dependency missing"
fi

# Test 6: Check auth header generation
echo "Test 2.6: Authentication Header"
if grep -q "fn auth_header" /Users/hsp/Projects/wadah-engine/crates/payment/src/cdp_client.rs; then
    test_result 0 "Auth header method exists"
else
    test_result 1 "Auth header method missing"
fi

# Test 7: Verify payment verification method
echo "Test 2.7: Payment Verification Method"
if grep -q "pub async fn verify_payment" /Users/hsp/Projects/wadah-engine/crates/payment/src/cdp_client.rs; then
    test_result 0 "Payment verification method exists"
else
    test_result 1 "Payment verification method missing"
fi

# Test 8: Check analytics tracking method
echo "Test 2.8: Analytics Tracking Method"
if grep -q "pub async fn track_payment_event" /Users/hsp/Projects/wadah-engine/crates/payment/src/cdp_client.rs; then
    test_result 0 "Analytics tracking method exists"
else
    test_result 1 "Analytics tracking method missing"
fi

echo ""
echo "====================================="
echo "📊 Results"
echo "====================================="
echo "Passed: $PASS"
echo "Failed: $FAIL"
echo "Total:  $((PASS + FAIL))"
echo ""

if [ $FAIL -eq 0 ]; then
    echo "🎉 All CDP client tests passed!"
    exit 0
else
    echo "⚠️  Some tests failed."
    exit 1
fi

