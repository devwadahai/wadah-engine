# 🔑 Updated CDP Setup Guide (2024/2025)

## What You See in CDP Portal

After signing up at https://portal.cdp.coinbase.com, you'll see:

### 1. Create API Key
- Your API key allows you to use any product on the platform
- Generates credentials for backend API calls

### 2. Install SDK
- SDK for backend (Node.js, Python, etc.)
- SDK for frontend (React, etc.)

---

## 📋 Step-by-Step Setup

### Step 1: Create API Key

1. **In CDP Portal, click "Create API Key"**
   
2. **Fill in details:**
   - **Name**: `wadah-payments`
   - **Description**: `AI agent payments with x402`
   
3. **Save these credentials** (shown once!):
   ```
   API Key Name: wadah-payments
   API Key ID: xxx-xxx-xxx
   API Secret: xxxxx (or download JSON)
   ```

4. **Copy the credentials** - you can't see them again!

---

### Step 2: For Wadah Backend (Rust)

Since Wadah uses **Rust** (not Node.js), we'll use the x402 protocol directly:

#### Option A: Use Direct HTTP Calls (Recommended)

Our current implementation already does this! No SDK needed.

The `wadah-payment` crate uses:
- Direct HTTP calls to facilitator
- No CDP SDK dependency
- Works with any facilitator

#### Option B: Add CDP API Support (Optional)

If you want CDP-specific features, create `.env`:

```bash
# /Users/hsp/Projects/wadah-engine/.env

# Basic Setup (No CDP API Key needed for basic payments!)
X402_FACILITATOR_URL=https://facilitator.x402.coinbase.com
X402_NETWORK=base-sepolia  # or 'base' for mainnet
WADAH_PAYMENT_ADDRESS=0xYourWalletAddress

# Optional: CDP API Key (for advanced features)
CDP_API_KEY_ID=your-api-key-id
CDP_API_SECRET=your-api-secret
```

---

### Step 3: For Wadah UI (React)

The UI uses **wagmi** and **viem**, which work without CDP SDK!

#### What's Already Working:
✅ Wallet connection (Coinbase Wallet, MetaMask)
✅ USDC payments on Base
✅ Transaction signing
✅ Payment verification

#### Optional: Add OnchainKit

OnchainKit is Coinbase's React component library:

1. **In CDP Portal**, under "Install SDK", get:
   - **Public API Key** for OnchainKit

2. **Add to Wadah UI:**

```bash
# /Users/hsp/Projects/wadah-ui/.env.local
VITE_ONCHAINKIT_API_KEY=your_public_key_here
```

3. **Or use Settings UI:**
   - Go to Settings → Web3 & Payment Configuration
   - (We can add OnchainKit field if needed)

---

## 🎯 What You Actually Need

### For Basic x402 Payments:
```
✅ Wallet address (to receive payments)
✅ That's it!
```

### For CDP-Enhanced Features:
```
⚠️ CDP API Key (optional)
   • Better analytics
   • Facilitator integration
   • OnchainKit components
```

---

## 🧪 Test Without CDP API Key

You can test **right now** without any CDP API key:

```bash
cd /Users/hsp/Projects/wadah-engine

# Create simple .env
cat > .env << EOF
X402_FACILITATOR_URL=https://facilitator.x402.coinbase.com
X402_NETWORK=base-sepolia
WADAH_PAYMENT_ADDRESS=0xYourWalletAddressHere
EOF

# Run server
cargo run -- serve examples/premium-agent-with-payment.yaml
```

This works because:
- x402 is an **open protocol**
- Payments go directly wallet-to-wallet
- No middleman required
- CDP API is optional enhancement

---

## 📊 CDP API vs No CDP API

| Feature | Without CDP API | With CDP API |
|---------|----------------|--------------|
| **Basic Payments** | ✅ Works | ✅ Works |
| **USDC Transfers** | ✅ Direct blockchain | ✅ Direct blockchain |
| **Wallet Connection** | ✅ wagmi/viem | ✅ wagmi/viem |
| **Payment Verification** | ✅ On-chain | ✅ Facilitator service |
| **Analytics** | ❌ DIY | ✅ CDP Dashboard |
| **OnchainKit UI** | ❌ Not available | ✅ Available |
| **Support** | ❌ Community | ✅ Coinbase support |

---

## 🚀 Recommended Approach

### Phase 1: Test Without CDP (Now)
1. Use your wallet address in `wadah.yaml`
2. Test payments on Base Sepolia
3. Verify everything works
4. **No CDP API key needed!**

### Phase 2: Add CDP Later (Optional)
1. Create CDP API key
2. Add to `.env`
3. Get enhanced analytics
4. Use OnchainKit components

---

## 💡 Quick Setup

**Minimal setup to test payments right now:**

```bash
# 1. Add your wallet address to agent manifest
cd /Users/hsp/Projects/wadah-engine
nano examples/premium-agent-with-payment.yaml

# Change this line:
# payTo: "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb"
# To your address:
# payTo: "0xYourWalletAddress"

# 2. Start server (no .env needed!)
cargo run -- serve examples/premium-agent-with-payment.yaml

# 3. Test it
curl -X POST http://localhost:3402/agent/premium-support-agent/run \
  -H "Content-Type: application/json" \
  -d '{"prompt": "Hello"}'
```

**That's it!** Payments work without any CDP API key.

---

## 🔐 When to Use CDP API Key

Use CDP API key if you want:

1. **Analytics Dashboard** - See payment metrics in CDP Portal
2. **Facilitator Service** - Automated payment verification
3. **OnchainKit** - Pre-built React components
4. **Support** - Access to Coinbase developer support
5. **Scale** - Better rate limits and infrastructure

But for **basic x402 payments**, you don't need it!

---

## 📞 Next Steps

**Choose your path:**

### Path A: Test Now (No CDP Key)
1. Set your wallet address in `wadah.yaml`
2. Run `wadah serve`
3. Test payments
4. Done! ✅

### Path B: Full CDP Integration
1. Create API key in CDP Portal
2. Add to `.env`
3. Configure facilitator
4. Get enhanced features

---

**Which path do you want to take?**

