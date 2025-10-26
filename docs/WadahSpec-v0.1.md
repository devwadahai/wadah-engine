# WadahSpec v0.1 Specification

## Overview

WadahSpec is the declarative manifest format for defining AI agents in Wadah. Think of it as the `Dockerfile` for AI agents.

**Philosophy**: Start minimal, add complexity only when needed.

## Minimal Example

The simplest possible agent:

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

That's it! No security policies required for development.

## Format

WadahSpec manifests are written in YAML and follow a versioned schema.

## Schema

### Root Structure

```yaml
apiVersion: wadah.ai/v0.1
kind: Agent
metadata: {...}           # Required
runtime: {...}            # Required
plugins: [...]            # Optional: Security features
policy: {...}             # Optional: Legacy security (use plugins instead)
artifacts: {...}          # Optional: Files to package
lock: {...}               # Optional: Generated during packaging
```

### `metadata` (required)

Basic identification for your agent.

```yaml
metadata:
  name: string              # Required: lowercase, alphanumeric + hyphens
  version: string           # Required: semver format (e.g., "0.1.0")
  description: string       # Optional
  authors: [string]         # Required: at least one author
  license: string           # Optional (e.g., "Apache-2.0", "MIT")
  tags: [string]            # Optional: searchable tags
```

**Validation Rules:**
- `name`: Must match `^[a-z0-9-]+$`
- `version`: Must follow semantic versioning (X.Y.Z)
- `authors`: Recommended format: "Name <email>"

### `runtime` (required)

Configures the execution environment.

```yaml
runtime:
  entrypoint: string        # Optional: custom entrypoint (e.g., "python -m app.main:agent")
  model: {...}              # Required
  memory: {...}             # Optional
  tools: [{...}]            # Optional
  env: [{...}]              # Optional: environment variables
```

#### `runtime.entrypoint` (optional)

Custom entrypoint for framework-based agents (LangChain, LlamaIndex, etc.).

```yaml
# Python module
entrypoint: python -m app.main:agent

# Node.js
entrypoint: node dist/agent.js

# Binary
entrypoint: ./agent
```

**Use cases:**
- LangChain/LlamaIndex agents
- Custom Python/TypeScript code
- Compiled binaries

If not specified, Wadah uses default runtime behavior.

#### `runtime.model` (required)

Specifies which AI model to use.

```yaml
model:
  provider: string          # Required: openai, ollama, tgi, vllm
  endpoint: string          # Optional: API endpoint
  modelId: string           # Required: model identifier
  params:                   # Optional: model parameters
    temperature: float      # Optional: 0.0-2.0
    top_p: float            # Optional: 0.0-1.0
    max_tokens: int         # Optional
    seed: int               # Optional: for deterministic output
```

**Supported Providers:**
- `openai`: OpenAI API or compatible (requires `OPENAI_API_KEY`)
- `ollama`: Local Ollama instance (free, no key needed)
- `tgi`: Text Generation Inference
- `vllm`: vLLM inference server

**Examples:**

OpenAI:
```yaml
model:
  provider: openai
  modelId: gpt-4o-mini
```

Ollama (local):
```yaml
model:
  provider: ollama
  endpoint: http://localhost:11434
  modelId: llama3
```

#### `runtime.memory` (optional)

Persistent memory for your agent.

```yaml
memory:
  type: string              # vector, rag, kv, none
  backend: string           # qdrant, lancedb, pgvector, redis
  index: string             # Path to index/database (for file-based)
  settings: {...}           # Backend-specific config
```

**Examples:**

Qdrant (vector DB):
```yaml
memory:
  type: vector
  backend: qdrant
  settings:
    url: ${QDRANT_URL}
    collection: my_docs
    persist: true
```

LanceDB (embedded):
```yaml
memory:
  type: vector
  backend: lancedb
  settings:
    path: ./data/lancedb
```

Redis (KV store):
```yaml
memory:
  type: kv
  backend: redis
  settings:
    url: redis://localhost:6379
```

#### `runtime.tools` (optional)

External tools your agent can use.

```yaml
tools:
  - id: string              # Unique identifier
    type: string            # http, wasm, process, builtin
    manifest: string        # Path to tool config
    config: {...}           # Tool-specific settings
```

#### `runtime.env` (optional)

Environment variables. Supports both object and array format.

**Object format** (simple):
```yaml
env:
  OPENAI_API_KEY: "${OPENAI_API_KEY}"  # From host environment
  CUSTOM_VAR: "value"
```

**Array format** (Kubernetes-style):
```yaml
env:
  - name: OPENAI_API_KEY
    value: "${OPENAI_API_KEY}"
  - name: QDRANT_URL
    value: "http://localhost:6333"
  - name: COMPOSIO_API_KEY  # Value from host env
```

The array format is recommended for complex setups and matches Kubernetes conventions.

### `plugins` (optional) 🔌

**New in v0.1!** Security features as opt-in plugins.

```yaml
plugins:
  - id: security.budgets
    enabled: true
    config:
      usd_per_day: 10.0
      max_duration_secs: 3600
  
  - id: security.toolcaps
    enabled: true
    # config loaded from ToolCaps.json
  
  - id: observability.tracing
    enabled: true
```

**Available Plugins:**
- `security.toolcaps` - Tool permission enforcement
- `security.budgets` - Cost/token/time limits
- `security.network` - Domain whitelisting
- `security.filesystem` - File access control
- `observability.tracing` - Execution audit logs

See `wadah plugins` for full list.

### `policy` (optional, legacy)

**Note**: Using `plugins` is now preferred. `policy` is kept for backward compatibility.

```yaml
policy:
  toolcaps: ToolCaps.json   # Path to policy file
  budgets:
    tokens_per_minute: 100000
    usd_per_day: 50.0
    max_duration_secs: 3600
  network:
    allow_domains: ["api.openai.com"]
  filesystem:
    allow_paths: ["./workspace"]
    read_only: true
```

### `artifacts` (optional)

Files to include when packaging.

```yaml
artifacts:
  include:
    - "prompts/**"
    - "code/**"
  exclude:
    - "**/*.log"
    - "**/node_modules/**"
```

### `lock` (generated)

Auto-generated during `wadah pack` for reproducibility.

```yaml
lock:
  version: "0.1"
  generated_at: "2025-10-19T..."
  model:
    provider: openai
    model_id: gpt-4o-mini
    digest: "sha256:..."
  tools: [...]
  artifacts: {...}
```

## Security Levels

### Minimal (No Security)

```yaml
apiVersion: wadah.ai/v0.1
kind: Agent
metadata:
  name: quick-test
  version: 0.1.0
  authors: ["Dev"]
runtime:
  model:
    provider: openai
    modelId: gpt-4o-mini

# No plugins = permissive mode
```

**Use for**: Local development, quick experiments

### Standard (Budget Limits)

```yaml
apiVersion: wadah.ai/v0.1
kind: Agent
metadata:
  name: safe-agent
  version: 0.1.0
  authors: ["Dev"]
runtime:
  model:
    provider: openai
    modelId: gpt-4o-mini

plugins:
  - id: security.budgets
    enabled: true
    config:
      usd_per_day: 100.0
```

**Use for**: Most applications, reasonable safety

### Strict (Full Security)

```yaml
apiVersion: wadah.ai/v0.1
kind: Agent
metadata:
  name: prod-agent
  version: 1.0.0
  authors: ["Team <team@company.com>"]
  license: Apache-2.0
  tags: [production]

runtime:
  model:
    provider: openai
    modelId: gpt-4o
    params:
      temperature: 0.2
      seed: 1337

plugins:
  - id: security.toolcaps
    enabled: true
  - id: security.budgets
    enabled: true
    config:
      usd_per_day: 50.0
      max_duration_secs: 1800
  - id: security.network
    enabled: true
  - id: security.filesystem
    enabled: true
  - id: observability.tracing
    enabled: true

policy:
  toolcaps: ToolCaps.json
  network:
    allow_domains: ["api.openai.com"]
  filesystem:
    allow_paths: ["./workspace"]
    read_only: true

artifacts:
  include: ["prompts/**", "code/**"]
```

**Use for**: Production, untrusted agents, compliance requirements

## Validation

```bash
# Check syntax
wadah pack -m wadah.yaml --dry-run

# Verify package
wadah verify agent.wpkg
```

## Complete Examples

### Hello World

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

### With Ollama (Local, Free)

```yaml
apiVersion: wadah.ai/v0.1
kind: Agent
metadata:
  name: local-agent
  version: 0.1.0
  authors: ["You"]
runtime:
  model:
    provider: ollama
    endpoint: http://localhost:11434
    modelId: llama3
```

### Production Agent

```yaml
apiVersion: wadah.ai/v0.1
kind: Agent
metadata:
  name: production-agent
  version: 1.0.0
  description: "Production-ready with full security"
  authors: ["Team"]
  license: Apache-2.0

runtime:
  model:
    provider: openai
    modelId: gpt-4o
    params:
      temperature: 0.2
      max_tokens: 8192
      seed: 1337

plugins:
  - id: security.budgets
    enabled: true
    config:
      usd_per_day: 50.0
  - id: observability.tracing
    enabled: true

artifacts:
  include: ["prompts/**", "code/**"]
```

## Migration from Earlier Versions

If you have a spec with `policy` section, it still works! But consider migrating to `plugins`:

**Old style:**
```yaml
policy:
  budgets:
    usd_per_day: 10.0
```

**New style (preferred):**
```yaml
plugins:
  - id: security.budgets
    enabled: true
    config:
      usd_per_day: 10.0
```

## Tips

1. **Start minimal** - Add security as you need it
2. **Use Ollama** for free local development
3. **Enable tracing** for production (`observability.tracing`)
4. **Pin seeds** for reproducible outputs
5. **Use lockfiles** in CI/CD

## See Also

- [Quickstart](Quickstart.md) - 2-minute tutorial
- [Security Plugins](Plugins.md) - Plugin reference
- [ToolCaps](ToolCaps.md) - Advanced permissions (optional)
- [Templates](../templates/) - Example agents
