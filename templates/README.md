# Wadah Agent Templates

Pre-configured templates to kickstart your AI agent development.

## Available Templates

### 1. **hello-world** 👋
**Simplest possible agent - 30 seconds to run**

- ✅ No security configuration
- ✅ 5-line `wadah.yaml`
- ✅ Perfect for learning
- ✅ Works with OpenAI or Ollama

```bash
wadah init my-first-agent --template hello-world
cd my-first-agent
wadah run wadah.yaml --prompt "Hello!"
```

**Use case**: Learning, experimentation, quick prototypes

---

### 2. **langchain-rag** 🔗 **NEW**
**Production RAG with LangChain + TGI + Qdrant**

- ✅ Complete LangChain integration
- ✅ TGI for model serving (self-hosted)
- ✅ Qdrant vector database
- ✅ Composio for tool integration
- ✅ Docker Compose stack included
- ✅ Kubernetes manifests ready

```bash
wadah init my-rag --template langchain-rag
cd my-rag
docker-compose up -d        # Start TGI + Qdrant
wadah run wadah.yaml --prompt "What is RAG?"
```

**Use case**: Document Q&A, knowledge bases, semantic search

**Stack**:
- **Framework**: LangChain
- **Model**: TGI (Mistral-7B-Instruct)
- **Vector DB**: Qdrant
- **Tools**: Composio (GitHub integration)

**See**: [Interoperability Guide](../docs/Interoperability.md) for details

---

### 3. **rag-service** 📚
**Simple RAG service with vector memory**

- ✅ Vector-based memory
- ✅ Document ingestion
- ✅ Standard security preset
- ✅ ToolCaps for read-only operations

```bash
wadah init doc-qa --template rag-service
cd doc-qa
wadah run wadah.yaml --prompt "Summarize the docs"
```

**Use case**: Internal documentation, FAQ bots, research assistants

---

### 4. **devops-copilot** 🔧
**Infrastructure automation agent**

- ✅ Kubernetes operations
- ✅ Docker commands
- ✅ Git workflows
- ✅ Strict ToolCaps (filesystem, network, shell)

```bash
wadah init devops-bot --template devops-copilot
cd devops-bot
wadah run wadah.yaml --prompt "List all K8s pods"
```

**Use case**: CI/CD automation, infrastructure management, deployment workflows

**Security**: Strict preset with explicit permissions

---

### 5. **defi-risk-watcher** 💰
**DeFi protocol monitoring**

- ✅ On-chain data fetching
- ✅ Risk analysis
- ✅ Multi-chain support
- ✅ Budget controls

```bash
wadah init risk-monitor --template defi-risk-watcher
cd risk-monitor
wadah run wadah.yaml --prompt "Analyze Aave risk"
```

**Use case**: Protocol monitoring, risk dashboards, alert systems

---

## Template Comparison

| Template | Complexity | Security | Frameworks | Best For |
|----------|-----------|----------|------------|----------|
| **hello-world** | ⭐ | Minimal | None | Learning |
| **langchain-rag** | ⭐⭐⭐⭐ | Standard | LangChain, TGI, Qdrant | Production RAG |
| **rag-service** | ⭐⭐ | Standard | None (built-in) | Simple Q&A |
| **devops-copilot** | ⭐⭐⭐ | Strict | None | Automation |
| **defi-risk-watcher** | ⭐⭐⭐⭐ | Standard | None | Monitoring |

---

## Quick Start

### Using a Template

```bash
# Initialize from template
wadah init my-agent --template <template-name>

# Or with security level
wadah init my-agent --template hello-world --security minimal
wadah init my-agent --template rag-service --security standard
```

### Creating Your Own Template

1. Create a directory in `templates/`
2. Add required files:
   ```
   my-template/
   ├── wadah.yaml          # Required
   ├── README.md           # Required
   ├── prompts/
   │   └── system.txt      # Required
   ├── ToolCaps.json       # Optional
   └── app/                # Optional (for frameworks)
   ```
3. Test it:
   ```bash
   wadah init test --template my-template
   ```

---

## Template Structure

### Minimal Template (hello-world)
```
hello-world/
├── wadah.yaml          # 5-line manifest
├── prompts/
│   └── system.txt      # Simple prompt
└── README.md           # Quick start guide
```

### Framework Template (langchain-rag)
```
langchain-rag/
├── wadah.yaml              # Full manifest with entrypoint
├── app/
│   ├── main.py             # LangChain code
│   └── requirements.txt    # Python dependencies
├── prompts/
│   └── system.txt
├── tools/
│   └── composio_github.yaml
├── docker-compose.yml      # Complete stack
├── k8s/                    # Kubernetes manifests
└── README.md               # Full documentation
```

### Production Template (devops-copilot)
```
devops-copilot/
├── wadah.yaml          # Standard security
├── ToolCaps.json       # Explicit permissions
├── prompts/
│   └── system.txt
└── README.md
```

---

## Security Levels by Template

### Minimal (hello-world)
- No ToolCaps
- No budgets
- No network restrictions
- **Development only**

### Standard (rag-service, langchain-rag, defi-risk-watcher)
- ToolCaps enabled
- Budget limits (tokens, USD)
- Network domain whitelist
- **Testing & staging**

### Strict (devops-copilot)
- Full ToolCaps enforcement
- Strict budgets
- Network policies
- Filesystem controls
- **Production ready**

---

## Framework Integration

### Bring Your Own Framework

All templates support custom frameworks via `runtime.entrypoint`:

```yaml
# LangChain
runtime:
  entrypoint: python -m app.main:agent

# LlamaIndex
runtime:
  entrypoint: python -m app.index:query

# Custom
runtime:
  entrypoint: node dist/agent.js
```

**See**: [Interoperability Guide](../docs/Interoperability.md) for complete examples

---

## Customization

### Override Model Backend

```bash
# Use Ollama instead of OpenAI
wadah init my-agent --template hello-world
cd my-agent
# Edit wadah.yaml:
#   provider: ollama
#   modelId: llama3
wadah run wadah.yaml
```

### Add Security

```bash
# Start minimal, add security later
wadah init my-agent --template hello-world --security minimal
cd my-agent
# Generate ToolCaps
wadah plugins generate --preset standard > ToolCaps.json
# Edit wadah.yaml to add plugins section
```

---

## Contributing Templates

Want to contribute a template? See [CONTRIBUTING.md](../CONTRIBUTING.md)

**Template requirements**:
- ✅ Complete README with quick start
- ✅ Working `wadah.yaml`
- ✅ System prompt in `prompts/`
- ✅ Tested with `wadah run`
- ✅ Security appropriate for use case

---

## Resources

- [Quickstart](../docs/Quickstart.md) - Get started in 2 minutes
- [WadahSpec v0.1](../docs/WadahSpec-v0.1.md) - Manifest reference
- [Interoperability](../docs/Interoperability.md) - Framework integration
- [Security Plugins](../docs/Plugins.md) - Security configuration
- [Deployment](../docs/Deployment.md) - Docker/K8s integration

---

**Start building agents today!** 🌊

