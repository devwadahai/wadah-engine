# Security Plugins Reference

## Overview

Wadah's security features are implemented as **optional plugins**. Enable only what you need.

## Philosophy

**Security should not block getting started.**

- 🟢 **Development**: No plugins - fast iteration
- 🟡 **Testing**: Budget plugin - basic safety  
- 🔴 **Production**: All plugins - full lockdown

## Available Plugins

### 1. `security.budgets`

**Prevents runaway costs and resource usage.**

```yaml
plugins:
  - id: security.budgets
    enabled: true
    config:
      tokens_per_minute: 100000  # Rate limit API calls
      usd_per_day: 50.0           # Daily spending cap
      max_duration_secs: 3600     # Max execution time
```

**When to use:**
- ✅ Always recommended for cloud APIs
- ✅ Protects against infinite loops
- ✅ Controls costs automatically

**Example:**
```bash
wadah init --security standard  # Includes budgets by default
```

### 2. `security.toolcaps`

**Controls which tools agents can use and how.**

Requires a `ToolCaps.json` file:

```yaml
plugins:
  - id: security.toolcaps
    enabled: true
```

```json
{
  "version": "0.1",
  "allow": [
    {
      "tool": "http",
      "actions": ["GET", "POST"],
      "limits": {"per_min": 100},
      "domains": ["api.openai.com", "api.github.com"]
    }
  ],
  "deny": [
    {
      "tool": "shell",
      "actions": ["rm", "dd"],
      "reason": "destructive operations not allowed"
    }
  ]
}
```

**When to use:**
- ✅ Untrusted agents
- ✅ Production environments
- ✅ Compliance requirements

**Learn more:** [ToolCaps Documentation](ToolCaps.md)

### 3. `security.network`

**Restricts network access by domain.**

```yaml
plugins:
  - id: security.network
    enabled: true

policy:
  network:
    allow_domains:
      - "api.openai.com"
      - "*.github.com"
    deny_domains:
      - "suspicious.com"
```

**When to use:**
- ✅ Prevent data exfiltration
- ✅ Whitelist known APIs
- ✅ Block malicious domains

### 4. `security.filesystem`

**Controls file access permissions.**

```yaml
plugins:
  - id: security.filesystem
    enabled: true

policy:
  filesystem:
    allow_paths:
      - "./workspace"
      - "./data"
    deny_paths:
      - "/etc"
      - "~/.ssh"
    read_only: true  # No writes allowed
```

**When to use:**
- ✅ Protect sensitive files
- ✅ Restrict agent to specific directories
- ✅ Read-only agents

### 5. `observability.tracing`

**Records all agent actions for audit and replay.**

```yaml
plugins:
  - id: observability.tracing
    enabled: true
```

Then run with:
```bash
wadah run agent.yaml --trace audit.jsonl
```

**When to use:**
- ✅ Production deployments
- ✅ Debugging
- ✅ Compliance/audit trails
- ✅ Deterministic replay

**Learn more:** [OpenAgentTrace (OAT)](OAT.md)

## Security Presets

### Permissive (No Plugins)

```yaml
# wadah.yaml
apiVersion: wadah.ai/v0.1
kind: Agent
metadata:
  name: dev-agent
  version: 0.1.0
  authors: ["Dev"]
runtime:
  model:
    provider: openai
    modelId: gpt-4o-mini

# No plugins section = fully open
```

**CLI:**
```bash
wadah init --security minimal
wadah run agent.yaml --security permissive
```

**Use for:**
- Local experiments
- Quick prototypes
- Trusted environments

### Standard (Budget Plugin)

```yaml
plugins:
  - id: security.budgets
    enabled: true
    config:
      usd_per_day: 100.0
      max_duration_secs: 3600
```

**CLI:**
```bash
wadah init --security standard  # Default
```

**Use for:**
- Most applications
- Development with safety nets
- Cost-conscious deployments

### Strict (All Plugins)

```yaml
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
```

**CLI:**
```bash
wadah init --security strict
```

**Use for:**
- Production systems
- Untrusted agents
- High-security environments
- Compliance requirements

## Mixing & Matching

Enable only what you need:

```yaml
# Just budgets and tracing
plugins:
  - id: security.budgets
    enabled: true
    config:
      usd_per_day: 20.0
  - id: observability.tracing
    enabled: true
```

## Runtime Overrides

Override security level at runtime:

```bash
# Agent has strict security, but run permissive for testing
wadah run strict-agent.yaml --security permissive

# Agent has no security, but enforce strict for production
wadah run dev-agent.yaml --security strict --trace audit.jsonl
```

## Checking Plugin Status

```bash
# List available plugins
wadah plugins

# Detailed information
wadah plugins --verbose

# Check agent's plugins
wadah verify agent.wpkg
```

## Plugin Configuration Reference

### Budget Plugin

```yaml
plugins:
  - id: security.budgets
    enabled: true
    config:
      tokens_per_minute: int     # API rate limit
      usd_per_day: float         # Daily cost cap
      max_duration_secs: int     # Execution timeout
```

### ToolCaps Plugin

```yaml
plugins:
  - id: security.toolcaps
    enabled: true
    # Configuration in ToolCaps.json
```

### Network Plugin

```yaml
plugins:
  - id: security.network
    enabled: true
    # Configuration in policy.network section
```

### Filesystem Plugin

```yaml
plugins:
  - id: security.filesystem
    enabled: true
    # Configuration in policy.filesystem section
```

### Tracing Plugin

```yaml
plugins:
  - id: observability.tracing
    enabled: true
    config:
      sample_rate: 1.0           # Optional: sampling (0.0-1.0)
      redact_secrets: true       # Optional: auto-redact sensitive data
```

## Best Practices

### 1. Start Minimal, Add Gradually

```bash
# Week 1: No security
wadah init myagent --security minimal

# Week 2: Add budgets
vim wadah.yaml  # Add security.budgets plugin

# Week 3: Lock down for prod
wadah init prod-agent --security strict
```

### 2. Different Configs for Different Environments

```
project/
├── wadah.dev.yaml    # Minimal security
├── wadah.staging.yaml # Standard security
└── wadah.prod.yaml   # Strict security
```

```bash
wadah run wadah.dev.yaml     # Development
wadah run wadah.prod.yaml    # Production
```

### 3. Use Tracing in Production

Always enable tracing for production:

```yaml
plugins:
  - id: observability.tracing
    enabled: true
```

```bash
wadah run agent.yaml --trace logs/$(date +%Y%m%d).jsonl
```

### 4. Review Plugin Configs Regularly

```bash
# Audit security settings
cat wadah.yaml | grep -A 10 "plugins:"

# Check effective policies
wadah run agent.yaml --dry-run --show-policies
```

## Migration Guide

### From Policy to Plugins

**Old (still works):**
```yaml
policy:
  budgets:
    usd_per_day: 10.0
```

**New (preferred):**
```yaml
plugins:
  - id: security.budgets
    enabled: true
    config:
      usd_per_day: 10.0
```

### Why Plugins?

- 🔧 More modular and composable
- 🎯 Enable only what you need
- 🚀 Faster for development (no security overhead)
- 📦 Easier to extend with community plugins

## See Also

- [WadahSpec](WadahSpec-v0.1.md) - Agent manifest format
- [ToolCaps](ToolCaps.md) - Fine-grained permissions
- [Threat Model](ThreatModel.md) - Security deep-dive
- [Quickstart](Quickstart.md) - Get started in 2 minutes

