# DevOps Copilot Agent

An AI-powered DevOps assistant for infrastructure management, deployment automation, and operational tasks.

## Features

- **Kubernetes Management**: Deploy, monitor, and troubleshoot K8s clusters
- **Docker Operations**: Build, push, and manage container images
- **Git Integration**: Repository management and CI/CD workflows
- **Infrastructure as Code**: Generate and validate Terraform/Helm configs
- **Incident Response**: Automated troubleshooting and log analysis
- **Policy-Enforced**: Safe operations with ToolCaps restrictions

## Quick Start

```bash
# Set required environment variables
export OPENAI_API_KEY="your-key-here"
export KUBECONFIG="~/.kube/config"

# Check cluster status
wadah run wadah.yaml --prompt "Show me the current pods in the default namespace"

# Interactive mode for complex tasks
wadah run wadah.yaml --interactive
```

## Example Tasks

### Kubernetes Operations

```bash
# Check pod status
wadah run wadah.yaml --prompt "List all pods and their status"

# Analyze logs
wadah run wadah.yaml --prompt "Show logs for pod my-app-123 and identify errors"

# Deploy application
wadah run wadah.yaml --prompt "Deploy the application from ./k8s/deployment.yaml"
```

### Docker Workflows

```bash
# Build and push image
wadah run wadah.yaml --prompt "Build Docker image from current directory and push to registry"

# Inspect images
wadah run wadah.yaml --prompt "Show all local Docker images and their sizes"
```

### Git Operations

```bash
# Check repository status
wadah run wadah.yaml --prompt "What's the current git status and recent commits?"

# Create branch and commit
wadah run wadah.yaml --prompt "Create a new branch 'feature-x' and commit the changes"
```

## Safety & Security

### ToolCaps Protection

The agent is restricted by ToolCaps policies:
- **Allowed**: Read operations, safe deployments, log analysis
- **Denied**: Destructive commands, critical resource deletion
- **Rate Limited**: All tool calls have per-minute limits

### Audit Trail

All operations are traced:
```bash
# Run with tracing enabled
wadah run wadah.yaml --prompt "Your task" --trace traces/ops.jsonl

# Review operations
wadah trace stats traces/ops.jsonl
```

## Configuration

### Change Model

Edit `wadah.yaml`:
```yaml
runtime:
  model:
    provider: ollama  # or openai, tgi, vllm
    endpoint: http://localhost:11434
    modelId: llama3
```

### Adjust Permissions

Edit `ToolCaps.json` to modify allowed operations and rate limits.

### Budget Controls

Set spending limits in `wadah.yaml`:
```yaml
policy:
  budgets:
    tokens_per_minute: 300000
    usd_per_day: 100.0
    max_duration_secs: 1800
```

## Use Cases

- **Cluster Management**: Monitor and manage Kubernetes clusters
- **CI/CD Automation**: Automated deployments and rollbacks
- **Incident Response**: Rapid troubleshooting and log analysis
- **Infrastructure Audits**: Security and compliance checks
- **Documentation**: Auto-generate runbooks and procedures

## Architecture

```
User Request → DevOps Agent → ToolCaps Policy Check
                    ↓
              Execute (kubectl/docker/git)
                    ↓
              LLM Analysis → Response + Trace
```

## Best Practices

1. **Always review commands** before execution in production
2. **Use tracing** for audit and compliance
3. **Test in dev** environments first
4. **Set appropriate budgets** to control costs
5. **Review ToolCaps** regularly for security

## License

Apache 2.0

