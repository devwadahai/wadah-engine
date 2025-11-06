# x402 & Coinbase Integration Test Suite

This directory contains all tests related to x402 payment protocol and Coinbase Developer Platform integration.

## 📁 Test Structure

```
tests/x402/
├── README.md                      # This file
├── 01_setup_test.sh              # Test CDP setup and configuration
├── 02_cdp_client_test.sh         # Test CDP API client
├── 03_facilitator_test.sh        # Test facilitator communication
├── 04_payment_flow_test.sh       # Test end-to-end payment flow
├── 05_usdc_transfer_test.sh      # Test USDC transfers on Base
├── 06_serve_command_test.sh      # Test wadah serve with payments
├── 07_analytics_test.sh          # Test CDP analytics integration
├── integration/                   # Integration tests
│   ├── payment_integration_test.rs
│   ├── facilitator_integration_test.rs
│   └── cdp_analytics_test.rs
└── fixtures/                      # Test fixtures
    ├── test_payment_request.json
    ├── test_payment_response.json
    └── test_agent_manifest.yaml
```

## 🧪 Test Categories

### 1. Setup & Configuration Tests
- CDP API key validation
- Environment variable configuration
- Wallet address validation
- Network configuration

### 2. CDP Client Tests
- API authentication
- HTTP request/response handling
- Error handling
- Rate limiting

### 3. Facilitator Tests
- Payment requirement generation
- Payment verification
- Signature validation
- Network switching (Sepolia/Mainnet)

### 4. Payment Flow Tests
- Complete payment flow from UI to backend
- Transaction submission
- Confirmation handling
- Error scenarios

### 5. USDC Transfer Tests
- Balance checks
- Transfer execution
- Gas estimation
- Transaction receipts

### 6. Server Tests
- wadah serve command
- 402 Payment Required responses
- X-PAYMENT header handling
- Agent execution after payment

### 7. Analytics Tests
- Event tracking
- Metrics collection
- Dashboard integration
- Revenue reporting

## 🚀 Running Tests

### All Tests
```bash
./run_all_tests.sh
```

### Individual Test Suites
```bash
# Setup tests
./01_setup_test.sh

# CDP client tests
./02_cdp_client_test.sh

# Payment flow tests
./04_payment_flow_test.sh
```

### Rust Integration Tests
```bash
cd /Users/hsp/Projects/wadah-engine
cargo test --test x402_integration
cargo test --test cdp_client
```

### TypeScript Tests (UI)
```bash
cd /Users/hsp/Projects/wadah-ui
npm test -- x402
```

## 📋 Test Checklist

### Prerequisites
- [ ] CDP API key configured
- [ ] Base Sepolia RPC access
- [ ] Test wallet with ETH
- [ ] Test wallet with USDC
- [ ] wadah-engine compiled
- [ ] wadah-ui dependencies installed

### Backend Tests (Rust)
- [ ] CDP client initialization
- [ ] API key authentication
- [ ] Facilitator communication
- [ ] Payment verification
- [ ] Signature validation
- [ ] Error handling
- [ ] Timeout handling

### Frontend Tests (TypeScript)
- [ ] Wallet connection
- [ ] USDC balance checks
- [ ] Payment dialog UI
- [ ] Transaction signing
- [ ] Confirmation handling
- [ ] Error messages
- [ ] Revenue dashboard

### Integration Tests
- [ ] End-to-end payment flow
- [ ] Testnet transactions
- [ ] Analytics tracking
- [ ] Multi-agent scenarios
- [ ] Network switching
- [ ] Error recovery

## 🔧 Test Configuration

### Environment Variables
```bash
# For tests
export TEST_MODE=true
export CDP_API_KEY_ID=your-test-key
export CDP_PRIVATE_KEY=your-test-private-key
export X402_FACILITATOR_URL=https://facilitator.x402.coinbase.com
export X402_NETWORK=base-sepolia
export TEST_WALLET_ADDRESS=0xYourTestWalletAddress
export TEST_PRIVATE_KEY=your-test-private-key
```

### Test Fixtures
Located in `fixtures/` directory:
- Sample payment requests
- Mock API responses
- Test agent manifests
- Expected outputs

## 📊 Test Coverage

Target coverage: **80%+**

### Current Coverage
- [ ] CDP Client: _%
- [ ] Facilitator: _%
- [ ] Payment Verification: _%
- [ ] USDC Transfers: _%
- [ ] Server Endpoints: _%
- [ ] Analytics: _%

## 🐛 Known Issues & Workarounds

### Issue 1: Rate Limiting
- **Problem**: CDP API rate limits during tests
- **Workaround**: Add delays between requests
- **Status**: Monitored

### Issue 2: Testnet Availability
- **Problem**: Base Sepolia RPC sometimes slow
- **Workaround**: Retry with backoff
- **Status**: Ongoing

## 📚 References

- **x402 Protocol**: https://github.com/coinbase/x402
- **CDP Docs**: https://docs.cdp.coinbase.com
- **Base Network**: https://docs.base.org
- **USDC Contract**: https://www.circle.com/en/usdc

## 🤝 Contributing

When adding new tests:
1. Follow naming convention: `NN_description_test.sh`
2. Add test to this README
3. Update test checklist
4. Document expected outcomes
5. Include error scenarios

## 📝 Test Results

Latest test run: [Date]
- Total Tests: X
- Passed: X
- Failed: X
- Skipped: X
- Coverage: X%

---

**Last Updated**: Auto-generated during setup

