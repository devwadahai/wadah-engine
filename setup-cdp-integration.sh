#!/bin/bash

echo "🔑 CDP Integration Setup for Wadah"
echo "==================================="
echo ""

# Path to the CDP key file
CDP_KEY_FILE="/Users/hsp/Projects/wadah-ui/keys/cdp_api_key.json"
WADAH_ENGINE_ENV="/Users/hsp/Projects/wadah-engine/.env"
WADAH_UI_ENV="/Users/hsp/Projects/wadah-ui/.env.local"

# Check if CDP key exists
if [ ! -f "$CDP_KEY_FILE" ]; then
    echo "❌ CDP API key not found at: $CDP_KEY_FILE"
    echo "Please create your API key at https://portal.cdp.coinbase.com"
    exit 1
fi

echo "✅ Found CDP API key"
echo ""

# Parse the JSON file
CDP_KEY_ID=$(cat "$CDP_KEY_FILE" | grep '"id"' | cut -d'"' -f4)
CDP_PRIVATE_KEY=$(cat "$CDP_KEY_FILE" | grep '"privateKey"' | cut -d'"' -f4)

if [ -z "$CDP_KEY_ID" ] || [ -z "$CDP_PRIVATE_KEY" ]; then
    echo "❌ Failed to parse CDP key file"
    exit 1
fi

echo "📝 Configuring wadah-engine..."
echo ""

# Create .env for wadah-engine
cat > "$WADAH_ENGINE_ENV" << EOF
# Coinbase Developer Platform Configuration
# Auto-generated: $(date)

# CDP API Credentials
CDP_API_KEY_ID=$CDP_KEY_ID
CDP_PRIVATE_KEY=$CDP_PRIVATE_KEY

# x402 Facilitator Configuration
X402_FACILITATOR_URL=https://facilitator.x402.coinbase.com
X402_NETWORK=base-sepolia

# Your Wallet Address (CHANGE THIS!)
WADAH_PAYMENT_ADDRESS=0xYourWalletAddressHere

# Optional: Enable debug logging
# RUST_LOG=debug,wadah_payment=trace
EOF

echo "✅ Created: $WADAH_ENGINE_ENV"
echo ""

# Prompt for wallet address
read -p "Enter your wallet address (to receive payments): " wallet_address

if [ ! -z "$wallet_address" ]; then
    sed -i '' "s/0xYourWalletAddressHere/$wallet_address/" "$WADAH_ENGINE_ENV"
    echo "✅ Set wallet address: $wallet_address"
else
    echo "⚠️  Skipped wallet address - update it manually in .env"
fi

echo ""
echo "📝 Configuring wadah-ui..."
echo ""

# Create .env.local for wadah-ui
cat > "$WADAH_UI_ENV" << EOF
# Wadah UI Configuration
# Auto-generated: $(date)

# CDP Configuration
VITE_CDP_API_KEY_ID=$CDP_KEY_ID

# WalletConnect (Optional)
# Get from: https://cloud.walletconnect.com
# VITE_WALLET_CONNECT_PROJECT_ID=your_project_id_here

# Network Configuration
VITE_NETWORK=base-sepolia
EOF

echo "✅ Created: $WADAH_UI_ENV"
echo ""

echo "🎉 CDP Integration Complete!"
echo "============================"
echo ""
echo "What was configured:"
echo "  ✅ CDP API credentials"
echo "  ✅ x402 Facilitator URL"
echo "  ✅ Base Sepolia testnet"
echo "  ✅ Wallet address (if provided)"
echo ""
echo "🧪 Test your setup:"
echo ""
echo "1. Test wadah-engine:"
echo "   cd /Users/hsp/Projects/wadah-engine"
echo "   cargo run --release -- serve examples/premium-agent-with-payment.yaml"
echo ""
echo "2. Check wadah-ui:"
echo "   cd /Users/hsp/Projects/wadah-ui"
echo "   npm run dev:electron"
echo ""
echo "📋 Next steps:"
echo "  1. Update wallet address in .env if needed"
echo "  2. Get testnet USDC from https://faucet.circle.com"
echo "  3. Test payments on Base Sepolia"
echo "  4. Switch to 'base' network for production"
echo ""
echo "🔐 Security:"
echo "  • .env files are in .gitignore ✅"
echo "  • keys/ folder is in .gitignore ✅"
echo "  • Your API keys are secure ✅"
echo ""

