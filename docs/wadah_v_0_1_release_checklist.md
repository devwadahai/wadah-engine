# 🧱 Wadah v0.1 Release Checklist

## 🎯 Goal
Ship a **clean, minimal, working first release** that proves Wadah's core idea: *AI-native packaging that rides on Docker/Kubernetes rails.*

Focus: **usability + determinism + compatibility**. Defer everything else.

---

## 📊 Current Status

### ✅ Already Complete (Design/Documentation Phase)
- [x] **README.md** — Intelligence layer positioning, Docker/K8s integration
- [x] **CHANGELOG.md** — v0.1.0 release notes prepared
- [x] **docs/Quickstart.md** — 2-minute setup guide
- [x] **docs/WadahSpec-v0.1.md** — Complete manifest specification
- [x] **docs/Interoperability.md** — Framework integration (LangChain, TGI, Qdrant)
- [x] **docs/Plugins.md** — Security plugin system
- [x] **docs/Deployment.md** — Docker/K8s deployment patterns
- [x] **docs/OAT.md** — OpenAgentTrace specification
- [x] **docs/ToolCaps.md** — Permission system
- [x] **docs/ThreatModel.md** — Security considerations
- [x] **docs/README.md** — Documentation index
- [x] **Templates** — 5 complete reference implementations
  - hello-world, langchain-rag, rag-service, devops-copilot, defi-risk-watcher
- [x] **Rust skeleton** — All 6 crates with proper structure

### 🔄 Remaining Work (Implementation Phase)
All core functionality needs implementation — see checklist below.

---

## ✅ Core CLI Commands (Must Work End-to-End)

### 1. `wadah init <name>`
- [ ] Creates project scaffold:
  - `wadah.yaml` (agent manifest)
  - `prompts/system.txt`
  - Optional: `ToolCaps.json` (if security > minimal)
  - `README.md` (template-specific)
- [ ] Template support: `--template hello-world|langchain-rag|rag-service|devops-copilot|defi-risk-watcher`
- [ ] Security levels: `--security minimal|standard|strict`
- [ ] Clean CLI output with success messages

### 2. `wadah pack -m wadah.yaml -o agent.wpkg`
- [ ] Builds `.wpkg` (tar+zstd, OCI-compatible)
- [ ] Validates `wadah.yaml` against schema
- [ ] Verifies all referenced files exist
- [ ] Generates `wadah.lock` for reproducibility
- [ ] Computes SHA256 digests for integrity
- [ ] Clean progress indicators

### 3. `wadah run <agent.wpkg|wadah.yaml>`
- [ ] Executes packaged agent or direct manifest
- [ ] Model adapters:
  - [ ] OpenAI (via `OPENAI_API_KEY`)
  - [ ] Ollama (local development)
- [ ] Prompt input:
  - [ ] `--prompt "text"` (single query)
  - [ ] `--interactive` (REPL mode)
- [ ] Output modes:
  - [ ] Stdout (clean, formatted)
  - [ ] `--trace execution.jsonl` (OAT format)
- [ ] Plugin enforcement:
  - [ ] Budgets (basic token/cost tracking)
  - [ ] ToolCaps (basic permission checks)
- [ ] Error handling with helpful messages

### 4. `wadah trace` commands
- [ ] `wadah trace stats <file.jsonl>` — Show summary statistics
  - Total tokens, cost, duration
  - Model calls breakdown
  - Tool invocations
- [ ] `wadah trace replay <file.jsonl> --lock wadah.lock`
  - Deterministic replay with same seed
  - Validate against original trace
  - Report differences

### 5. `wadah push/pull` (OCI Distribution)
- [ ] `wadah push <reference>` — Push to registry
  - GHCR support (ghcr.io)
  - DockerHub support (docker.io)
  - Authentication via docker credentials
- [ ] `wadah pull <reference>` — Pull from registry
  - Download and verify integrity
  - Extract to local .wpkg

### 6. `wadah verify <agent.wpkg>`
- [ ] Verify package integrity (SHA256 checksums)
- [ ] Validate manifest schema
- [ ] Check file completeness
- [ ] Report status with details

### 7. `wadah plugins`
- [ ] List available plugins
- [ ] Show plugin descriptions
- [ ] Display presets (minimal/standard/strict)

---

## 📦 Packaging & Compatibility

- [ ] `.wpkg` structure finalized and validated:
  ```
  agent.wpkg (tar+zstd):
  ├── wadah.yaml       # Agent manifest
  ├── wadah.lock       # Dependency pins
  ├── manifest.json    # Package metadata
  ├── prompts/         # Prompt files
  ├── ToolCaps.json    # Optional: permissions
  ├── data/            # Optional: vector indexes
  └── app/             # Optional: framework code
  ```
- [ ] OCI-compatible layer format
- [ ] Push/pull verified on:
  - [ ] GitHub Container Registry (GHCR)
  - [ ] Docker Hub
- [ ] Docker integration demo working:
  ```bash
  docker run -v ./agent.wpkg:/agent.wpkg \
    -e OPENAI_API_KEY=$OPENAI_API_KEY \
    ghcr.io/devwadahai/wadah-runtime:0.1.0 wadah run /agent.wpkg
  ```

---

## 🧠 Runtime Core (MVP Implementation)

### Architecture
- [ ] CLI in Rust (Tokio + Clap)
- [ ] Async runtime (Tokio)
- [ ] Proper error handling (anyhow/thiserror)
- [ ] Structured logging (tracing crate)

### Model Adapters
- [ ] OpenAI adapter (cloud)
  - Streaming support
  - Error handling
  - Rate limiting
- [ ] Ollama adapter (local)
  - Auto-detection
  - Model validation
  - Fallback handling

### Core Features
- [ ] Manifest parsing (wadah.yaml)
- [ ] Lockfile generation (wadah.lock)
- [ ] Prompt template loading
- [ ] Environment variable substitution
- [ ] Deterministic RNG (seed support)
- [ ] OAT trace recording (JSONL)
- [ ] Plugin system framework

### Plugin Implementation (Basic)
- [ ] Plugin loader
- [ ] Budget tracking (tokens, USD, time)
- [ ] ToolCaps enforcement (basic checks)
- [ ] Preset configurations (minimal/standard/strict)

---

## 🧪 Testing

### Unit Tests
- [ ] `spec` crate: Manifest parsing, validation
- [ ] `pack` crate: Package build/extract, integrity
- [ ] `runtime` crate: Model adapters, executor
- [ ] `trace` crate: OAT recording/replay
- [ ] `oci` crate: Reference parsing, client

### Integration Tests
- [ ] `wadah init` creates valid project
- [ ] `wadah pack` builds valid .wpkg
- [ ] `wadah run` executes agent (mocked model)
- [ ] `wadah trace replay` deterministic replay
- [ ] `wadah verify` catches corruption
- [ ] Template validation (all 5 templates)

### End-to-End Tests
- [ ] Full workflow: init → pack → run → trace
- [ ] OCI push/pull roundtrip
- [ ] Docker integration test
- [ ] Multiple security levels

---

## 🔒 Security (Plugin-Based, MVP Scope)

### Implemented
- [ ] Plugin system framework
- [ ] Basic budget tracking (warn on exceed)
- [ ] Basic ToolCaps (policy validation)
- [ ] Security presets (minimal/standard/strict)

### Deferred to v0.2
- [ ] WASM tool sandbox
- [ ] Network policy enforcement
- [ ] Filesystem isolation
- [ ] Advanced budget limits
- [ ] Audit logging

---

## 🎨 Templates (Validated & Working)

- [ ] **hello-world**: Works with OpenAI and Ollama
- [ ] **langchain-rag**: LangChain integration verified
- [ ] **rag-service**: Vector search functional
- [ ] **devops-copilot**: ToolCaps enforcement demo
- [ ] **defi-risk-watcher**: API integration example
- [ ] All templates have README.md
- [ ] All templates tested in CI

---

## 🔄 CI/CD Pipeline

### GitHub Actions
- [ ] Rust CI workflow:
  - Build on Linux/macOS
  - Run tests
  - Run clippy (lints)
  - Check formatting
- [ ] Documentation checks:
  - Link validation
  - Markdown linting
- [ ] Release automation:
  - Build binaries (Linux, macOS, Windows)
  - Create GitHub Release
  - Publish to crates.io
  - Build Docker images

### Docker Images
- [ ] `ghcr.io/devwadahai/wadah-runtime:0.1.0` — Base runtime image
- [ ] `ghcr.io/devwadahai/wadah-runtime:latest` — Latest tag
- [ ] Published to GHCR

---

## 📦 Distribution & Installation

### Binaries
- [ ] GitHub Releases with artifacts:
  - `wadah-linux-x86_64`
  - `wadah-macos-x86_64`
  - `wadah-macos-aarch64` (Apple Silicon)
  - `wadah-windows-x86_64.exe`
- [ ] Checksums (SHA256) file
- [ ] Installation instructions

### Package Managers
- [ ] Rust: Published to crates.io
  ```bash
  cargo install wadah
  ```
- [ ] Homebrew formula (optional, nice to have)
  ```bash
  brew install wadah
  ```

### Docker
- [ ] Runtime image on GHCR:
  ```bash
  docker pull ghcr.io/devwadahai/wadah-runtime:0.1.0
  ```

---

## 📘 Documentation (Already Complete, Verify Only)

### User Documentation
- [ ] README.md — Quick start works (verify commands)
- [ ] docs/Quickstart.md — All steps verified
- [ ] docs/WadahSpec-v0.1.md — Examples match implementation
- [ ] docs/Interoperability.md — LangChain template works
- [ ] All doc links valid (no 404s)

### Developer Documentation
- [ ] CONTRIBUTING.md — Updated with build instructions
- [ ] Architecture diagrams (optional, nice to have)
- [ ] API documentation (rustdoc)

---

## 🎬 Demo Scenario (Release Showcase)

**Goal:** Prove end-to-end workflow works perfectly.

### Demo Steps
```bash
# 1. Initialize
wadah init hello-agent --template hello-world
cd hello-agent

# 2. Configure
export OPENAI_API_KEY="sk-..."

# 3. Run locally (interactive)
wadah run wadah.yaml --interactive

# 4. Package
wadah pack -m wadah.yaml -o build/hello.wpkg

# 5. Run packaged
wadah run build/hello.wpkg --prompt "Hello, Wadah!" --trace execution.jsonl

# 6. Verify trace
wadah trace stats execution.jsonl

# 7. Replay (deterministic)
wadah trace replay execution.jsonl --lock wadah.lock

# 8. Push to registry
wadah push ghcr.io/devwadahai/hello-agent:v1 --package build/hello.wpkg

# 9. Pull and run
wadah pull ghcr.io/devwadahai/hello-agent:v1
wadah run build/hello.wpkg
```

### Expected Outcomes
- ✅ Clean CLI output (no errors)
- ✅ Valid `.wpkg` artifact (< 10 MB)
- ✅ Recorded trace file (JSONL)
- ✅ Deterministic replay (identical output)
- ✅ Successful push to GHCR
- ✅ Successful pull and re-run

---

## 📌 Release Management

### Version Management
- [ ] All crates versioned at `0.1.0` in Cargo.toml
- [ ] CHANGELOG.md updated with:
  - Release date
  - All features
  - Known limitations
  - Migration guide (if applicable)
- [ ] Git tag: `v0.1.0`
- [ ] GitHub Release created with:
  - Release notes
  - Binaries attached
  - Installation instructions

### Announcement
- [ ] README.md updated with installation instructions
- [ ] Badge: ![Version](https://img.shields.io/badge/version-0.1.0-blue)
- [ ] Social media (optional)
- [ ] Blog post (optional)

---

## 🚀 Definition of Done

### Must Have (Blockers)
- ✅ All 7 core CLI commands work end-to-end
- ✅ hello-world template works perfectly
- ✅ `.wpkg` build + push verified on GHCR
- ✅ Deterministic replay works with wadah.lock
- ✅ All unit tests pass
- ✅ All integration tests pass
- ✅ Documentation verified and accurate
- ✅ CI/CD pipeline green
- ✅ Binaries built for all platforms
- ✅ Published to crates.io

### Should Have (Important)
- ✅ Ollama adapter working
- ✅ All 5 templates validated
- ✅ Docker image on GHCR
- ✅ End-to-end demo script successful

### Nice to Have (Optional)
- 🔄 TGI adapter
- 🔄 vLLM adapter
- 🔄 Homebrew formula
- 🔄 Windows installer
- 🔄 Architecture diagrams

---

## 🎯 Success Criteria

**v0.1.0 is ready when:**
1. A developer can go from `cargo install wadah` to running an agent in < 5 minutes
2. The hello-world demo works perfectly every time
3. Deterministic replay produces identical results
4. Packages can be shared via GHCR
5. All documentation is accurate and tested
6. CI/CD is green and release artifacts are built

---

## 📈 Post-Release Roadmap (v0.2+)

### Defer to v0.2
- [ ] K8s operator (CRD: WadahPackage)
- [ ] WASM tool sandbox
- [ ] Advanced security plugins
- [ ] Multi-agent orchestration
- [ ] Web dashboard
- [ ] Additional model adapters (Anthropic, Cohere)
- [ ] Advanced ToolCaps enforcement

### Community Building
- [ ] Collect feedback
- [ ] GitHub Discussions
- [ ] Issue triage
- [ ] Contribution guidelines refinement

---

**That's your v0.1 milestone** — the minimum viable foundation for the Wadah ecosystem. 🌊

**Current Phase**: Implementation  
**Target Release**: Q4 2025 / Q1 2026  
**Focus**: Core functionality + excellent DX
