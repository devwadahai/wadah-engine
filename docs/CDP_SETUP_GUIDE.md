# Coinbase Developer Platform (CDP) Setup Guide

## 📋 Prerequisites

1. **Coinbase Account**: https://www.coinbase.com/signup
2. **CDP Portal Access**: https://portal.cdp.coinbase.com

## 🔑 Step-by-Step Setup

### 1. Register for CDP

1. Go to: **https://portal.cdp.coinbase.com**
2. Click **"Sign Up"** or **"Get Started"**
3. Sign in with Coinbase account
4. Complete developer verification
5. Accept terms

### 2. Create a Project

1. In CDP Portal, click **"Create Project"**
2. Fill in:
   - **Project Name**: `wadah-payments`
   - **Description**: `AI agent payments with x402`
   - **Use Case**: Web3 Application
3. Click **"Create"**

### 3. Generate API Key

1. Go to **"API Keys"** in your project
2. Click **"Create API Key"**
3. Choose:
   - **Key Type**: Server
   - **Permissions**: Enable x402, OnchainKit
4. **IMPORTANT**: Save these securely:
   ```
   API Key Name: wadah-prod
   API Key ID: organizations/xxx/apiKeys/xxx
   Private Key: (Download JSON - can't retrieve later!)
   ```

### 4. Configure Wadah Engine

Create `/Users/hsp/Projects/wadah-engine/.env`:

```bash
# CDP API Credentials
CDP_API_KEY_NAME=wadah-prod
CDP_API_KEY_PRIVATE_KEY=-----BEGIN EC PRIVATE KEY-----
YOUR_PRIVATE_KEY_HERE
-----END EC PRIVATE KEY-----

# x402 Facilitator
X402_FACILITATOR_URL=https://facilitator.x402.coinbase.com
X402_NETWORK=base-sepolia  # Use 'base' for mainnet

# Your Wallet Address (where you receive payments)
WADAH_PAYMENT_ADDRESS=0xYourWalletAddressHere
```

### 5. Configure Wadah UI

Create `/Users/hsp/Projects/wadah-ui/.env.local`:

```bash
# OnchainKit API Key (optional, for UI features)
VITE_ONCHAINKIT_API_KEY=your_onchainkit_key

# WalletConnect Project ID (optional, for mobile wallets)
VITE_WALLET_CONNECT_PROJECT_ID=your_wc_project_id
```

Or use the **Settings UI** in Wadah Desktop:
1. Go to **Settings**
2. Scroll to **"Web3 & Payment Configuration"**
3. Enter your WalletConnect Project ID
4. Click **"Save Web3 Config"**

## 🧪 Test CDP Integration

### Test 1: Verify API Connection

```bash
cd /Users/hsp/Projects/wadah-engine

# Load .env and test
cargo run -- serve examples/premium-agent-with-payment.yaml
```

You should see:
```
✓ CDP API Key loaded
✓ Facilitator URL: https://facilitator.x402.coinbase.com
✓ Server started on http://0.0.0.0:3402
```

### Test 2: Test Payment Flow

```bash
# Without payment (should get 402)
curl -X POST http://localhost:3402/agent/premium-support-agent/run \
  -H "Content-Type: application/json" \
  -d '{"prompt": "Hello"}'

# Should return:
# {
#   "x402_version": "1.0",
#   "accepts": [...]
# }
```

## 📚 Useful Links

- **CDP Portal**: https://portal.cdp.coinbase.com
- **CDP Docs**: https://docs.cdp.coinbase.com
- **x402 Protocol**: https://docs.cdp.coinbase.com/x402
- **OnchainKit**: https://onchainkit.xyz
- **API Key Guide**: https://docs.cdp.coinbase.com/get-started/docs/api-keys

## 🔐 Security Best Practices

1. **Never commit `.env` files** to git
2. **Store private keys securely**
3. **Use different keys for dev/prod**
4. **Rotate keys periodically**
5. **Limit key permissions** to only what's needed

## ⚠️ Important Notes

- **Free Tier**: CDP has a generous free tier for testing
- **Testnet First**: Always test on Base Sepolia before mainnet
- **Private Key**: Download and save immediately - can't retrieve later!
- **Rate Limits**: Be aware of API rate limits in production

## 🎯 What CDP Enables

With CDP configured, you get:

✅ **Payment Verification**: Automatic verification via facilitator  
✅ **Analytics**: Track payment metrics  
✅ **Better UX**: Smoother payment flows  
✅ **Support**: Access to Coinbase developer support  
✅ **OnchainKit**: UI components for Web3  

## 🚀 Next Steps

After setup:

1. ✅ Test on Base Sepolia testnet
2. ✅ Verify payments work end-to-end
3. ✅ Check analytics in CDP Portal
4. ✅ Deploy to production (Base mainnet)

---

**Questions?** Check the CDP docs or let me know!

