# Wadah Quickstart - Build on Docker/K8s

Get started with Wadah in **2 minutes** using tools you already know.

## 🎯 What You'll Learn

- Run a simple agent (30 seconds)
- Package as OCI artifact
- Push to Docker registry
- Deploy to Kubernetes (optional)

## Prerequisites

- Docker or Kubernetes (optional for local dev)
- OpenAI API key (or use Ollama locally)

---

## Quick Start (Local)

### 1. Create Your First Agent

```bash
# Install
cargo install wadah

# Initialize (no security for dev)
wadah init hello-world --security minimal
cd hello-world

# Set API key
export OPENAI_API_KEY="sk-..."
```

Your `wadah.yaml`:
```yaml
apiVersion: wadah.ai/v0.1
kind: Agent
metadata:
  name: hello-world
  version: 0.1.0
  authors: ["You"]
runtime:
  model:
    provider: openai
    modelId: gpt-4o-mini
```

### 2. Run It

```bash
# Single prompt
wadah run wadah.yaml --prompt "Hello, Wadah!"

# Interactive
wadah run wadah.yaml --interactive
```

**That's it!** You have a working AI agent.

---

## Package & Distribute (Docker Way)

### Framework Integration

Use your favorite AI framework with Wadah:

```bash
# LangChain + RAG template
wadah init my-rag --template langchain-rag
cd my-rag

# Start dependencies (TGI + Qdrant)
docker-compose up -d

# Run agent
wadah run wadah.yaml --prompt "What is RAG?"
```

**See**: [Interoperability Guide](Interoperability.md) for complete examples with:
- LangChain + TGI + Qdrant + Composio
- LlamaIndex patterns
- Custom Python/TypeScript frameworks

---

### 3. Build .wpkg (Like `docker build`)

```bash
wadah pack -m wadah.yaml -o build/hello.wpkg
```

The `.wpkg` is an **OCI-compatible artifact** containing:
```
hello.wpkg (tar+zstd):
├── wadah.yaml       # Agent config
├── prompts/         # Prompts
├── manifest.json    # OCI manifest
└── wadah.lock       # Dependency pins
```

### 4. Push to Registry (Like `docker push`)

```bash
# GitHub Container Registry
wadah push ghcr.io/username/hello-world:v1 \
  --package build/hello.wpkg

# Docker Hub
wadah push docker.io/username/hello-world:v1

# AWS ECR
wadah push 123456789.dkr.ecr.us-east-1.amazonaws.com/hello-world:v1
```

### 5. Pull & Run (Like `docker pull`)

```bash
# On another machine
wadah pull ghcr.io/username/hello-world:v1 \
  --output agent.wpkg

wadah run agent.wpkg --prompt "Hi!"
```

---

## Run in Docker Container

### Option 1: Direct Docker Run

```bash
docker run -it \
  -e OPENAI_API_KEY="sk-..." \
  -v ./agent.wpkg:/agent.wpkg:ro \
  wadah/runtime:latest \
  wadah run /agent.wpkg --prompt "Hello from Docker!"
```

### Option 2: Dockerfile

Create `Dockerfile`:
```dockerfile
FROM wadah/runtime:latest

# Copy agent package
COPY build/hello.wpkg /app/agent.wpkg

# Set entrypoint
ENTRYPOINT ["wadah", "run", "/app/agent.wpkg"]
```

Build and run:
```bash
docker build -t my-agent:latest .
docker run -e OPENAI_API_KEY="sk-..." my-agent:latest \
  --prompt "Hello!"
```

---

## Deploy to Kubernetes

### 1. Create ConfigMap for Agent

```bash
kubectl create configmap hello-agent \
  --from-file=agent.wpkg=build/hello.wpkg
```

### 2. Create Deployment

`agent-deployment.yaml`:
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: hello-agent
spec:
  replicas: 2
  selector:
    matchLabels:
      app: hello-agent
  template:
    metadata:
      labels:
        app: hello-agent
    spec:
      containers:
      - name: wadahd
        image: wadah/runtime:0.1.0
        command:
          - wadah
          - run
          - /app/agent.wpkg
          - --interactive
        env:
        - name: OPENAI_API_KEY
          valueFrom:
            secretKeyRef:
              name: openai-secret
              key: api-key
        volumeMounts:
        - name: agent
          mountPath: /app
          readOnly: true
      volumes:
      - name: agent
        configMap:
          name: hello-agent
```

Deploy:
```bash
# Create secret
kubectl create secret generic openai-secret \
  --from-literal=api-key="sk-..."

# Deploy
kubectl apply -f agent-deployment.yaml

# Check status
kubectl get pods -l app=hello-agent
```

---

## Use Local Models (Free!)

### With Ollama (No API Keys)

```bash
# Install Ollama
brew install ollama  # or: https://ollama.ai

# Download model
ollama pull llama3

# Update wadah.yaml
vim wadah.yaml
```

Change to:
```yaml
runtime:
  model:
    provider: ollama
    endpoint: http://host.docker.internal:11434  # For Docker
    modelId: llama3
```

Run:
```bash
wadah run wadah.yaml --prompt "Hello!"

# Or in Docker
docker run --add-host=host.docker.internal:host-gateway \
  -v ./agent.wpkg:/agent.wpkg \
  wadah/runtime wadah run /agent.wpkg
```

---

## Add Security (Progressive)

### Level 1: Budget Limits

```yaml
# wadah.yaml
plugins:
  - id: security.budgets
    enabled: true
    config:
      usd_per_day: 10.0
      max_duration_secs: 3600
```

### Level 2: Tool Permissions

Create `ToolCaps.json`:
```json
{
  "version": "0.1",
  "allow": [
    {
      "tool": "http",
      "actions": ["GET"],
      "limits": {"per_min": 60},
      "domains": ["api.openai.com"]
    }
  ]
}
```

Enable in `wadah.yaml`:
```yaml
plugins:
  - id: security.toolcaps
    enabled: true

policy:
  toolcaps: ToolCaps.json
```

### Level 3: Full Production

```bash
# Initialize with all security
wadah init prod-agent --security strict

# Run with audit trail
wadah run agent.wpkg --trace /var/log/wadah/audit.jsonl
```

---

## Observability (OTLP Integration)

### Export to Prometheus/Grafana

Wadah emits OpenTelemetry:

```yaml
# In K8s deployment
env:
- name: OTEL_EXPORTER_OTLP_ENDPOINT
  value: "http://otel-collector:4317"
- name: OTEL_SERVICE_NAME
  value: "hello-agent"
```

View in Grafana:
- Token usage per minute
- Cost per day
- Execution duration
- Tool call frequency

### Deterministic Replay

```bash
# Record execution
wadah run agent.wpkg --trace execution.jsonl

# Replay exactly (same seed → same output)
wadah trace replay execution.jsonl --lock wadah.lock

# Analyze
wadah trace stats execution.jsonl
```

---

## Architecture Overview

```
Your Workflow:
┌─────────────────────────────────────────────┐
│ wadah init → wadah pack → wadah push        │
│      ↓            ↓             ↓           │
│  wadah.yaml   .wpkg      GHCR/DockerHub     │
└─────────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────────┐
│ Deployment Options:                         │
│ • Local: wadah run                          │
│ • Docker: docker run wadah/runtime          │
│ • K8s: kubectl apply -f deployment.yaml     │
└─────────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────────┐
│ Wadah Layer (Intelligence):                 │
│ • OAT tracing                               │
│ • ToolCaps enforcement                      │
│ • Budget tracking                           │
└─────────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────────┐
│ Docker/K8s Layer (Reused):                  │
│ • Container isolation                       │
│ • Networking & storage                      │
│ • Orchestration & scaling                   │
└─────────────────────────────────────────────┘
```

---

## Common Patterns

### 1. Development Loop

```bash
# Edit
vim wadah.yaml

# Test locally
wadah run wadah.yaml --prompt "test"

# Package
wadah pack -m wadah.yaml -o agent.wpkg

# Test package
wadah run agent.wpkg
```

### 2. CI/CD Pipeline

```yaml
# .github/workflows/deploy.yml
- name: Build agent
  run: wadah pack -m wadah.yaml -o agent.wpkg

- name: Push to registry
  run: wadah push ghcr.io/${{ github.repository }}:${{ github.sha }}

- name: Deploy to K8s
  run: kubectl set image deployment/agent agent=ghcr.io/${{ github.repository }}:${{ github.sha }}
```

### 3. Multi-Environment

```
envs/
├── dev.yaml       # Minimal security, OpenAI
├── staging.yaml   # Standard security, cost limits
└── prod.yaml      # Strict security, audit enabled
```

```bash
wadah run -m envs/dev.yaml
wadah run -m envs/staging.yaml --trace staging.jsonl
wadah run -m envs/prod.yaml --trace audit.jsonl
```

---

## Next Steps

1. **Try Templates**: Explore `templates/` for real examples
   - `hello-world/` - Simplest agent
   - `rag-service/` - Document Q&A
   - `devops-copilot/` - Infrastructure automation

2. **Read Architecture**: [Layered Architecture](wadah_layered_architecture_diagram_docker_reuse.md)

3. **Explore Features**:
   - [Security Plugins](Plugins.md) - Optional security
   - [WadahSpec](WadahSpec-v0.1.md) - Agent manifest
   - [OAT](OAT.md) - Tracing format

4. **Deploy to Production**:
   - [Deployment Patterns](Deployment.md)
   - [Threat Model](ThreatModel.md)

---

## Troubleshooting

### API Key Issues
```bash
# Check environment
echo $OPENAI_API_KEY

# Or use Ollama (no key needed)
wadah run wadah.yaml  # with provider: ollama
```

### Docker Connection
```bash
# Test Docker is running
docker ps

# Test registry access
docker login ghcr.io
```

### Kubernetes Issues
```bash
# Check pods
kubectl get pods

# View logs
kubectl logs -l app=hello-agent

# Describe for events
kubectl describe pod <pod-name>
```

---

**You're ready!** Wadah works with your existing Docker/K8s knowledge.

Just add the intelligence layer. 🌊
