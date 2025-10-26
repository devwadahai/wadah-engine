# Changelog

All notable changes to Wadah will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2025-10-19

### 🌊 Philosophy

**Wadah = Intelligence Layer for Docker/K8s**

This release establishes Wadah as a **complementary layer** on top of Docker and Kubernetes infrastructure, not a replacement. We reuse proven container technology and add AI-native primitives.

### Added

#### Core Runtime & CLI
- **Wadah CLI**: Complete command-line interface (`init`, `pack`, `run`, `verify`, `trace`, `push`, `pull`, `plugins`)
- **Plugin System**: Security as optional, progressive add-ons
  - `security.budgets` - Cost and token limits
  - `security.toolcaps` - Fine-grained permissions
  - `security.network` - Domain whitelisting
  - `security.filesystem` - File access control
  - `observability.tracing` - Execution audit logs
- **Security Levels**: Permissive/Standard/Strict presets
- **Runtime Engine**: Execute agents with policy enforcement

#### Packaging & Distribution (OCI-Compatible)
- **`.wpkg` Format**: Semantic agent packaging (tar+zstd)
  - Prompts, policies, memory schemas, model configs
  - SHA256 integrity verification for all artifacts
  - Deterministic lockfiles (`wadah.lock`)
- **OCI Integration**: Push/pull to Docker registries
  - GitHub Container Registry (GHCR)
  - Docker Hub
  - AWS ECR, GCP Artifact Registry, Azure ACR
- **Manifest System**: Complete metadata and dependency tracking

#### Model Adapters
- **OpenAI**: GPT-4, GPT-4o, GPT-3.5, compatible APIs
- **Ollama**: Local model inference (free, no API keys)
- **TGI**: HuggingFace Text Generation Inference
- **vLLM**: High-performance inference server

#### Observability (OpenTelemetry-Compatible)
- **OpenAgentTrace (OAT)**: Structured trace format
  - Agent lifecycle events
  - Model requests/responses (prompts, tokens, costs)
  - Tool invocations
  - Memory operations
  - Policy checks
- **JSONL Export**: Streaming trace recording
- **Deterministic Replay**: Reproduce executions from traces + lockfiles
- **OTLP Integration**: Export to Prometheus/Grafana/Jaeger

#### Docker/K8s Integration
- **Container Images**: `wadah/runtime` base images
- **Kubernetes Manifests**: Deployment examples
- **Helm Charts**: Coming in 0.2.0
- **Service Mesh Compatible**: Istio, Linkerd ready
- **GPU Support**: CUDA/ROCm passthrough

#### Templates
- **hello-world**: Minimal agent (no security, 30-second start)
- **langchain-rag**: **NEW** - LangChain + TGI + Qdrant + Composio (complete RAG example)
- **rag-service**: Document Q&A with vector search
- **devops-copilot**: Infrastructure automation (K8s, Docker, Git)
- **defi-risk-watcher**: Blockchain protocol monitoring

#### Documentation
- **Quickstart**: Get running in 2 minutes
- **WadahSpec v0.1**: Agent manifest specification
- **Interoperability**: **NEW** - Framework integration guide (LangChain, LlamaIndex, TGI, Qdrant)
- **Security Plugins**: Progressive security documentation
- **Deployment Patterns**: Docker/K8s integration guide
- **Layered Architecture**: How Wadah fits in the stack
- **OpenAgentTrace (OAT)**: Tracing specification
- **ToolCaps**: Fine-grained permissions (optional)
- **Threat Model**: Security considerations

### Architecture Highlights

#### What We Reuse from Docker/K8s
- ✅ OCI artifacts and registries
- ✅ Container isolation (namespaces, cgroups, seccomp)
- ✅ Kubernetes orchestration (scheduling, autoscaling)
- ✅ Networking (Services, Ingress, NetworkPolicies)
- ✅ Storage (Volumes, PVCs, CSI)
- ✅ Observability (Prometheus, Grafana, OTLP)

#### What We Add (AI-Native Layer)
- 🆕 Semantic packaging (prompts, policies, memory)
- 🆕 Deterministic reasoning and replay
- 🆕 AI-level permissions (ToolCaps)
- 🆕 Built-in RAG and vector store integration
- 🆕 Multi-agent orchestration primitives
- 🆕 Framework integration (LangChain, LlamaIndex, custom code)

### Design Philosophy

**Progressive Security**:
- Start with `--security minimal` for development
- Add `--security standard` for basic protection
- Use `--security strict` for production

**Docker-Like UX**:
```bash
wadah init    → wadah pack    → wadah push    → wadah run
(like Docker) (docker build)  (docker push)   (docker run)
```

**Composable, Not Competitive**:
- Wadah doesn't replace Docker/K8s
- It adds intelligence layer on top
- Use existing tools and knowledge

### Developer Experience

- **30-second quick start** for experiments
- **No security required** for local development
- **Optional plugins** add features progressively
- **Full Docker/K8s compatibility** for production

### Performance

- Single binary CLI (~50MB)
- Minimal runtime overhead
- OCI-compliant packaging
- Efficient tar+zstd compression

### Known Limitations (v0.1)

- No web UI (CLI only, UI in v0.2)
- WASM tool sandbox not yet implemented
- Kubernetes operator coming in v0.2
- Multi-agent graphs (basic support, enhanced in v0.2)

---

## Release Notes

### 0.1.0 - "Intelligence Layer"

This is the first public release of Wadah.

**Key Message**: Wadah is not a Docker replacement. It's a complementary AI-native layer that runs **on top of** Docker/Kubernetes infrastructure.

**Quick Start:**
```bash
# Install
cargo install wadah

# Create agent (no security required)
wadah init hello-world --security minimal
cd hello-world

# Run locally
wadah run wadah.yaml --prompt "Hello!"

# Package as OCI artifact
wadah pack -m wadah.yaml -o agent.wpkg

# Push to Docker registry
wadah push ghcr.io/username/agent:v1

# Deploy to Kubernetes
kubectl apply -f deployment.yaml
```

**Architecture:**
```
Application:    Your AI Agents
Intelligence:   Wadah (NEW) ← You are here
Containers:     Docker/Podman
Orchestration:  Kubernetes/Nomad
Infrastructure: Linux/Cloud/Edge
```

**Use Cases:**
- 🚀 Quick AI experiments (30 seconds)
- 🤖 Production chatbots (with budgets)
- 📊 RAG document services
- ⚙️ DevOps automation agents
- 🔐 Compliant AI systems (full audit trail)

**What's Next (v0.2):**
- Web UI for traces and monitoring
- Kubernetes operator for `.wpkg` CRDs
- WASM tool sandbox
- More model adapters (Anthropic, Cohere)
- Helm charts
- Multi-agent graph enhancements

---

## Migration Guide

### From Earlier Versions

N/A - This is the initial release.

### From Other Platforms

**From Docker**:
- Wadah `.wpkg` files can be stored as OCI artifacts
- Use existing Docker registries (GHCR, DockerHub, ECR)
- Deploy on existing K8s clusters

**From LangChain/LlamaIndex**:
- Package your chains/agents in `wadah.yaml`
- Add observability with OAT traces
- Get reproducibility with lockfiles

---

## Upgrade Instructions

N/A for v0.1.0 (initial release)

---

[Unreleased]: https://github.com/zenri/wadah/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/zenri/wadah/releases/tag/v0.1.0
