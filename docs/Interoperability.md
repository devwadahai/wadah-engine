# Wadah Interoperability Specification v0.1

## Overview

This specification defines how Wadah integrates with popular AI/ML frameworks and tools in the ecosystem. Wadah is **framework-agnostic** - it packages and runs agents built with LangChain, LlamaIndex, or custom Python/TypeScript code.

**Design Principle**: Wadah provides the container/runtime layer. You bring your framework of choice.

---

## Supported Frameworks & Tools

### AI Frameworks
- ✅ **LangChain** - Agent chains and tools
- ✅ **LlamaIndex** - RAG and data frameworks
- 🔄 **CrewAI** - Multi-agent orchestration (planned)
- 🔄 **AutoGen** - Conversational agents (planned)
- ✅ **Custom Python/TypeScript** - Any framework

### Model Serving
- ✅ **OpenAI API** - GPT-4, GPT-3.5, compatible endpoints
- ✅ **TGI** (Text Generation Inference) - HuggingFace models
- ✅ **vLLM** - High-performance inference
- ✅ **Ollama** - Local model serving
- ✅ **Llama.cpp** - Quantized models

### Vector Databases
- ✅ **Qdrant** - Vector search engine
- ✅ **LanceDB** - Embedded vector database
- 🔄 **Weaviate** - Vector database (planned)
- 🔄 **Milvus** - Vector database (planned)
- ✅ **pgvector** - PostgreSQL extension

### Tool Integrations
- ✅ **Composio** - 100+ pre-built tool integrations
- ✅ **HTTP APIs** - Custom REST APIs
- 🔄 **LangChain Tools** - Native tool ecosystem (planned)
- 🔄 **WASM Tools** - Sandboxed execution (planned)

---

## Reference Implementation: LangChain + TGI + Qdrant + Composio

Complete working example showing all integration points.

### Project Structure

```
repo-assistant/
├── wadah.yaml              # WadahSpec manifest
├── ToolCaps.json           # Security policies
├── wadah.lock              # Auto-generated lockfile
├── prompts/
│   └── system.md
├── app/
│   ├── main.py             # LangChain entrypoint
│   └── requirements.txt
├── tools/
│   └── composio_github.yaml
├── tests/
│   └── conformance.yaml
├── data/
│   └── (qdrant index)
└── Makefile
```

### 1. WadahSpec Manifest

`wadah.yaml`:
```yaml
apiVersion: wadah.ai/v0.1
kind: Agent
metadata:
  name: repo-assistant
  version: 0.1.0
  description: "LangChain agent with TGI, Qdrant, and Composio"
  authors: ["Your Name <you@example.com>"]
  license: Apache-2.0

runtime:
  # Entrypoint for framework code
  entrypoint: python -m app.main:agent
  
  # Environment variables
  env:
    - name: COMPOSIO_API_KEY
    - name: QDRANT_URL
    - name: OPENAI_BASE         # Point to TGI
      value: http://tgi:8080/v1
  
  # Model configuration (TGI in this example)
  model:
    provider: tgi
    endpoint: http://tgi:8080
    modelId: mistral-7b-instruct
    params:
      temperature: 0.2
      top_p: 0.95
      max_tokens: 1024
      seed: 42                  # For deterministic replay
  
  # Vector memory (Qdrant)
  memory:
    type: vector
    backend: qdrant
    settings:
      url: ${QDRANT_URL:-http://localhost:6333}
      collection: repo_assistant
      persist: true
  
  # External tools (Composio)
  tools:
    - id: composio_github
      type: http
      manifest: tools/composio_github.yaml

# Security policies (optional)
plugins:
  - id: security.toolcaps
    enabled: true
  - id: security.budgets
    enabled: true
    config:
      tokens_per_minute: 200000
      usd_per_day: 25.0

policy:
  toolcaps: ToolCaps.json
  budgets:
    tokens_per_minute: 200000
    usd_per_day: 25.0
  network:
    allow_domains:
      - "api.github.com"
      - "raw.githubusercontent.com"

# Files to package
artifacts:
  include:
    - "prompts/**"
    - "app/**"
    - "tools/**"
    - "tests/**"
    - "requirements.txt"
```

### 2. Security Policies

`ToolCaps.json`:
```json
{
  "version": "0.1",
  "allow": [
    {
      "tool": "http",
      "actions": ["GET"],
      "domains": ["api.github.com", "raw.githubusercontent.com"],
      "limits": {"per_min": 60}
    },
    {
      "tool": "composio_github",
      "actions": ["read_issues", "create_issue"],
      "limits": {"per_min": 5}
    },
    {
      "tool": "fs",
      "actions": ["read"],
      "paths": ["/workspace/repo", "/workspace/prompts"]
    }
  ],
  "deny": [
    {
      "tool": "shell",
      "actions": ["exec"],
      "reason": "No arbitrary shell execution"
    },
    {
      "tool": "wallet",
      "actions": ["transfer"],
      "reason": "No financial operations"
    }
  ],
  "redaction": {
    "secrets": ["OPENAI_API_KEY", "COMPOSIO_API_KEY"]
  }
}
```

### 3. Tool Integration

`tools/composio_github.yaml`:
```yaml
id: composio_github
baseUrl: https://api.github.com
scopes:
  - read:issues
  - write:issues
headers:
  Authorization: Bearer ${COMPOSIO_API_KEY}
  Accept: application/vnd.github+json
```

### 4. LangChain Code

`app/main.py`:
```python
import os
from langchain_openai import ChatOpenAI
from langchain_core.prompts import ChatPromptTemplate
from langchain_core.runnables import RunnableLambda

# Load system prompt
SYSTEM_PROMPT = open("prompts/system.md").read()

# Model adapter (TGI via OpenAI-compatible API)
# OPENAI_BASE is set in wadah.yaml env
llm = ChatOpenAI(
    model=os.getenv("OPENAI_MODEL", "mistral-7b-instruct"),
    temperature=0.2
)

prompt = ChatPromptTemplate.from_messages([
    ("system", SYSTEM_PROMPT),
    ("human", "Summarize this repository: {repo_text}")
])

chain = prompt | llm | RunnableLambda(lambda x: x.content)

def agent(repo_text: str = ""):
    """Main agent entrypoint called by Wadah"""
    result = chain.invoke({"repo_text": repo_text})
    return {"summary": result}

if __name__ == "__main__":
    # For local testing
    print(agent(repo_text="Example repository content..."))
```

`app/requirements.txt`:
```
langchain>=0.1.0
langchain-openai>=0.0.5
qdrant-client>=1.7.0
```

### 5. Prompts

`prompts/system.md`:
```markdown
You are RepoAssistant, an AI that analyzes code repositories.

Your capabilities:
- Summarize repository structure and content
- Identify code patterns and potential issues
- Suggest actionable improvements

Constraints:
- Provide concise, actionable feedback
- Only suggest changes if policy permits
- Cite specific files when making recommendations

Always prioritize clarity and accuracy.
```

### 6. Conformance Tests

`tests/conformance.yaml`:
```yaml
name: repo-assistant-conformance
version: 0.1

tests:
  - name: deterministic_output
    type: run
    seed: 42
    input:
      repo_text: "This is a demo repo with two modules..."
    expect:
      contains: ["Summary", "actionable", "issue"]
      not_contains: ["error", "failed"]
  
  - name: policy_enforcement
    type: policy
    deny_actions:
      - "shell.exec"
      - "wallet.transfer"
    allow_actions:
      - "composio_github.read_issues"
  
  - name: budget_limits
    type: budget
    max_usd: 1.00
    max_tokens: 10000
```

### 7. Developer Workflow

`Makefile`:
```makefile
.PHONY: init pack run replay test push

init:
	python -m venv .venv
	. .venv/bin/activate && pip install -r app/requirements.txt

pack:
	wadah pack -m wadah.yaml -o build/repo-assistant.wpkg

run:
	wadah run build/repo-assistant.wpkg \
		--trace traces/run-$$(date +%Y%m%d-%H%M%S).jsonl

replay:
	wadah trace replay traces/run-*.jsonl --lock wadah.lock

test:
	wadah run build/repo-assistant.wpkg \
		--conformance tests/conformance.yaml

push:
	wadah push ghcr.io/yourorg/repo-assistant:0.1.0 \
		--package build/repo-assistant.wpkg

pull:
	wadah pull ghcr.io/yourorg/repo-assistant:0.1.0
```

---

## Integration Patterns

### Pattern 1: Model Backend Swap

Change model provider without code changes:

```yaml
# Use OpenAI
runtime:
  model:
    provider: openai
    modelId: gpt-4o-mini

# Use TGI
runtime:
  model:
    provider: tgi
    endpoint: http://tgi:8080
    modelId: mistral-7b-instruct

# Use Ollama
runtime:
  model:
    provider: ollama
    endpoint: http://localhost:11434
    modelId: llama3
```

LangChain code remains unchanged - Wadah handles the adapter.

### Pattern 2: Vector Database Swap

```yaml
# Qdrant
runtime:
  memory:
    type: vector
    backend: qdrant
    settings:
      url: http://localhost:6333

# LanceDB
runtime:
  memory:
    type: vector
    backend: lancedb
    settings:
      path: ./data/lancedb

# pgvector
runtime:
  memory:
    type: vector
    backend: pgvector
    settings:
      connection_string: postgresql://...
```

### Pattern 3: Tool Integration via Composio

```yaml
runtime:
  tools:
    - id: github
      type: http
      manifest: tools/github.yaml
    
    - id: slack
      type: http
      manifest: tools/slack.yaml
    
    - id: jira
      type: http
      manifest: tools/jira.yaml
```

Each tool manifest defines scopes, auth, and rate limits.

---

## Deployment with Docker/K8s

### Docker Compose Stack

`docker-compose.yml`:
```yaml
version: '3.8'

services:
  # Wadah runtime
  wadah:
    image: wadah/runtime:0.1.0
    command: wadah run /app/agent.wpkg
    environment:
      - COMPOSIO_API_KEY=${COMPOSIO_API_KEY}
      - QDRANT_URL=http://qdrant:6333
      - OPENAI_BASE=http://tgi:8080/v1
    volumes:
      - ./build/repo-assistant.wpkg:/app/agent.wpkg:ro
    depends_on:
      - tgi
      - qdrant
  
  # TGI model server
  tgi:
    image: ghcr.io/huggingface/text-generation-inference:latest
    command: --model-id mistralai/Mistral-7B-Instruct-v0.2
    ports:
      - "8080:8080"
    volumes:
      - tgi-data:/data
    environment:
      - HUGGING_FACE_HUB_TOKEN=${HF_TOKEN}
  
  # Qdrant vector database
  qdrant:
    image: qdrant/qdrant:latest
    ports:
      - "6333:6333"
    volumes:
      - qdrant-data:/qdrant/storage

volumes:
  tgi-data:
  qdrant-data:
```

Run:
```bash
docker-compose up
```

### Kubernetes Deployment

`k8s/agent-deployment.yaml`:
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: repo-assistant
spec:
  replicas: 2
  template:
    spec:
      containers:
      # Wadah agent
      - name: agent
        image: wadah/runtime:0.1.0
        command: ["wadah", "run", "/app/agent.wpkg"]
        env:
        - name: COMPOSIO_API_KEY
          valueFrom:
            secretKeyRef:
              name: api-keys
              key: composio
        - name: QDRANT_URL
          value: "http://qdrant-service:6333"
        - name: OPENAI_BASE
          value: "http://tgi-service:8080/v1"
        volumeMounts:
        - name: agent-package
          mountPath: /app
      
      volumes:
      - name: agent-package
        configMap:
          name: repo-assistant-wpkg
---
# TGI service
apiVersion: v1
kind: Service
metadata:
  name: tgi-service
spec:
  selector:
    app: tgi
  ports:
  - port: 8080
---
# Qdrant service
apiVersion: v1
kind: Service
metadata:
  name: qdrant-service
spec:
  selector:
    app: qdrant
  ports:
  - port: 6333
```

---

## Observability & Tracing

### OpenTelemetry Integration

Wadah emits OAT traces compatible with OpenTelemetry:

```yaml
# In K8s deployment
env:
- name: OTEL_EXPORTER_OTLP_ENDPOINT
  value: "http://jaeger-collector:4317"
- name: OTEL_SERVICE_NAME
  value: "repo-assistant"
```

### Trace Structure

Every agent execution produces an OAT trace:

```json
{
  "trace_id": "abc123",
  "agent_name": "repo-assistant",
  "spans": [
    {
      "span_id": "span-1",
      "kind": "agent",
      "name": "execution",
      "events": [
        {
          "type": "model_request",
          "model": "mistral-7b-instruct",
          "prompt": "Summarize...",
          "tokens": 150
        },
        {
          "type": "tool_call",
          "tool": "composio_github",
          "action": "read_issues",
          "duration_ms": 245
        },
        {
          "type": "model_response",
          "response": "Summary: ...",
          "tokens": 320,
          "cost": 0.002
        }
      ]
    }
  ]
}
```

---

## Acceptance Criteria

For a framework to be "Wadah-compatible":

### Required
- ✅ Can be invoked via entrypoint (function or CLI)
- ✅ Accepts environment variables for config
- ✅ Returns structured output (JSON, dict, etc.)
- ✅ Works with OpenAI-compatible model APIs

### Recommended
- 📊 Emits logs/traces in structured format
- 🔒 Respects policy constraints (ToolCaps)
- 🎯 Supports deterministic execution (seed parameter)
- 📦 Minimal dependencies (faster packaging)

### Optional
- 🔄 Native OAT tracing integration
- 🧩 Composio tool plugin support
- 💾 Direct vector DB integration

---

## Framework-Specific Guides

### LangChain Integration

**Best Practices:**
- Use `ChatOpenAI` with `OPENAI_BASE` env var for any provider
- Leverage `RunnableSequence` for composable chains
- Store prompts in `prompts/` directory
- Use `langchain-community` for tool integrations

**Example Patterns:**
```python
# Agent with tools
from langchain.agents import create_openai_functions_agent

agent = create_openai_functions_agent(
    llm=llm,
    tools=tools,  # Wadah provides these
    prompt=prompt
)

# RAG chain
from langchain.chains import RetrievalQA

qa_chain = RetrievalQA.from_chain_type(
    llm=llm,
    retriever=vector_store.as_retriever()  # Qdrant
)
```

### LlamaIndex Integration

**Best Practices:**
- Use `Settings.llm` for global model config
- Store indexes in `data/` directory
- Use `VectorStoreIndex` with Qdrant backend

**Example:**
```python
from llama_index.core import Settings, VectorStoreIndex
from llama_index.vector_stores.qdrant import QdrantVectorStore

# Wadah provides connection via env
vector_store = QdrantVectorStore(
    url=os.getenv("QDRANT_URL"),
    collection_name="docs"
)

index = VectorStoreIndex.from_vector_store(vector_store)
query_engine = index.as_query_engine()
```

---

## Roadmap

### v0.2 (Next Release)
- 🔄 Native LangChain tool integration
- 🔄 WASM tool sandbox
- 🔄 Weaviate/Milvus support
- 🔄 CrewAI multi-agent patterns

### v0.3 (Future)
- 🔮 AutoGen conversational patterns
- 🔮 Semantic Kernel integration
- 🔮 Tool marketplace (Composio Hub)
- 🔮 Multi-modal support (vision, speech)

---

## See Also

- [WadahSpec v0.1](WadahSpec-v0.1.md) - Manifest specification
- [Deployment Patterns](Deployment.md) - Docker/K8s integration
- [Security Plugins](Plugins.md) - ToolCaps and policies
- [OAT Specification](OAT.md) - Tracing format

---

**Wadah is framework-agnostic** - bring your preferred AI framework, we provide the container runtime. 🌊

