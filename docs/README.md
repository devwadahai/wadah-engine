# Wadah Documentation

Complete guide to building, deploying, and securing AI agents with Wadah.

---

## 🚀 Getting Started

### Quick Links
- **[Quickstart Guide](Quickstart.md)** - Get running in 2 minutes
- **[Templates](../templates/README.md)** - Pre-built agent examples
- **[hello-world Template](../templates/hello-world/)** - Simplest possible agent (30 seconds)

### First Steps
1. **Install**: `cargo install wadah`
2. **Create**: `wadah init my-agent --security minimal`
3. **Run**: `wadah run wadah.yaml --prompt "Hello!"`

---

## 📖 Core Documentation

### Specifications

| Document | Description | Audience |
|----------|-------------|----------|
| [WadahSpec v0.1](WadahSpec-v0.1.md) | Agent manifest format | All users |
| [OpenAgentTrace (OAT)](OAT.md) | Tracing specification | Advanced |
| [ToolCaps](ToolCaps.md) | Permission system | Security-focused |

### Architecture

| Document | Description | Audience |
|----------|-------------|----------|
| [Layered Architecture](wadah_layered_architecture_diagram_docker_reuse.md) | How Wadah fits with Docker/K8s | Architects |
| [Threat Model](ThreatModel.md) | Security considerations | Security teams |

### Integration

| Document | Description | Audience |
|----------|-------------|----------|
| [Interoperability](Interoperability.md) | Framework integration (LangChain, TGI, Qdrant) | Developers |
| [Deployment](Deployment.md) | Docker/K8s patterns | DevOps |
| [Security Plugins](Plugins.md) | Optional security features | Security-focused |

---

## 🎯 By Use Case

### I want to...

**...learn Wadah quickly**
→ [Quickstart](Quickstart.md) + [hello-world template](../templates/hello-world/)

**...integrate LangChain/LlamaIndex**
→ [Interoperability Guide](Interoperability.md) + [langchain-rag template](../templates/langchain-rag/)

**...deploy to Kubernetes**
→ [Deployment Guide](Deployment.md) → Section: "Kubernetes Deployment"

**...add security policies**
→ [Security Plugins](Plugins.md) → [ToolCaps](ToolCaps.md)

**...build a RAG agent**
→ [rag-service template](../templates/rag-service/) + [Interoperability](Interoperability.md)

**...automate infrastructure**
→ [devops-copilot template](../templates/devops-copilot/)

**...monitor DeFi protocols**
→ [defi-risk-watcher template](../templates/defi-risk-watcher/)

**...understand agent traces**
→ [OpenAgentTrace (OAT)](OAT.md)

**...contribute code**
→ [CONTRIBUTING.md](../CONTRIBUTING.md)

**...report security issues**
→ [SECURITY.md](../SECURITY.md)

---

## 📚 Documentation Map

```
Wadah Documentation
│
├── Getting Started
│   ├── Quickstart.md              ← Start here
│   ├── templates/README.md        ← Example agents
│   └── hello-world/               ← 30-second tutorial
│
├── Core Concepts
│   ├── WadahSpec-v0.1.md          ← Manifest format
│   ├── Plugins.md                 ← Security features
│   └── wadah_layered_...md        ← Architecture
│
├── Integration
│   ├── Interoperability.md        ← Framework guide
│   ├── Deployment.md              ← Docker/K8s
│   └── OAT.md                     ← Tracing
│
├── Security
│   ├── Plugins.md                 ← Security overview
│   ├── ToolCaps.md                ← Permissions
│   └── ThreatModel.md             ← Security model
│
└── Reference
    ├── WadahSpec-v0.1.md          ← Complete spec
    ├── OAT.md                     ← Trace format
    └── ToolCaps.md                ← Policy format
```

---

## 🎓 Learning Path

### Beginner (Day 1)
1. Read: [Quickstart](Quickstart.md) (10 minutes)
2. Try: [hello-world template](../templates/hello-world/) (5 minutes)
3. Explore: [WadahSpec basics](WadahSpec-v0.1.md) (15 minutes)

### Intermediate (Week 1)
1. Read: [Interoperability](Interoperability.md)
2. Try: [langchain-rag template](../templates/langchain-rag/)
3. Read: [Security Plugins](Plugins.md)
4. Try: Add security to your agent

### Advanced (Month 1)
1. Read: [Deployment Guide](Deployment.md)
2. Deploy: Agent to Kubernetes
3. Read: [OAT Specification](OAT.md)
4. Implement: Custom tracing
5. Read: [ToolCaps](ToolCaps.md)
6. Implement: Fine-grained permissions

---

## 🔍 Index by Topic

### Packaging
- [WadahSpec](WadahSpec-v0.1.md) - `artifacts` section
- [Quickstart](Quickstart.md) - `wadah pack` command

### Runtime
- [WadahSpec](WadahSpec-v0.1.md) - `runtime` section
- [Interoperability](Interoperability.md) - Model adapters
- [Deployment](Deployment.md) - Environment setup

### Security
- [Plugins](Plugins.md) - Plugin system
- [ToolCaps](ToolCaps.md) - Permissions
- [ThreatModel](ThreatModel.md) - Security analysis

### Observability
- [OAT](OAT.md) - Trace format
- [Deployment](Deployment.md) - Prometheus/Grafana setup

### Deployment
- [Deployment](Deployment.md) - Complete guide
- [Quickstart](Quickstart.md) - Quick examples
- [wadah_layered_...md](wadah_layered_architecture_diagram_docker_reuse.md) - Architecture

### Frameworks
- [Interoperability](Interoperability.md) - LangChain, LlamaIndex, etc.
- [langchain-rag template](../templates/langchain-rag/) - Working example

---

## 🤝 Contributing

- **Found a bug?** → Open an issue
- **Want to contribute?** → Read [CONTRIBUTING.md](../CONTRIBUTING.md)
- **Security issue?** → Read [SECURITY.md](../SECURITY.md)

---

## 📊 Document Status

| Document | Status | Last Updated | Audience |
|----------|--------|--------------|----------|
| Quickstart.md | ✅ Complete | 2025-10-19 | Beginners |
| WadahSpec-v0.1.md | ✅ Complete | 2025-10-19 | All |
| Interoperability.md | ✅ Complete | 2025-10-19 | Developers |
| Deployment.md | ✅ Complete | 2025-10-19 | DevOps |
| Plugins.md | ✅ Complete | 2025-10-19 | Security |
| OAT.md | ✅ Complete | 2025-10-19 | Advanced |
| ToolCaps.md | ✅ Complete | 2025-10-19 | Security |
| ThreatModel.md | ✅ Complete | 2025-10-19 | Security |
| wadah_layered_...md | ✅ Complete | 2025-10-19 | Architects |

---

## 💡 Quick Reference

### Commands
```bash
# Initialize
wadah init <name> [--security minimal|standard|strict]

# Package
wadah pack -m wadah.yaml -o agent.wpkg

# Run
wadah run agent.wpkg [--prompt "..."] [--interactive]

# Trace
wadah trace replay trace.jsonl [--lock wadah.lock]
wadah trace stats trace.jsonl

# OCI Registry
wadah push ghcr.io/org/agent:tag
wadah pull ghcr.io/org/agent:tag

# Plugins
wadah plugins [--verbose]
```

### File Structure
```
my-agent/
├── wadah.yaml          # Manifest (required)
├── prompts/
│   └── system.txt      # Prompts (required)
├── ToolCaps.json       # Security (optional)
├── app/                # Code (optional, for frameworks)
├── data/               # Memory (optional)
└── README.md           # Docs (recommended)
```

### Manifest Minimal Example
```yaml
apiVersion: wadah.ai/v0.1
kind: Agent
metadata:
  name: my-agent
  version: 0.1.0
  authors: ["You"]
runtime:
  model:
    provider: openai
    modelId: gpt-4o-mini
```

---

## 🌊 Additional Resources

- **GitHub**: https://github.com/devwadahai/wadah-engine
- **License**: Apache 2.0 ([LICENSE](../LICENSE))
- **Changelog**: [CHANGELOG.md](../CHANGELOG.md)

---

**Need help?** Start with the [Quickstart Guide](Quickstart.md) or explore the [templates](../templates/).


