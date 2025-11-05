# x402 Payment Protocol Integration Plan

**Branch**: `feature/x402-payment-integration`  
**Status**: Planning Phase  
**Protocol**: [Coinbase x402](https://github.com/coinbase/x402)

---

## 🎯 Overview

[x402](https://github.com/coinbase/x402) is Coinbase's open-source HTTP payment protocol that enables micropayments for AI agent executions. This integration will allow Wadah agents to:

1. **Charge for agent execution** using blockchain payments
2. **Monetize AI services** with pay-per-use model
3. **Enable agent marketplace** where agents can be run for crypto payments
4. **Support multiple chains** (Ethereum, Base, Polygon, etc.)

---

## 📋 What is x402?

x402 is a payments protocol built on HTTP that uses:
- **HTTP 402 status code** (Payment Required)
- **Standard headers** (`X-PAYMENT`, `X-PAYMENT-RESPONSE`)
- **Blockchain payments** (ERC20 tokens on EVM chains)
- **Facilitator pattern** for verification and settlement
- **Multiple payment schemes** (exact, upto, etc.)

### Protocol Flow

```
1. Client requests AI agent execution
2. Server responds: 402 Payment Required + payment details
3. Client creates blockchain payment signature
4. Client re-requests with X-PAYMENT header
5. Server verifies payment (via facilitator or local)
6. Server executes agent
7. Server settles payment on blockchain
8. Server returns result + X-PAYMENT-RESPONSE
```

---

## 🚀 Integration Possibilities

### Option 1: Agent Execution Payments (Recommended)

**Use Case**: Pay to run an agent
- User wants to run a premium agent
- Agent owner sets price (e.g., 0.01 USDC per run)
- Payment required before execution
- Payment settled after successful execution

**Implementation**:
```rust
// In wadah-cli/src/commands/run.rs
pub async fn execute(
    manifest_path: &str,
    prompt: &str,
    payment_required: bool,  // New flag
) -> Result<()> {
    if payment_required {
        // Check for X-PAYMENT header or return 402
        let payment = verify_x402_payment()?;
    }
    
    // Execute agent...
    let result = executor.run(prompt).await?;
    
    if payment_required {
        // Settle payment and return X-PAYMENT-RESPONSE
        settle_payment(payment)?;
    }
    
    Ok(())
}
```

### Option 2: Token-Based Agent Usage

**Use Case**: Pay per token/output generated
- LLM agents that cost varies by output length
- Pay only for what you use
- Uses x402 "upto" scheme (future)

**Implementation**:
```rust
// Track usage during execution
struct UsageTracker {
    tokens_input: u64,
    tokens_output: u64,
    cost_per_token: u64,
}

// Calculate final cost after execution
let total_cost = tracker.calculate_cost();
settle_variable_payment(total_cost)?;
```

### Option 3: Agent Marketplace with x402

**Use Case**: Decentralized agent marketplace
- Agents listed with prices
- Anyone can run for payment
- Revenue split between agent creator and platform

**Features**:
- List agents with crypto prices
- Browse and pay to run
- Revenue sharing (90% creator, 10% platform)
- On-chain payment history

### Option 4: API Gateway with x402

**Use Case**: Wadah as paid API service
- Expose agents as HTTP APIs
- Require x402 payment for each call
- Rate limiting based on payment tier
- Analytics and billing dashboard

---

## 🏗️ Architecture

### New Components Needed

#### 1. **Payment Middleware** (`wadah-payment` crate)
```rust
pub struct X402Middleware {
    facilitator_url: String,
    supported_schemes: Vec<PaymentScheme>,
    supported_networks: Vec<Network>,
}

impl X402Middleware {
    pub async fn verify_payment(&self, headers: &HeaderMap) -> Result<PaymentVerification>;
    pub async fn settle_payment(&self, payment: Payment) -> Result<SettlementResult>;
    pub fn create_payment_requirements(&self, agent: &Agent) -> PaymentRequirements;
}
```

#### 2. **Payment Configuration in wadah.yaml**
```yaml
name: premium-agent
version: 1.0.0
runtime:
  model: gpt-4
payment:
  enabled: true
  scheme: exact
  networks:
    - base
    - ethereum
  price:
    amount: "10000"  # 0.01 USDC (6 decimals)
    asset: "0x833589fcd6edb6e08f4c7c32d4f71b54bda02913"  # USDC on Base
  payTo: "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb"
  description: "Premium AI agent - GPT-4 powered"
```

#### 3. **Facilitator Integration**
- Use Coinbase's hosted facilitator or self-host
- Handles verification and settlement
- Supports multiple chains (Base, Ethereum, Polygon)

#### 4. **UI Components** (wadah-ui)
```typescript
// Payment Required Dialog
<PaymentDialog
  amount="0.01 USDC"
  network="base"
  onPay={handlePayment}
/>

// Wallet Connection
<WalletConnect
  supportedChains={['base', 'ethereum', 'polygon']}
/>

// Payment History
<PaymentHistory
  transactions={userPayments}
/>
```

---

## 📦 Dependencies

### Rust (wadah-engine)
```toml
[dependencies]
# x402 protocol
x402-rust = "0.1"  # Will need to create or use Go/Python bindings

# Ethereum integration
ethers = "2.0"
alloy = "0.4"

# Payment verification
secp256k1 = "0.27"
```

### TypeScript (wadah-ui)
```json
{
  "dependencies": {
    "@coinbase/x402": "latest",
    "viem": "^2.0.0",
    "wagmi": "^2.0.0",
    "@rainbow-me/rainbowkit": "^2.0.0"
  }
}
```

---

## 🛣️ Implementation Roadmap

### Phase 1: Foundation (Week 1-2)
- [ ] Create `wadah-payment` crate
- [ ] Implement x402 protocol types in Rust
- [ ] Add payment field to `wadah.yaml` spec
- [ ] Basic x402 verification (local only)

### Phase 2: Backend Integration (Week 3-4)
- [ ] Integrate with Coinbase facilitator
- [ ] Add payment middleware to CLI
- [ ] Implement `wadah run --require-payment`
- [ ] Settlement logic with blockchain submission
- [ ] Payment verification before execution

### Phase 3: UI Integration (Week 5-6)
- [ ] Add wallet connection (Rainbow Kit)
- [ ] Payment dialog for premium agents
- [ ] Show payment requirements in agent details
- [ ] Display payment history
- [ ] Transaction status tracking

### Phase 4: Marketplace Features (Week 7-8)
- [ ] List paid agents in Templates
- [ ] Browse by price/network
- [ ] Revenue dashboard for creators
- [ ] Payment analytics
- [ ] Revenue split configuration

### Phase 5: Advanced Features (Week 9-10)
- [ ] Variable pricing (pay per token)
- [ ] Subscription model (monthly payments)
- [ ] Bulk payment discounts
- [ ] Multi-chain support (Polygon, Arbitrum)
- [ ] Payment webhooks

---

## 💡 Use Cases

### 1. **AI Agency as a Service**
- Agencies deploy premium agents
- Charge per execution
- Automated revenue collection
- On-chain payment proof

### 2. **AI Model Marketplace**
- Fine-tuned models as paid agents
- Pay per inference
- Model creators earn revenue
- Transparent pricing

### 3. **Enterprise AI Gateway**
- Company deploys Wadah internally
- Departments pay for AI usage
- Chargeback to business units
- Usage analytics and billing

### 4. **Decentralized AI Platform**
- Public agent marketplace
- Anyone can publish paid agents
- Crypto-native monetization
- No intermediaries

### 5. **LLM Token Billing**
- Pay per token generated
- More accurate pricing
- Lower costs for short queries
- Enterprise-grade billing

---

## 🔐 Security Considerations

### Payment Verification
- ✅ Use EIP-712 signatures for payment proofs
- ✅ Verify signatures before execution
- ✅ Check payment amounts match requirements
- ✅ Prevent replay attacks with nonces

### Settlement
- ✅ Only settle after successful execution
- ✅ Handle failed settlements gracefully
- ✅ Provide refund mechanism
- ✅ Audit trail for all payments

### Privacy
- ✅ No personal data in payment headers
- ✅ Public blockchain transactions
- ✅ Optional: zkPayments for privacy

---

## 📊 Technical Comparison

### x402 vs Traditional Payments

| Feature | x402 | Stripe | Crypto Direct |
|---------|------|--------|---------------|
| **Setup Time** | Minutes | Days | Hours |
| **Global** | ✅ Yes | ⚠️ Limited | ✅ Yes |
| **Fees** | ~$0.01 | 2.9% + $0.30 | Gas fees |
| **Settlement** | Instant | 2-7 days | Minutes |
| **KYC Required** | ❌ No | ✅ Yes | ❌ No |
| **Micropayments** | ✅ Yes | ❌ No | ⚠️ Gas cost |
| **Integration** | HTTP headers | API + Webhooks | Smart contracts |

---

## 🎨 UI Mockups

### Agent Details with Payment
```
┌─────────────────────────────────────┐
│ Premium Support Agent               │
│ ⭐⭐⭐⭐⭐ 4.8 (234 runs)           │
│                                     │
│ 💰 0.01 USDC per run               │
│ 🌐 Networks: Base, Ethereum        │
│                                     │
│ [Connect Wallet] [Run Agent →]     │
└─────────────────────────────────────┘
```

### Payment Dialog
```
┌─────────────────────────────────────┐
│       Payment Required               │
│                                     │
│ Agent: Premium Support Agent        │
│ Price: 0.01 USDC                   │
│ Network: Base                       │
│                                     │
│ Your Balance: 5.42 USDC            │
│                                     │
│ [Cancel] [Pay & Run →]             │
└─────────────────────────────────────┘
```

---

## 🔗 Resources

- **x402 Specification**: https://github.com/coinbase/x402
- **x402 Website**: https://x402.org
- **Coinbase Developer Platform**: https://www.coinbase.com/developer-platform/products/x402
- **Base Network**: https://base.org
- **EIP-3009**: https://eips.ethereum.org/EIPS/eip-3009

---

## 🎯 Success Metrics

### Phase 1 (Foundation)
- [ ] x402 protocol implemented in Rust
- [ ] Payment verification working locally
- [ ] 10+ unit tests passing

### Phase 2 (Backend)
- [ ] First successful paid agent execution
- [ ] Payment settled on Base testnet
- [ ] Integration tests with facilitator

### Phase 3 (UI)
- [ ] Wallet connection working
- [ ] Payment dialog functional
- [ ] 1 complete end-to-end transaction

### Phase 4 (Marketplace)
- [ ] 5+ paid agents published
- [ ] 100+ successful payments
- [ ] Revenue dashboard live

---

## 🤔 Open Questions

1. **Which chain to prioritize?**
   - Base (Coinbase's L2, low fees)
   - Ethereum mainnet (most established)
   - Polygon (alternative L2)

2. **Pricing model?**
   - Fixed per execution
   - Variable by token usage
   - Subscription tiers

3. **Revenue split?**
   - 100% to agent creator
   - 90/10 split (creator/platform)
   - Configurable

4. **Refund policy?**
   - Automatic if execution fails
   - Manual review process
   - No refunds (pre-paid)

5. **Self-hosted facilitator?**
   - Use Coinbase's hosted service
   - Run our own facilitator
   - Hybrid approach

---

## 🚦 Next Steps

1. **Review this plan** with team
2. **Decide on initial use case** (Option 1 recommended)
3. **Set up Base testnet** accounts and faucet
4. **Prototype payment verification** in Rust
5. **Test with x402 TypeScript SDK**
6. **Create first paid agent** as proof of concept

---

**Status**: ✅ Planning Complete - Ready for Development

*Last Updated*: November 5, 2024  
*Author*: Wadah Team  
*Branch*: `feature/x402-payment-integration`

