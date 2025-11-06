#!/bin/bash

echo "🧪 Test 01: CDP Setup & Configuration"
echo "======================================"
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

# Test 1: Check if CDP key file exists
echo "Test 1.1: CDP API Key File"
if [ -f "/Users/hsp/Projects/wadah-ui/keys/cdp_api_key.json" ]; then
    test_result 0 "CDP API key file exists"
else
    test_result 1 "CDP API key file not found"
fi

# Test 2: Validate CDP key JSON format
echo "Test 1.2: CDP API Key Format"
if cat /Users/hsp/Projects/wadah-ui/keys/cdp_api_key.json | jq . > /dev/null 2>&1; then
    test_result 0 "Valid JSON format"
else
    test_result 1 "Invalid JSON format"
fi

# Test 3: Check required fields
echo "Test 1.3: CDP API Key Fields"
CDP_ID=$(cat /Users/hsp/Projects/wadah-ui/keys/cdp_api_key.json | jq -r '.id // empty' 2>/dev/null)
CDP_KEY=$(cat /Users/hsp/Projects/wadah-ui/keys/cdp_api_key.json | jq -r '.privateKey // empty' 2>/dev/null)

if [ ! -z "$CDP_ID" ] && [ ! -z "$CDP_KEY" ]; then
    test_result 0 "Required fields present (id, privateKey)"
else
    test_result 1 "Missing required fields"
fi

# Test 4: Check wadah-engine .env
echo "Test 1.4: Wadah Engine Configuration"
if [ -f "/Users/hsp/Projects/wadah-engine/.env" ]; then
    test_result 0 ".env file exists"
    
    # Check for required variables
    if grep -q "CDP_API_KEY_ID" /Users/hsp/Projects/wadah-engine/.env; then
        test_result 0 "CDP_API_KEY_ID configured"
    else
        test_result 1 "CDP_API_KEY_ID missing"
    fi
    
    if grep -q "X402_FACILITATOR_URL" /Users/hsp/Projects/wadah-engine/.env; then
        test_result 0 "X402_FACILITATOR_URL configured"
    else
        test_result 1 "X402_FACILITATOR_URL missing"
    fi
else
    test_result 1 ".env file not found"
fi

# Test 5: Check wadah-ui .env.local
echo "Test 1.5: Wadah UI Configuration"
if [ -f "/Users/hsp/Projects/wadah-ui/.env.local" ]; then
    test_result 0 ".env.local file exists"
else
    test_result 1 ".env.local file not found"
fi

# Test 6: Verify .gitignore protection
echo "Test 1.6: Security - .gitignore"
if grep -q "keys/" /Users/hsp/Projects/wadah-ui/.gitignore; then
    test_result 0 "keys/ folder in .gitignore"
else
    test_result 1 "keys/ folder NOT protected"
fi

if grep -q ".env" /Users/hsp/Projects/wadah-engine/.gitignore; then
    test_result 0 ".env files protected"
else
    test_result 1 ".env files NOT protected"
fi

# Test 7: Check CDP client module
echo "Test 1.7: CDP Client Module"
if [ -f "/Users/hsp/Projects/wadah-engine/crates/payment/src/cdp_client.rs" ]; then
    test_result 0 "CDP client module exists"
else
    test_result 1 "CDP client module missing"
fi

# Test 8: Verify facilitator URL
echo "Test 1.8: Facilitator URL Validation"
FACILITATOR_URL=$(grep "X402_FACILITATOR_URL" /Users/hsp/Projects/wadah-engine/.env 2>/dev/null | cut -d'=' -f2)
if echo "$FACILITATOR_URL" | grep -q "facilitator.x402.coinbase.com"; then
    test_result 0 "Correct facilitator URL"
else
    test_result 1 "Invalid facilitator URL"
fi

echo ""
echo "======================================"
echo "📊 Results"
echo "======================================"
echo "Passed: $PASS"
echo "Failed: $FAIL"
echo "Total:  $((PASS + FAIL))"
echo ""

if [ $FAIL -eq 0 ]; then
    echo "🎉 All setup tests passed!"
    exit 0
else
    echo "⚠️  Some tests failed. Please fix configuration."
    exit 1
fi

