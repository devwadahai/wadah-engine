#!/bin/bash

echo "🔑 CDP Configuration Helper"
echo "==========================="
echo ""
echo "This will help you set up Coinbase Developer Platform credentials."
echo ""
echo "📋 Before you start, make sure you have:"
echo "   1. CDP Account at https://portal.cdp.coinbase.com"
echo "   2. Created a project"
echo "   3. Generated API key"
echo ""

read -p "Have you completed the above? (y/n): " ready
if [[ ! $ready =~ ^[Yy]$ ]]; then
    echo ""
    echo "👉 Go to: https://portal.cdp.coinbase.com"
    echo "   1. Sign up / Sign in"
    echo "   2. Create a project"
    echo "   3. Go to API Keys → Create API Key"
    echo "   4. Save the credentials"
    echo "   5. Run this script again"
    echo ""
    exit 1
fi

echo ""
echo "Great! Let's configure wadah-engine..."
echo ""

# Check if .env exists
ENV_FILE="/Users/hsp/Projects/wadah-engine/.env"
if [ -f "$ENV_FILE" ]; then
    echo "⚠️  .env file already exists."
    read -p "Overwrite it? (y/n): " overwrite
    if [[ ! $overwrite =~ ^[Yy]$ ]]; then
        echo "Aborted. Edit $ENV_FILE manually."
        exit 0
    fi
fi

echo ""
echo "Enter your CDP credentials:"
echo ""

# Prompt for API Key Name
read -p "CDP API Key Name (e.g., wadah-prod): " api_key_name

# Prompt for Private Key
echo ""
echo "Paste your Private Key (multi-line, press Ctrl+D when done):"
echo "Format:"
echo "-----BEGIN EC PRIVATE KEY-----"
echo "..."
echo "-----END EC PRIVATE KEY-----"
echo ""
private_key=$(cat)

# Prompt for wallet address
echo ""
read -p "Your wallet address (to receive payments): " wallet_address

# Prompt for network
echo ""
echo "Choose network:"
echo "  1) base-sepolia (testnet - recommended for testing)"
echo "  2) base (mainnet - for production)"
read -p "Enter choice (1 or 2): " network_choice

if [ "$network_choice" == "1" ]; then
    network="base-sepolia"
elif [ "$network_choice" == "2" ]; then
    network="base"
else
    echo "Invalid choice. Defaulting to base-sepolia"
    network="base-sepolia"
fi

# Create .env file
cat > "$ENV_FILE" << EOF
# Coinbase Developer Platform Configuration
# Generated: $(date)

# CDP API Credentials
CDP_API_KEY_NAME=$api_key_name
CDP_API_KEY_PRIVATE_KEY=$private_key

# x402 Facilitator Configuration
X402_FACILITATOR_URL=https://facilitator.x402.coinbase.com
X402_NETWORK=$network

# Payment Configuration
WADAH_PAYMENT_ADDRESS=$wallet_address

# Optional: Enable debug logging
# RUST_LOG=debug
EOF

echo ""
echo "✅ Configuration saved to: $ENV_FILE"
echo ""
echo "🧪 Test your configuration:"
echo ""
echo "   cd /Users/hsp/Projects/wadah-engine"
echo "   cargo run -- serve examples/premium-agent-with-payment.yaml"
echo ""
echo "You should see:"
echo "   ✓ CDP API Key loaded"
echo "   ✓ Facilitator URL: https://facilitator.x402.coinbase.com"
echo "   ✓ Server started on http://0.0.0.0:3402"
echo ""
echo "🔐 Security reminder:"
echo "   • .env is in .gitignore (won't be committed)"
echo "   • Keep your private key secure"
echo "   • Never share it publicly"
echo ""

