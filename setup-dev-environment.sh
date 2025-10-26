#!/bin/bash
# Wadah Development Environment Setup
# Run this script to install Rust and verify setup

echo "🦀 Installing Rust for Wadah Development"
echo "=========================================="
echo ""

# Install Rust
echo "📥 Installing Rust..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Source the environment
echo ""
echo "🔧 Setting up environment..."
source $HOME/.cargo/env

# Verify installation
echo ""
echo "✅ Verifying installation..."
rustc --version
cargo --version

# Test the spec crate
echo ""
echo "🧪 Testing Wadah spec crate..."
cd /Users/hsp/Projects/wadah-engine
cargo test -p wadah-spec

echo ""
echo "=========================================="
echo "✅ Setup Complete!"
echo ""
echo "Next steps:"
echo "1. Read: IMPLEMENTATION_PROGRESS.md"
echo "2. Start with: cd crates/pack"
echo "3. Implement: src/builder.rs"
echo ""
echo "Happy coding! 🌊"

