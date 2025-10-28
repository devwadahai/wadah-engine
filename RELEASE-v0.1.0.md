# Wadah v0.1.0 Release Notes

**Release Date:** October 26, 2025  
**Status:** 🎉 First Public Release

---

## 🌟 What is Wadah?

Wadah is an **AI-native runtime for agents** - like Docker, but for AI. It packages, runs, traces, and governs AI agents as portable artifacts using OCI standards.

```bash
# Familiar workflow, AI-native primitives
wadah init my-agent              # Create agent
wadah pack -m wadah.yaml         # Package to .wpkg
wadah run agent.wpkg             # Execute
wadah push ghcr.io/org/agent:v1  # Distribute
```

---

## ✨ Features

### **Core Runtime**
- ✅ **3 Model Adapters**: OpenAI, Ollama, TGI/vLLM
- ✅ **Async Execution**: High-performance Rust runtime
- ✅ **Budget Tracking**: Token limits, cost controls, duration
- ✅ **Policy Enforcement**: ToolCaps, network, filesystem policies
- ✅ **Trace Recording**: OpenAgentTrace (OAT) format

### **Packaging & Distribution**
- ✅ **.wpkg Format**: tar+zstd with integrity verification
- ✅ **OCI Compatible**: Push/pull from container registries
- ✅ **Manifest System**: SHA256 digests, metadata, lockfiles
- ✅ **Artifact Management**: Include prompts, tools, configs

### **CLI Tools**
- ✅ `wadah init` - Scaffold new agents (3 security levels)
- ✅ `wadah pack` - Build .wpkg packages
- ✅ `wadah verify` - Integrity checking
- ✅ `wadah run` - Execute agents (single or interactive)
- ✅ `wadah trace` - Replay and analyze traces
- ✅ `wadah push/pull` - Registry operations
- ✅ `wadah plugins` - List security plugins

### **Security & Observability**
- ✅ **Plugin System**: Optional security features (minimal/standard/strict)
- ✅ **ToolCaps**: Fine-grained permission control
- ✅ **Budget Limits**: Prevent runaway costs
- ✅ **Network Policies**: Domain whitelist/blacklist
- ✅ **Filesystem Controls**: Path restrictions
- ✅ **Full Tracing**: Deterministic replay with OAT

### **Developer Experience**
- ✅ **Beautiful CLI**: Colored output, spinners, progress bars
- ✅ **Interactive Mode**: Chat-like agent interaction
- ✅ **Comprehensive Docs**: 15+ documentation files
- ✅ **Agent Templates**: Ready-to-use examples
- ✅ **CI/CD Ready**: GitHub Actions workflow included

---

## 📦 Installation

### From Binary (Recommended)
```bash
# macOS (ARM)
curl -L https://github.com/devwadahai/wadah-engine/releases/download/v0.1.0/wadah-macos-arm64.tar.gz | tar xz
sudo mv wadah /usr/local/bin/

# macOS (Intel)
curl -L https://github.com/devwadahai/wadah-engine/releases/download/v0.1.0/wadah-macos-amd64.tar.gz | tar xz
sudo mv wadah /usr/local/bin/

# Linux
curl -L https://github.com/devwadahai/wadah-engine/releases/download/v0.1.0/wadah-linux-amd64.tar.gz | tar xz
sudo mv wadah /usr/local/bin/
```

### From Source
```bash
git clone https://github.com/devwadahai/wadah-engine.git
cd wadah-engine
cargo build --release
sudo cp target/release/wadah /usr/local/bin/
```

### From Cargo
```bash
cargo install wadah-cli
```

---

## 🚀 Quick Start

### 1. Create Your First Agent
```bash
wadah init hello-agent --security minimal
cd hello-agent
```

### 2. Set Your API Key
```bash
# OpenAI
export OPENAI_API_KEY='your-key-here'

# Or use Ollama (no API key needed)
# ollama serve
```

### 3. Run Your Agent
```bash
# Single prompt
wadah run wadah.yaml --prompt "Hello! Tell me about AI agents."

# Interactive mode
wadah run wadah.yaml --interactive

# With tracing
wadah run wadah.yaml --trace logs/execution.jsonl --prompt "Hello!"
```

### 4. Package & Distribute
```bash
# Build package
wadah pack -m wadah.yaml -o hello-agent.wpkg

# Verify integrity
wadah verify hello-agent.wpkg

# Push to registry (coming in v0.2.0)
wadah push ghcr.io/username/hello-agent:v1
```

---

## 📊 Project Stats

```
Lines of Code:     ~4,000+
Test Coverage:     21 unit tests
Dependencies:      50+ Rust crates
Documentation:     15+ markdown files
Templates:         2 agent examples
Supported Platforms: Linux, macOS (Windows coming soon)
```

---

## 🎯 What's New in v0.1.0

Everything! This is the first public release with:

### **Runtime Engine**
- Complete async agent executor
- 3 production-ready model adapters
- Budget tracking and policy enforcement
- OpenAgentTrace (OAT) recording

### **Packaging System**
- .wpkg format with tar+zstd compression
- Integrity verification with SHA256
- Manifest-based metadata
- Artifact management

### **CLI Tools**
- 8 complete commands
- Interactive and batch modes
- Beautiful terminal UI
- Comprehensive help system

### **Infrastructure**
- GitHub Actions CI/CD
- Multi-OS testing
- Security audits
- Documentation builds

---

## 📚 Documentation

- **[README](../README.md)** - Project overview
- **[Quickstart](docs/Quickstart.md)** - Get started in 5 minutes
- **[WadahSpec](docs/WadahSpec-v0.1.md)** - Manifest format reference
- **[Deployment](docs/Deployment.md)** - Production deployment guide
- **[Interoperability](docs/Interoperability.md)** - Framework integration
- **[Complete Index](docs/INDEX.md)** - All documentation

---

## 🛣️ Roadmap

### **v0.1.x (Maintenance)**
- Bug fixes
- Performance improvements
- Documentation enhancements
- Community feedback

### **v0.2.0 (Q4 2025)**
- Complete OCI blob uploads
- Tool execution framework
- Memory store integrations (Qdrant, LanceDB)
- More agent templates
- Windows support

### **v0.3.0 (Q1 2026)**
- WASM tool sandbox
- Multi-agent orchestration
- Kubernetes operator
- Advanced observability

---

## 🙏 Acknowledgments

Built with:
- **Rust** - Systems programming language
- **Tokio** - Async runtime
- **OCI Distribution** - Container standards
- **Serde** - Serialization framework
- **Clap** - CLI framework

Special thanks to the Rust community for amazing tools and libraries!

---

## 🐛 Known Issues

1. **OCI Push:** Manifest-only (blobs coming in v0.2.0)
2. **Windows:** Not yet supported (coming in v0.2.0)
3. **Tools:** Tool execution framework pending (v0.2.0)
4. **Memory:** Memory stores not integrated yet (v0.2.0)

See [GitHub Issues](https://github.com/devwadahai/wadah-engine/issues) for full list.

---

## 🤝 Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for:
- Code of conduct
- Development setup
- Contribution guidelines
- First-timer friendly issues

---

## 📜 License

Apache 2.0 - See [LICENSE](LICENSE) for details.

---

## 🔗 Links

- **GitHub**: https://github.com/devwadahai/wadah-engine
- **Documentation**: https://wadah.ai/docs
- **Discord**: Coming soon!
- **Twitter**: Coming soon!

---

**Happy Agent Building! 🎉**

*"Wadah: Because AI agents deserve their own container."*

