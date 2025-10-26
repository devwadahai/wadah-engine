# Wadah Development Setup Guide

**For**: macOS (darwin 24.5.0)  
**Goal**: Get ready to implement Wadah v0.1

---

## 🔧 Prerequisites Installation

### 1. Install Rust & Cargo

```bash
# Install rustup (Rust toolchain installer)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Follow prompts (usually just press Enter for defaults)
# This installs:
# - rustc (Rust compiler)
# - cargo (package manager)
# - rustup (toolchain manager)

# Restart shell or run:
source $HOME/.cargo/env

# Verify installation
rustc --version  # Should show: rustc 1.75+ 
cargo --version  # Should show: cargo 1.75+
```

### 2. Install Build Tools (if needed)

```bash
# Xcode Command Line Tools (if not already installed)
xcode-select --install

# Or install full Xcode from App Store
```

### 3. Install Additional Dependencies

```bash
# For OpenSSL (required by some Rust crates)
brew install openssl pkg-config

# For compression (zstd)
brew install zstd
```

---

## 🚀 Quick Start (After Rust Installation)

### 1. Verify Setup

```bash
cd /Users/hsp/Projects/wadah-engine

# Build spec crate (should work immediately)
cargo build -p wadah-spec

# Run tests
cargo test -p wadah-spec

# If successful, you'll see:
# ✓ All tests passed
```

### 2. Start Implementation

```bash
# Start with pack crate
cd crates/pack

# Try building (will show what needs implementation)
cargo build

# Run tests (will fail until implemented)
cargo test
```

---

## 📋 Development Workflow

### Build Commands

```bash
# Build specific crate
cargo build -p wadah-spec
cargo build -p wadah-pack
cargo build -p wadah-runtime

# Build entire workspace
cargo build --workspace

# Build with optimizations (release)
cargo build --release
```

### Test Commands

```bash
# Test specific crate
cargo test -p wadah-spec

# Test entire workspace
cargo test --workspace

# Test with output
cargo test -- --nocapture

# Test specific test
cargo test test_parse_minimal_spec
```

### Lint & Format

```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Fix linter warnings automatically
cargo clippy --fix
```

### Watch Mode (Auto-rebuild on save)

```bash
# Install cargo-watch
cargo install cargo-watch

# Watch and rebuild
cargo watch -x build

# Watch and test
cargo watch -x test
```

---

## 🎯 Implementation Order (Follow This)

### Week 1: Pack Crate

**Day 1-2: Builder**
```bash
cd crates/pack
# Edit: src/builder.rs
# Implement: WadahPackageBuilder

# Test as you go:
cargo test test_build_package
```

**Day 3: Extractor**
```bash
# Edit: src/extractor.rs
# Implement: WadahPackageExtractor

cargo test test_extract_package
```

**Day 4: Integrity**
```bash
# Edit: src/integrity.rs
# Implement: Digest computation

cargo test test_compute_digest
```

**Day 5: Integration**
```bash
# Full pack crate should work
cargo test -p wadah-pack

# Test .wpkg creation
cargo run -p wadah-cli -- pack -m templates/hello-world/wadah.yaml -o test.wpkg
```

### Week 2: Runtime & Trace Crates

**Day 6-7: Model Adapters**
```bash
cd crates/runtime
# Edit: src/adapters/openai.rs, ollama.rs

cargo test -p wadah-runtime
```

**Day 8-9: Executor & Trace**
```bash
# Edit: src/executor.rs
# Edit: ../trace/src/recorder.rs

cargo test -p wadah-runtime
cargo test -p wadah-trace
```

**Day 10: Integration**
```bash
# Test runtime + trace together
cargo test --workspace
```

### Week 3: OCI & CLI

**Day 11-12: OCI Distribution**
```bash
cd crates/oci
# Edit: src/client.rs, reference.rs

cargo test -p wadah-oci
```

**Day 13-15: CLI Commands**
```bash
cd crates/cli
# Implement all commands

cargo test -p wadah-cli
```

### Week 4: Polish & Release

**Day 16-20: Testing & Polish**
```bash
# Full workspace tests
cargo test --workspace

# Integration tests
./scripts/test-e2e.sh

# Build release
cargo build --release

# Binary at: target/release/wadah
```

---

## 🐛 Common Issues & Solutions

### Issue: OpenSSL linking errors
```bash
# Solution:
brew install openssl
export OPENSSL_DIR=$(brew --prefix openssl)
cargo clean
cargo build
```

### Issue: zstd not found
```bash
# Solution:
brew install zstd
export ZSTD_SYS_USE_PKG_CONFIG=1
cargo clean
cargo build
```

### Issue: Slow compilation
```bash
# Solution: Use cranelift (faster debug builds)
cat >> ~/.cargo/config.toml <<EOF
[profile.dev]
codegen-backend = "cranelift"
EOF
```

### Issue: Out of disk space
```bash
# Clean build artifacts
cargo clean

# Remove old build cache
rm -rf ~/.cargo/registry/cache
```

---

## 🧪 Testing Strategy

### Unit Tests (Per Module)

```rust
// In each .rs file
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        // Test code
        assert_eq!(2 + 2, 4);
    }
}
```

Run: `cargo test`

### Integration Tests (Cross-Module)

```bash
# Create: crates/pack/tests/integration_test.rs
cd crates/pack
mkdir tests
# Write integration tests

cargo test --test integration_test
```

### End-to-End Tests

```bash
# Create: tests/e2e/
# Full workflow tests

cargo test --test e2e
```

---

## 📊 Progress Tracking

### Check What's Done

```bash
# See all TODO/FIXME comments
rg "TODO|FIXME" --type rust

# Count lines of code
tokei

# Check test coverage (after installing tarpaulin)
cargo install cargo-tarpaulin
cargo tarpaulin
```

### Git Workflow

```bash
# Create feature branch
git checkout -b feat/implement-pack-crate

# Commit frequently
git add crates/pack/src/builder.rs
git commit -m "feat(pack): implement WadahPackageBuilder"

# Push to GitHub
git push origin feat/implement-pack-crate

# Merge when done
git checkout dev/initial-implementation
git merge feat/implement-pack-crate
git push origin dev/initial-implementation
```

---

## 🎓 Rust Resources (If Needed)

### Learning
- **The Rust Book**: https://doc.rust-lang.org/book/
- **Rust by Example**: https://doc.rust-lang.org/rust-by-example/
- **Tokio Tutorial**: https://tokio.rs/tokio/tutorial

### Documentation
- **Std Docs**: https://doc.rust-lang.org/std/
- **Crates.io**: https://crates.io/ (find dependencies)
- **Docs.rs**: https://docs.rs/ (read crate docs)

### Tools
```bash
# Rust analyzer (VS Code extension)
# Install: "rust-analyzer" in VS Code

# Cargo extensions
cargo install cargo-edit    # Add/remove deps easily
cargo install cargo-watch   # Auto-rebuild
cargo install cargo-expand  # Expand macros
cargo install cargo-audit   # Security audit
```

---

## ✅ Ready Checklist

Before starting implementation, ensure:

- [ ] Rust installed (`rustc --version` works)
- [ ] Cargo installed (`cargo --version` works)
- [ ] Can build spec crate (`cargo build -p wadah-spec`)
- [ ] Can run tests (`cargo test -p wadah-spec`)
- [ ] Git configured (`git config --list`)
- [ ] Editor ready (VS Code with rust-analyzer)
- [ ] Read IMPLEMENTATION_PROGRESS.md
- [ ] Read release checklist

---

## 🚀 First Command to Run

```bash
# After Rust installation:
cd /Users/hsp/Projects/wadah-engine

# This should work immediately (spec crate is done):
cargo test -p wadah-spec

# Expected output:
# running 3 tests
# test wadah_spec::tests::test_parse_minimal_spec ... ok
# test wadah_spec::tests::test_security_level_detection ... ok
# test toolcaps::tests::test_action_allowed ... ok
#
# test result: ok. 3 passed; 0 failed
```

---

**Next**: Install Rust, then run the command above to verify setup! 🦀

**After that**: Start implementing pack crate following IMPLEMENTATION_PROGRESS.md 🚀

