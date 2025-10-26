# Hello World - The Simplest Wadah Agent

The absolute simplest agent to get started with Wadah.

## Quick Start (30 seconds)

```bash
# Set API key
export OPENAI_API_KEY="sk-..."

# Run it!
wadah run wadah.yaml --prompt "Hello, what can you do?"
```

That's it! No configuration needed.

## What's In It?

Just the essentials:

```yaml
# wadah.yaml
apiVersion: wadah.ai/v0.1
kind: Agent
metadata:
  name: hello-world
  version: 0.1.0
  authors: ["Wadah Team"]
runtime:
  model:
    provider: openai
    modelId: gpt-4o-mini

# No security, no policies - pure simplicity
```

## Try It

### Single Prompt
```bash
wadah run wadah.yaml --prompt "Tell me a joke"
```

### Interactive Chat
```bash
wadah run wadah.yaml --interactive
```

### With Local Model (Free!)
```bash
# Install Ollama first: https://ollama.ai
ollama pull llama3

# Edit wadah.yaml:
#   provider: ollama
#   modelId: llama3

wadah run wadah.yaml --prompt "Hello!"
```

## Next Steps

### Add Cost Protection
```yaml
plugins:
  - id: security.budgets
    enabled: true
    config:
      usd_per_day: 10.0
```

### Package It
```bash
wadah pack -m wadah.yaml -o hello.wpkg
wadah run hello.wpkg
```

### Share It
```bash
wadah push ghcr.io/username/hello-world:v1
```

## Learn More

- Try other templates:
  - `templates/rag-service/` - Document Q&A
  - `templates/devops-copilot/` - DevOps automation
  - `templates/defi-risk-watcher/` - DeFi monitoring

- Add security: [Security Plugins](../../docs/Plugins.md)
- Deep dive: [WadahSpec](../../docs/WadahSpec-v0.1.md)

## Philosophy

This template shows Wadah at its simplest:
- No security overhead
- No complex configuration
- Just a model and a prompt

Perfect for:
- 🎓 Learning Wadah
- 🚀 Quick experiments
- 💡 Proof of concepts

When you're ready for production, check out the other templates with full security enabled.

## License

Apache 2.0

