# ToolCaps - Tool Capabilities & Policy System

## Overview

ToolCaps is Wadah's declarative policy system for controlling what tools an agent can use, which actions it can perform, and at what rate. It provides fine-grained, auditable access control for AI agents.

## Concept

While traditional sandboxes limit *processes*, ToolCaps limits *agent capabilities*. It answers questions like:

- Can this agent call GitHub APIs?
- How many requests per minute?
- Can it write to the filesystem?
- Which domains can it access?

## Format

ToolCaps are defined in JSON format as `ToolCaps.json`:

```json
{
  "version": "0.1",
  "allow": [...],
  "deny": [...]
}
```

## Schema

### Root Structure

```json
{
  "version": "0.1",      // Required: ToolCaps version
  "allow": [],           // Required: allowlist rules
  "deny": []             // Optional: denylist rules
}
```

### Allow Rules

```json
{
  "tool": "string",         // Required: tool identifier
  "actions": ["string"],    // Required: allowed actions (or [] for all)
  "limits": {...},          // Optional: rate limits
  "fs": {...},              // Optional: filesystem policy
  "domains": ["string"],    // Optional: network domains
  ...                       // Tool-specific extensions
}
```

#### Action Limits

```json
"limits": {
  "per_min": 100,          // Requests per minute
  "per_hour": 1000,        // Requests per hour
  "per_day": 10000         // Requests per day
}
```

#### Filesystem Policy

```json
"fs": {
  "paths": ["./workspace", "/tmp"],
  "write": true            // Allow writes
}
```

#### Network Domains

```json
"domains": [
  "api.github.com",        // Exact match
  "*.openai.com"           // Wildcard subdomain
]
```

### Deny Rules

Deny rules take precedence over allow rules.

```json
{
  "tool": "string",
  "actions": ["string"],   // Denied actions (or [] for all)
  "reason": "string"       // Optional: explanation
}
```

## Examples

### HTTP Tool Restrictions

```json
{
  "version": "0.1",
  "allow": [
    {
      "tool": "http",
      "actions": ["GET", "POST"],
      "limits": {
        "per_min": 100
      },
      "domains": [
        "api.openai.com",
        "api.github.com"
      ]
    }
  ],
  "deny": [
    {
      "tool": "http",
      "actions": ["DELETE"],
      "reason": "Destructive operations not allowed"
    }
  ]
}
```

### Kubernetes Tool

```json
{
  "tool": "kubectl",
  "actions": [
    "get",
    "describe",
    "logs",
    "apply"
  ],
  "limits": {
    "per_min": 30
  }
}
```

### Filesystem Access

```json
{
  "tool": "filesystem",
  "actions": ["read", "write"],
  "fs": {
    "paths": ["./workspace"],
    "write": true
  },
  "limits": {
    "per_min": 1000
  }
}
```

### Deny Destructive Commands

```json
{
  "version": "0.1",
  "allow": [
    {
      "tool": "shell",
      "actions": ["execute"],
      "limits": {"per_min": 10}
    }
  ],
  "deny": [
    {
      "tool": "shell",
      "actions": ["rm -rf", "dd", "mkfs"],
      "reason": "Destructive filesystem operations"
    },
    {
      "tool": "kubectl",
      "actions": ["delete namespace"],
      "reason": "Critical resource deletion"
    }
  ]
}
```

## Built-in Tools

### Standard Tools

| Tool ID | Description | Common Actions |
|---------|-------------|----------------|
| `http` | HTTP requests | GET, POST, PUT, DELETE, PATCH |
| `shell` | Shell execution | execute, read_only |
| `filesystem` | File operations | read, write, delete, list |
| `git` | Git operations | clone, pull, push, commit, status |
| `docker` | Docker management | ps, build, push, pull, inspect |
| `kubectl` | Kubernetes control | get, describe, logs, apply, delete |

### Tool-Specific Extensions

Different tools may support additional fields:

```json
{
  "tool": "database",
  "actions": ["SELECT", "INSERT"],
  "connection": {
    "host": "localhost",
    "database": "mydb",
    "read_only": true
  },
  "limits": {
    "per_min": 50,
    "max_rows": 1000
  }
}
```

## Enforcement

### Runtime Checks

ToolCaps are enforced at runtime by the `PolicyEnforcer`:

1. Agent requests tool action
2. Policy checks ToolCaps rules
3. If denied → error returned
4. If allowed → rate limit checked
5. Action executed and logged

### Trace Events

All policy checks are recorded in traces:

```json
{
  "event_type": "policy_check",
  "rule": "http:GET",
  "allowed": true,
  "reason": null
}
```

### Violations

When a policy is violated:

```rust
RuntimeError::PolicyViolation("Tool action not allowed: github::delete_repo")
```

## Best Practices

### 1. Principle of Least Privilege

Grant only the minimum required permissions:

```json
// Good: Specific actions
{
  "tool": "kubectl",
  "actions": ["get", "describe"]
}

// Bad: Wildcard permissions
{
  "tool": "kubectl",
  "actions": []  // Allows everything
}
```

### 2. Rate Limiting

Always set rate limits to prevent abuse:

```json
{
  "tool": "http",
  "limits": {
    "per_min": 100,   // Prevent API hammering
    "per_hour": 5000  // Daily budget control
  }
}
```

### 3. Explicit Denies

Use deny rules for critical operations:

```json
{
  "deny": [
    {
      "tool": "wallet",
      "actions": ["transfer", "approve"],
      "reason": "No financial operations in this environment"
    }
  ]
}
```

### 4. Domain Whitelisting

Restrict network access to known domains:

```json
{
  "tool": "http",
  "domains": [
    "api.openai.com",
    "*.github.com"
  ]
}
```

### 5. Separate Environments

Use different ToolCaps for dev/staging/prod:

```
ToolCaps.dev.json    # Permissive
ToolCaps.staging.json  # Moderate
ToolCaps.prod.json   # Strict
```

## Validation

### Syntax Check

```bash
wadah validate ToolCaps.json
```

### Testing Policies

```bash
# Dry run to test policy enforcement
wadah run wadah.yaml --dry-run --simulate-action "kubectl delete pod"
```

## Integration

### In WadahSpec

Reference ToolCaps in your manifest:

```yaml
policy:
  toolcaps: ToolCaps.json
```

### Runtime Loading

```rust
use wadah_spec::ToolCaps;

let caps = ToolCaps::from_file("ToolCaps.json")?;
let enforcer = PolicyEnforcer::new(Some(caps));

// Check permission
enforcer.check_tool_action("github", "delete_repo")?;
```

## Advanced Features

### Dynamic Policies (Future)

```json
{
  "tool": "http",
  "actions": ["GET"],
  "conditions": {
    "time": "09:00-17:00",
    "max_cost": 10.0
  }
}
```

### Policy Inheritance (Future)

```json
{
  "extends": "base-policy.json",
  "allow": [
    // Additional rules
  ]
}
```

### Audit Mode (Future)

```json
{
  "mode": "audit",  // Log violations but don't block
  "alert_threshold": 10
}
```

## Reference

- Implementation: `crates/spec/src/toolcaps.rs`
- Enforcement: `crates/runtime/src/policy.rs`
- Examples: `templates/*/ToolCaps.json`

## See Also

- [WadahSpec](WadahSpec-v0.1.md) - Agent manifest
- [Threat Model](ThreatModel.md) - Security considerations
- [Examples](../templates/) - Template agents

