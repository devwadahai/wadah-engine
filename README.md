# Wadah.ai — Intelligence Layer for Docker/K8s

<div align="center">

**AI-native packaging that rides on Docker/Kubernetes rails**

[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)

*Wadah adds the AI layer. Docker/K8s handle the rest.*

</div>

---

## 🌊 What is Wadah?

**Wadah** packages and runs AI agents like Docker packages apps — but with **semantic, AI-native primitives**.

```bash
# Familiar workflow
wadah init my-agent        # Create agent
wadah pack -m wadah.yaml   # Build .wpkg (OCI artifact)
wadah run agent.wpkg       # Execute agent
wadah push ghcr.io/org/agent:v1  # Push to Docker registry
```

**The difference?** Wadah knows about prompts, tokens, tools, and reasoning — Docker doesn't.

## 📦 Layered Architecture

```
┌─────────────────────────────────────────────────┐
│  Wadah Layer (NEW)                              │
│  • .wpkg packaging (prompts, policies, memory)  │
│  • Deterministic reasoning & replay (OAT)       │
│  • AI-level policies (ToolCaps, budgets)        │
│  • Multi-agent orchestration                    │
└─────────────────────────────────────────────────┘
             ↓ uses ↓
┌─────────────────────────────────────────────────┐
│  Docker/K8s Layer (REUSED)                      │
│  • OCI registries (GHCR, DockerHub, ECR)        │
│  • Container isolation (namespaces, cgroups)    │
│  • Cluster orchestration (K8s, Nomad)           │
│  • Observability (Prometheus, Grafana, OTLP)    │
└─────────────────────────────────────────────────┘
```

**Bottom line**: Wadah **composes with** Docker/K8s, not replaces them.

## 🚀 Quick Start (2 minutes)

### CLI (Command Line)

```bash
# Minimal (No Security)
wadah init hello --security minimal
cd hello
export OPENAI_API_KEY="sk-..."
wadah run wadah.yaml --prompt "Hello!"
```

### 🎨 Web UI (Graphical Interface)

**Prefer a visual interface?** Check out **[Wadah UI](https://github.com/devwadahai/wadah-ui)** - a modern web dashboard for managing AI agents without touching the command line.

- **Repository:** https://github.com/devwadahai/wadah-ui
- **Tech Stack:** React + TypeScript + Tailwind CSS
- **Features:** Visual agent builder, real-time monitoring, template library, drag-and-drop
- **Status:** Active development (v0.2.0 coming Q4 2025)

### With Docker
```bash
# Build .wpkg as OCI artifact
wadah pack -m wadah.yaml -o agent.wpkg

# Push to Docker registry
wadah push ghcr.io/username/agent:v1

# Run in Docker
docker run -v ./agent.wpkg:/agent.wpkg \
  wadah/runtime wadah run /agent.wpkg
```

### On Kubernetes
```bash
# Deploy as K8s workload
kubectl apply -f wadah-deployment.yaml

# .wpkg loaded from ConfigMap or pulled from registry
```

## 🔄 What Wadah Adds to Docker/K8s

### 1. **Semantic Packaging** (`.wpkg`)
**Docker**: Packages binaries + dependencies  
**Wadah**: Packages prompts + policies + memory schemas + model configs

```
.wpkg contents:
├── wadah.yaml          # Agent manifest
├── prompts/            # System prompts, templates
├── ToolCaps.json       # AI-level permissions
├── data/               # RAG indexes, memory
└── wadah.lock          # Deterministic pins
```

### 2. **Deterministic Reasoning** (OAT)
**Docker**: Logs stdout/stderr  
**Wadah**: Traces prompts, tokens, costs, tool calls, decisions

```bash
# Record execution
wadah run agent.wpkg --trace execution.jsonl

# Replay deterministically (same seed → same output)
wadah trace replay execution.jsonl --lock wadah.lock
```

### 3. **AI-Level Policies** (ToolCaps)
**Docker**: Process isolation (filesystem, network at OS level)  
**Wadah**: Agent permissions (which tools, APIs, token budgets)

```yaml
# Control what agents can do
plugins:
  - id: security.budgets
    config:
      usd_per_day: 10.0
      max_tokens: 100000
  - id: security.network
    # Allow only specific domains
```

### 4. **Multi-Agent Orchestration**
**K8s**: Schedules containers  
**Wadah**: Orchestrates reasoning graphs, eval hooks, retries

## 🏗️ Deployment Patterns

### Pattern 1: Wadah in Docker (Single Host)
```bash
docker run -d \
  -v /var/wadah/agents:/agents \
  -v /var/wadah/data:/data \
  --gpus all \
  wadah/runtime:latest
```

### Pattern 2: Wadah on Kubernetes (Cluster)
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: wadah-agent
spec:
  replicas: 3
  template:
    spec:
      containers:
      - name: wadahd
        image: wadah/runtime:0.1.0
        volumeMounts:
        - name: agents
          mountPath: /agents
```

### Pattern 3: Hybrid (Edge + Cloud)
```
Edge: Wadah (low-latency tools, cache)
  ↕
Cloud: vLLM/TGI (heavy models), Vector DB
```

## 📊 Architecture Layers

### Full Stack View
```
┌──────────────────────────────────────────────┐
│ User Interfaces                              │
│ Wadah CLI • Web Dashboard • SDKs             │
├──────────────────────────────────────────────┤
│ Orchestration & Observability                │
│ K8s/Nomad • OAT/OTLP • Prometheus/Grafana    │
├──────────────────────────────────────────────┤
│ Intelligence Orchestrator                    │
│ Multi-agent • Routing • Eval • Guardrails    │
├──────────────────────────────────────────────┤
│ Runtime (Wadahd Core)                        │
│ Lifecycle • Policy VM • Sandboxing • I/O     │
├──────────────────────────────────────────────┤
│ Memory / Data Layer                          │
│ Vector DB • Redis • RAG • Knowledge Graph    │
├──────────────────────────────────────────────┤
│ Model Serving (Pluggable)                    │
│ OpenAI • vLLM • TGI • Ollama • Llama.cpp     │
├──────────────────────────────────────────────┤
│ System & Distribution (REUSED)               │
│ Docker/OCI • K8s • Registries • GPUs         │
└──────────────────────────────────────────────┘
```

## 🎯 What We Reuse from Docker/K8s

✅ **OCI Artifacts**: `.wpkg` as OCI artifacts in Docker registries  
✅ **Container Isolation**: Linux namespaces, cgroups, seccomp  
✅ **Orchestration**: K8s scheduling, autoscaling, service mesh  
✅ **Observability**: OTLP, Prometheus, Grafana, Jaeger  
✅ **Networking**: Docker networks, K8s Services, Ingress  
✅ **Storage**: Volumes, PVCs, CSI drivers  

## 💡 What We Add (AI-Native)

🆕 **Agent Packaging**: Prompts, policies, memory in `.wpkg`  
🆕 **Deterministic Replay**: Reproduce runs from traces + lockfiles  
🆕 **ToolCaps**: AI-level permissions (tools, domains, budgets)  
🆕 **RAG Primitives**: Built-in vector store integration  
🆕 **Multi-Agent Graphs**: Reasoning workflows, not just services  
🆕 **Framework Agnostic**: Works with LangChain, LlamaIndex, or custom code  

## 🔧 Features

### Core Runtime
- ✅ `.wpkg` packaging (OCI-compatible)
- ✅ Model adapters (OpenAI, Ollama, TGI, vLLM)
- ✅ Plugin system for security
- ✅ Docker registry push/pull
- ✅ Framework support (LangChain, LlamaIndex, custom)

### Observability
- ✅ OpenAgentTrace (OAT) format
- ✅ OTLP export (Prometheus/Grafana)
- ✅ Deterministic replay
- ✅ Cost/token tracking

### Security (Optional Plugins)
- 🔌 ToolCaps (fine-grained permissions)
- 🔌 Budget limits (tokens, cost, time)
- 🔌 Network policies (domain whitelist)
- 🔌 Filesystem controls

## 📚 Documentation

**📑 [Complete Documentation Index](docs/INDEX.md)** - All docs organized by topic

**Getting Started:**
- [Quickstart](docs/Quickstart.md) - Run your first agent
- [Architecture](docs/wadah_layered_architecture_diagram_docker_reuse.md) - How Wadah fits with Docker/K8s
- [Interoperability](docs/Interoperability.md) - Framework integration (LangChain, TGI, Qdrant, Composio)

**Specifications:**
- [WadahSpec v0.1](docs/WadahSpec-v0.1.md) - Agent manifest format
- [UI Specification](docs/UI-SPEC.md) - Frontend design and architecture
- [Security Plugins](docs/Plugins.md) - Optional security features
- [OpenAgentTrace (OAT)](docs/OAT.md) - Tracing specification

**Advanced:**
- [ToolCaps](docs/ToolCaps.md) - Fine-grained permissions
- [Deployment Patterns](docs/Deployment.md) - Docker/K8s integration
- [Threat Model](docs/ThreatModel.md) - Security considerations

**Interfaces:**
- **[Wadah UI](https://github.com/devwadahai/wadah-ui)** - Web dashboard (React/TypeScript)
- **CLI** - Command-line interface (this repo)

## ❓ FAQ

### "Why not just Docker?"
**Docker** standardizes *process* packaging. It knows nothing about:
- Prompts and model parameters
- Token budgets and costs
- Tool permissions and capabilities
- Deterministic reasoning replay

**Wadah** adds the *reasoning* layer on top of Docker's proven plumbing.

### "Does this replace Kubernetes?"
No! Wadah **runs on** Kubernetes. K8s handles:
- Container scheduling
- Networking and load balancing
- Autoscaling
- Infrastructure

Wadah adds:
- AI-specific packaging
- Reasoning observability
- Agent-level policies

### "Can I use existing Docker registries?"
Yes! `.wpkg` files are OCI-compatible:
```bash
wadah push ghcr.io/org/agent:v1      # GitHub Container Registry
wadah push docker.io/org/agent:v1    # Docker Hub
wadah push ecr.aws/org/agent:v1      # AWS ECR
```

### "Does it work with LangChain/LlamaIndex?"
Yes! Wadah is framework-agnostic. See [Interoperability Guide](docs/Interoperability.md) for complete examples:
- LangChain + TGI + Qdrant + Composio
- LlamaIndex with custom tools
- Any Python/TypeScript framework

## 🎯 Use Cases

| Use Case | Deployment | Security Level |
|----------|-----------|----------------|
| 🚀 Quick Experiments | Local, Docker | Minimal |
| 🤖 Chatbots | Docker Compose | Standard |
| 📊 RAG Services | K8s Deployment | Standard |
| ⚙️ DevOps Agents | K8s DaemonSet | Strict |
| 🔐 Production AI | K8s + Service Mesh | Strict |

## 🏗️ Building from Source

```bash
git clone https://github.com/zenri/wadah
cd wadah
cargo build --release
cargo install --path crates/cli
```

## 📋 Project Status

**Version**: 0.1.0 (MVP)

✅ **Implemented:**
- CLI and runtime
- OCI-compatible packaging
- Docker registry integration
- Plugin system
- OAT tracing

⏳ **Roadmap:**
- Kubernetes operator
- Web dashboard
- WASM tool sandbox
- Multi-agent graphs

## 🤝 Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md)

## 📄 License

Apache 2.0 - see [LICENSE](LICENSE)

## 🌐 Positioning

**Wadah sits above Docker/K8s in the stack:**

```
Application Level:  Your AI Agents
Intelligence Layer: 👉 Wadah 👈 (NEW)
Container Level:    Docker/Podman
Orchestration:      Kubernetes/Nomad
Infrastructure:     Linux/Cloud/Edge
```

**Comparable to:**
- Docker (but for AI agents, not apps)
- Kubernetes (but for reasoning, not just scheduling)
- OpenTelemetry (but for agent traces, not just logs)

---

**Wadah** — *The intelligence layer for Docker and Kubernetes*

Run minds safely. Anywhere. With the tools you already know. 🌊
