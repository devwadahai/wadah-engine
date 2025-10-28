# Wadah v0.1 → v0.2 Implementation Roadmap

**Current Status:** v0.1 Core Complete (CLI, Packaging, Structure)  
**Goal:** Complete all 6 option areas for production-ready v0.2

---

## ✅ Option 1: CI/CD (COMPLETED)

- ✅ GitHub Actions workflow
- ✅ Multi-OS testing (Ubuntu, macOS)
- ✅ Rust stable + nightly
- ✅ Integration tests
- ✅ Security audit
- ✅ Documentation builds

**Status:** DONE - CI running on every push

---

## 🔌 Option 2: Complete Runtime Integration

### Current State
- ✅ Runtime structure complete
- ✅ Model adapters (OpenAI, Ollama, TGI) - skeletons
- ✅ AgentExecutor with tracing
- ✅ Budget tracking
- ✅ Policy enforcement structure
- ⏳ **Missing:** Actual API calls

### Tasks

#### 2.1 OpenAI Adapter (Priority 1)
```rust
// crates/runtime/src/adapters/openai.rs
- [ ] Test with real API key
- [ ] Handle rate limits
- [ ] Parse streaming responses
- [ ] Error handling for API failures
- [ ] Add retry logic
```

#### 2.2 Ollama Adapter (Priority 2)
```rust
// crates/runtime/src/adapters/ollama.rs  
- [ ] Test with local Ollama
- [ ] Support model pulling
- [ ] Handle connection errors
- [ ] Streaming support
```

#### 2.3 Tool Execution Framework (Priority 3)
```rust
// crates/runtime/src/tools/ (NEW)
- [ ] Tool trait definition
- [ ] HTTP tool (curl-like)
- [ ] Filesystem tool (read/write)
- [ ] Shell command tool
- [ ] Tool result parsing
```

#### 2.4 Memory Integration (Priority 4)
```rust
// crates/runtime/src/memory/ (NEW)
- [ ] In-memory store (HashMap)
- [ ] Qdrant connector
- [ ] LanceDB connector
- [ ] Memory retrieval in executor
```

**Estimated Time:** 2-3 days

---

## 📦 Option 3: Improve OCI Implementation

### Current State
- ✅ OCI client structure
- ✅ Manifest creation/parsing
- ✅ Digest computation
- ⏳ **Missing:** Actual blob upload

### Tasks

#### 3.1 Blob Upload
```rust
// crates/oci/src/client.rs
- [ ] Implement chunked blob upload
- [ ] Handle large files (streaming)
- [ ] Progress indication
- [ ] Resume capability
```

#### 3.2 Authentication
```rust
- [ ] Docker config.json parsing
- [ ] Token refresh
- [ ] Multiple registry support
- [ ] Credential helpers
```

#### 3.3 Registry Testing
```bash
- [ ] Test push to GHCR
- [ ] Test push to DockerHub
- [ ] Test pull from registries
- [ ] Handle private registries
```

**Estimated Time:** 1-2 days

---

## 📝 Option 4: Add More Templates

### Current State
- ✅ `hello-world` - Minimal template
- ✅ `langchain-rag` - RAG service template
- ⏳ Need 3 more templates

### Tasks

#### 4.1 DevOps Copilot
```yaml
templates/devops-copilot/
├── wadah.yaml          # GitHub API, kubectl tools
├── prompts/
│   ├── system.txt      # DevOps assistant prompt
│   └── examples/
├── tools/
│   ├── github.json     # GitHub API config
│   └── kubectl.json    # Kubernetes config
└── README.md
```

#### 4.2 DeFi Risk Watcher
```yaml
templates/defi-risk-watcher/
├── wadah.yaml          # Blockchain APIs, analytics
├── prompts/
│   ├── system.txt      # Risk analysis prompt
│   └── risk-rules.txt
├── tools/
│   ├── etherscan.json
│   └── coingecko.json
└── README.md
```

#### 4.3 Customer Support Bot
```yaml
templates/customer-support/
├── wadah.yaml          # RAG + ticket system
├── prompts/
│   ├── system.txt      # Support agent prompt
│   └── escalation.txt
├── memory/
│   └── faqs.json       # Knowledge base
└── README.md
```

#### 4.4 Code Review Agent
```yaml
templates/code-review/
├── wadah.yaml          # GitHub integration
├── prompts/
│   ├── system.txt      # Code reviewer prompt
│   └── standards.txt   # Coding standards
└── tools/
    └── github-pr.json
```

**Estimated Time:** 2-3 days (0.5 day per template)

---

## 🛡️ Option 5: Production Hardening

### Current State
- ✅ Basic error handling
- ✅ Unit tests
- ⏳ Need comprehensive hardening

### Tasks

#### 5.1 Error Handling
```rust
- [ ] Add context to all errors
- [ ] User-friendly error messages
- [ ] Error recovery strategies
- [ ] Graceful degradation
```

#### 5.2 Retry Logic
```rust
// crates/runtime/src/retry.rs (NEW)
- [ ] Exponential backoff
- [ ] Configurable retry policies
- [ ] Circuit breaker pattern
- [ ] Timeout handling
```

#### 5.3 Observability
```rust
// crates/runtime/src/metrics.rs (NEW)
- [ ] Prometheus metrics
- [ ] OpenTelemetry integration
- [ ] Structured logging
- [ ] Performance profiling
```

#### 5.4 Security
```bash
- [ ] Dependency audit (cargo-audit)
- [ ] SAST scanning
- [ ] Secret detection
- [ ] Input validation everywhere
```

#### 5.5 Performance
```rust
- [ ] Async optimization
- [ ] Connection pooling
- [ ] Caching strategies
- [ ] Memory profiling
```

**Estimated Time:** 3-4 days

---

## 🎨 Option 6: Developer Experience

### Current State
- ✅ README and basic docs
- ✅ CONTRIBUTING.md
- ⏳ Need more DX improvements

### Tasks

#### 6.1 Documentation
```markdown
- [ ] Architecture decision records (ADR)
- [ ] API documentation (rustdoc)
- [ ] Tutorial series (blog posts)
- [ ] Video walkthrough
- [ ] Interactive examples
```

#### 6.2 Tooling
```bash
- [ ] VSCode extension for wadah.yaml
- [ ] Syntax highlighting
- [ ] Schema validation in IDE
- [ ] Autocomplete for specs
```

#### 6.3 Community
```bash
- [ ] Setup GitHub Discussions
- [ ] Create Discord server
- [ ] First-timer friendly issues
- [ ] Contribution rewards system
```

#### 6.4 Examples & Demos
```bash
- [ ] Live demo site
- [ ] Playground environment
- [ ] Example repository
- [ ] Showcase gallery
```

**Estimated Time:** 4-5 days

---

## 🚀 Option 7: Release v0.1.0

### Pre-Release Checklist

#### Code Quality
- [ ] All CI/CD passing
- [ ] No critical warnings
- [ ] Code coverage > 70%
- [ ] Performance benchmarks

#### Documentation
- [ ] README complete
- [ ] CHANGELOG updated
- [ ] Migration guide
- [ ] Release notes drafted

#### Testing
- [ ] Manual E2E testing
- [ ] Test on clean install
- [ ] Test all templates
- [ ] Test on multiple platforms

#### Distribution
- [ ] Publish to crates.io
- [ ] Build release binaries (Linux, macOS, Windows)
- [ ] Create Docker images
- [ ] Update package managers (brew, apt)

#### Announcement
- [ ] GitHub release with notes
- [ ] Blog post
- [ ] Social media (Twitter, Reddit, HN)
- [ ] Newsletter

**Estimated Time:** 1-2 days

---

## 📅 Timeline Estimate

**Fast Track (2 weeks):**
- Week 1: Options 2-3 (Runtime + OCI)
- Week 2: Option 4-5 (Templates + Hardening)
- Release: v0.1.0

**Comprehensive (4 weeks):**
- Week 1: Option 2 (Complete Runtime)
- Week 2: Options 3-4 (OCI + Templates)
- Week 3: Option 5 (Production Hardening)
- Week 4: Option 6 + Release (DX + v0.1.0)

**Current Priority Order:**
1. 🔥 Option 2 (Runtime) - Makes it actually work
2. 🔥 Option 3 (OCI) - Completes packaging story
3. 📝 Option 4 (Templates) - Shows value
4. 🛡️ Option 5 (Hardening) - Production ready
5. 🎨 Option 6 (DX) - Community growth
6. 🚀 Option 7 (Release) - Launch!

---

## 🎯 Next Immediate Steps

1. **Test Runtime** (`./tests/integration/test-runtime.sh`)
2. **Fix any blocking issues**
3. **Implement OpenAI adapter fully**
4. **Create one complete end-to-end demo**
5. **Document the workflow**

---

*Roadmap created: October 26, 2025*  
*Target v0.1.0 release: November 2025*  
*Target v0.2.0 (full features): December 2025*

