#!/bin/bash

echo "🧪 Test 04: End-to-End Payment Flow"
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

echo "⚠️  This test requires:"
echo "  1. Base Sepolia RPC access"
echo "  2. Test wallet with USDC"
echo "  3. wadah serve running"
echo ""

# Test 1: Check if wadah serve starts
echo "Test 4.1: Start wadah serve"
cd /Users/hsp/Projects/wadah-engine
timeout 5s cargo run --release -- serve examples/premium-agent-with-payment.yaml > /tmp/wadah-serve.log 2>&1 &
SERVE_PID=$!
sleep 3

if ps -p $SERVE_PID > /dev/null; then
    test_result 0 "wadah serve started successfully"
    kill $SERVE_PID 2>/dev/null
else
    test_result 1 "wadah serve failed to start"
    cat /tmp/wadah-serve.log
fi

# Test 2: Test 402 Payment Required response
echo "Test 4.2: 402 Payment Required"
response=$(curl -s -w "%{http_code}" -X POST http://localhost:3402/agent/premium-support-agent/run \
  -H "Content-Type: application/json" \
  -d '{"prompt": "test"}' \
  -o /tmp/payment-response.json)

if echo "$response" | grep -q "402"; then
    test_result 0 "Returns 402 Payment Required"
else
    test_result 1 "Did not return 402"
fi

# Test 3: Validate payment requirements format
echo "Test 4.3: Payment Requirements Format"
if [ -f /tmp/payment-response.json ]; then
    if cat /tmp/payment-response.json | jq -e '.accepts[0].scheme' > /dev/null 2>&1; then
        test_result 0 "Valid payment requirements JSON"
    else
        test_result 1 "Invalid payment requirements format"
    fi
else
    test_result 1 "No payment response file"
fi

# Test 4: Check x402 version
echo "Test 4.4: x402 Protocol Version"
if cat /tmp/payment-response.json 2>/dev/null | jq -e '.x402_version' | grep -q "1.0"; then
    test_result 0 "Correct x402 version"
else
    test_result 1 "Incorrect x402 version"
fi

# Test 5: Verify USDC contract address
echo "Test 4.5: USDC Contract Address"
if cat /tmp/payment-response.json 2>/dev/null | jq -r '.accepts[0].asset' | grep -qi "0x036CbD53842c5426634e7929541eC2318f3dCF7e"; then
    test_result 0 "Correct USDC address (Base Sepolia)"
else
    test_result 1 "Incorrect USDC address"
fi

# Test 6: Check payment amount
echo "Test 4.6: Payment Amount"
amount=$(cat /tmp/payment-response.json 2>/dev/null | jq -r '.accepts[0].max_amount_required')
if [ ! -z "$amount" ] && [ "$amount" != "null" ]; then
    test_result 0 "Payment amount specified: $amount"
else
    test_result 1 "Payment amount missing"
fi

# Test 7: Verify payTo address
echo "Test 4.7: Recipient Address"
if cat /tmp/payment-response.json 2>/dev/null | jq -e '.accepts[0].pay_to' > /dev/null 2>&1; then
    test_result 0 "Pay-to address present"
else
    test_result 1 "Pay-to address missing"
fi

# Test 8: Test health endpoint
echo "Test 4.8: Health Endpoint"
health=$(curl -s http://localhost:3402/health)
if echo "$health" | jq -e '.status' | grep -q "healthy"; then
    test_result 0 "Health endpoint working"
else
    test_result 1 "Health endpoint not working"
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
    echo "🎉 All payment flow tests passed!"
    exit 0
else
    echo "⚠️  Some tests failed."
    exit 1
fi

