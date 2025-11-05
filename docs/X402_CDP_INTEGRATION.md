# x402 Integration with Coinbase CDP - Updated Implementation Guide

**Updated**: November 5, 2024  
**Branch**: `feature/x402-payment-integration`  
**Official Resources**: [Coinbase CDP x402 Documentation](https://docs.cdp.coinbase.com/x402)

---

## 🎯 Official Coinbase CDP Resources

### Key Links
- **Main Documentation**: https://docs.cdp.coinbase.com/x402
- **GitHub Repository**: https://github.com/coinbase/x402
- **x402 Website**: https://x402.org
- **CDP Portal**: https://portal.cdp.coinbase.com

### Official Features
✅ **Hosted Facilitator** by Coinbase (production-ready)  
✅ **Built-in KYT Screening** (OFAC, illicit finance checks)  
✅ **Base Network Support** (low fees, fast settlement)  
✅ **TypeScript/JavaScript SDKs** available  
✅ **Self-hosted Facilitator** option for full control

---

## 📦 Official NPM Packages

### For wadah-ui (Electron + React)

```bash
# Client-side x402 support
npm install @coinbase/x402-fetch

# Wallet connection (OnchainKit)
npm install @coinbase/onchainkit

# Wagmi for Ethereum interaction
npm install wagmi viem @tanstack/react-query

# Optional: CDP SDK for backend
npm install @coinbase/coinbase-sdk
```

---

## 🏗️ Updated Architecture

### Coinbase CDP Facilitator
Instead of building our own facilitator, we'll use Coinbase's hosted service:

**Production Facilitator**:
- URL: `https://facilitator.x402.coinbase.com` (assumed)
- Features: Automatic verification, settlement, KYT screening
- Networks: Base, Ethereum mainnet
- No self-hosting required

**Testnet Facilitator**:
- URL: Base Sepolia testnet
- Use for development and testing

### Integration Points

1. **Wadah Engine (Rust)**
   - Keep our `wadah-payment` crate for Rust-side types
   - HTTP client to call Coinbase facilitator API
   - Settlement tracking and verification

2. **Wadah UI (Electron + React)**
   - Use `@coinbase/x402-fetch` for client-side payments
   - OnchainKit for wallet connection
   - CDP API integration for backend

3. **Wadah CLI**
   - `wadah serve` command to start x402-enabled server
   - Payment verification via Coinbase facilitator
   - Settlement hooks

---

## 🚀 Implementation Plan (Updated)

### Phase 1: ✅ Foundation (Completed)
- [x] Create `wadah-payment` crate with x402 types
- [x] Add payment field to wadah.yaml spec
- [x] Implement basic local verification
- [x] Add `wadah serve` CLI command

### Phase 2: Backend Integration (In Progress)

#### 2.1: Coinbase Facilitator Integration
```rust
// Update facilitator.rs to use Coinbase's hosted service
pub const COINBASE_FACILITATOR_PROD: &str = "https://facilitator.x402.coinbase.com";
pub const COINBASE_FACILITATOR_TEST: &str = "https://facilitator-sepolia.x402.coinbase.com";

impl FacilitatorClient {
    pub fn coinbase_production() -> Self {
        Self::new(COINBASE_FACILITATOR_PROD)
    }
    
    pub fn coinbase_testnet() -> Self {
        Self::new(COINBASE_FACILITATOR_TEST)
    }
}
```

#### 2.2: CDP API Integration
- Get CDP API credentials from portal.cdp.coinbase.com
- Store in environment: `CDP_API_KEY_ID`, `CDP_API_KEY_SECRET`
- Use for backend operations

#### 2.3: Settlement with Base Network
```rust
// Settlement on Base (low fees)
pub async fn settle_on_base(
    payment: &ExactPaymentData,
    facilitator: &FacilitatorClient,
) -> Result<SettlementResponse> {
    // Coinbase facilitator handles the actual blockchain submission
    facilitator.settle(payment_header, requirements).await
}
```

### Phase 3: UI Integration

#### 3.1: Install Dependencies
```bash
cd /Users/hsp/Projects/wadah-ui
npm install @coinbase/x402-fetch @coinbase/onchainkit wagmi viem @tanstack/react-query
```

#### 3.2: Wallet Connection Component
```typescript
// Use OnchainKit's ConnectWallet
import { ConnectWallet } from '@coinbase/onchainkit/wallet';
import { base } from 'wagmi/chains';

export function WalletConnect() {
  return (
    <ConnectWallet
      withWalletAggregator
      chain={base}
    />
  );
}
```

#### 3.3: x402 Payment Flow
```typescript
import { x402Fetch } from '@coinbase/x402-fetch';

// Make payment request
const response = await x402Fetch('/api/agent/run', {
  method: 'POST',
  body: JSON.stringify({ prompt: 'Hello' }),
  wallet: walletClient, // from wagmi
});
```

### Phase 4: Marketplace Features
- Agent listings with prices
- Browse by price/network
- Revenue dashboard using CDP analytics
- Payment webhooks

### Phase 5: Advanced Features
- Subscription payments
- Multi-chain support (Ethereum, Polygon)
- Bulk payment discounts
- CDP Staking integration

---

## 🔑 Environment Variables (Updated)

### wadah-engine (.env)
```bash
# Coinbase CDP credentials
CDP_API_KEY_ID=your_key_id
CDP_API_KEY_SECRET=your_key_secret

# Facilitator (use Coinbase hosted)
X402_FACILITATOR_URL=https://facilitator.x402.coinbase.com

# Wallet to receive payments
WADAH_PAYMENT_ADDRESS=0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb

# Network (base, ethereum)
X402_NETWORK=base
```

### wadah-ui (.env.local)
```bash
# OnchainKit
NEXT_PUBLIC_ONCHAINKIT_API_KEY=your_api_key
NEXT_PUBLIC_ONCHAINKIT_PROJECT_NAME=Wadah Desktop

# CDP API
CDP_API_KEY_ID=your_key_id
CDP_API_KEY_SECRET=your_key_secret

# Payment configuration
VITE_X402_ENABLED=true
VITE_X402_NETWORK=base
```

---

## 💰 Pricing Models

### Option 1: Per-Execution (Recommended)
```yaml
payment:
  enabled: true
  scheme: exact
  networks: [base]
  price:
    amount: "10000"  # 0.01 USDC
    asset: "0x833589fcd6edb6e08f4c7c32d4f71b54bda02913"  # USDC on Base
```

### Option 2: Per-Token (Future)
```yaml
payment:
  enabled: true
  scheme: upto
  networks: [base]
  price:
    max_amount: "100000"  # Max 0.1 USDC
    per_token: "100"  # 0.0001 USDC per token
    asset: "0x833589fcd6edb6e08f4c7c32d4f71b54bda02913"
```

---

## 🧪 Testing Strategy

### 1. Local Testing (Testnet)
```bash
# Use Base Sepolia for testing
export X402_NETWORK=base-sepolia
export X402_FACILITATOR_URL=https://facilitator-sepolia.x402.coinbase.com

# Get testnet USDC from faucet
wadah serve examples/premium-agent-with-payment.yaml --testnet
```

### 2. Production Testing
```bash
# Use Base mainnet with small amounts
export X402_NETWORK=base
wadah serve examples/premium-agent-with-payment.yaml
```

---

## 📊 Benefits of Using Coinbase CDP

| Feature | DIY Facilitator | Coinbase Facilitator |
|---------|----------------|---------------------|
| **Setup Time** | Days | Minutes |
| **KYT Screening** | Manual | ✅ Built-in |
| **Maintenance** | Your responsibility | Coinbase managed |
| **Compliance** | DIY | ✅ OFAC compliant |
| **Monitoring** | Build yourself | ✅ CDP Dashboard |
| **Support** | None | Coinbase support |
| **Cost** | Infrastructure cost | Transaction fees only |

---

## 🎯 Next Steps

1. **Get CDP Credentials**
   - Visit https://portal.cdp.coinbase.com
   - Create API keys
   - Configure in environment

2. **Update Facilitator Client**
   - Point to Coinbase hosted facilitator
   - Test verification endpoint
   - Test settlement endpoint

3. **Install UI Dependencies**
   - Add @coinbase/x402-fetch
   - Add @coinbase/onchainkit
   - Configure wallet connection

4. **Test End-to-End**
   - Create paid agent
   - Serve with `wadah serve`
   - Test payment from UI
   - Verify settlement on Base

---

## 📚 Official Documentation References

- **x402 Quickstart**: https://docs.cdp.coinbase.com/x402/quickstart-for-buyers
- **Network Support**: https://docs.cdp.coinbase.com/x402/network-support
- **Wallet Guide**: https://docs.cdp.coinbase.com/x402/docs/wallet
- **Mini Apps Tutorial**: https://docs.cdp.coinbase.com/x402/miniapps

---

**Status**: ✅ Foundation Complete, Moving to CDP Integration

*Last Updated*: November 5, 2024  
*Next*: Integrate Coinbase hosted facilitator and OnchainKit

