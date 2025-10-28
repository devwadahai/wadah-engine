# DevOps Copilot Agent

An AI-powered DevOps assistant for Kubernetes, CI/CD, and infrastructure management.

## Features

- 🚀 **Kubernetes Operations**: Deploy, scale, troubleshoot
- 🔄 **CI/CD Automation**: GitHub Actions, pipeline optimization
- 🐳 **Container Management**: Docker builds, registry operations
- 📊 **Monitoring**: Log analysis, incident response
- 🛡️ **Security**: Best practices, RBAC, network policies

## Quick Start

```bash
# 1. Set up credentials
export OPENAI_API_KEY='your-key'
export GITHUB_TOKEN='your-github-token'
export KUBECONFIG='path/to/kubeconfig'

# 2. Run the agent
wadah run wadah.yaml --interactive

# 3. Try some tasks
"Check the status of my prod cluster"
"Show me failing pods in namespace monitoring"
"Help me create a deployment for my app"
```

## Example Interactions

### Deploy an Application
```
You: Deploy my-app with 3 replicas to production
Agent: I'll help you deploy. First, let me check the current state...
       [runs kubectl commands]
       Here's the deployment manifest...
       [shows yaml]
       Shall I proceed with apply?
```

### Troubleshoot Issues
```
You: Pods in staging are crashing
Agent: Let me investigate:
       1. Checking pod status...
       2. Analyzing logs...
       3. Reviewing events...
       Found: OOMKilled errors. Your pods need more memory.
       Recommendation: Increase memory limit to 512Mi
```

### CI/CD Optimization
```
You: My GitHub Actions are slow
Agent: Analyzing your workflows...
       Issues found:
       1. No caching for dependencies
       2. Sequential jobs that could be parallel
       3. Building on every commit
       Here's an optimized workflow...
```

## Configuration

### Tools

The agent has access to:
- **kubectl**: All Kubernetes operations
- **github**: Repository and workflow management  
- **docker**: Container operations

### Security

- Network access limited to required domains
- File system restricted to workspace
- Budget limits to prevent runaway costs
- Full audit tracing enabled

### Customization

Edit `prompts/system.txt` to customize behavior:
- Add company-specific practices
- Include custom tool configurations
- Define preferred stack/technologies

## Use Cases

### Day-to-Day Operations
- Check cluster health
- Deploy applications
- Scale workloads
- Review logs and metrics

### Incident Response
- Identify failing services
- Analyze error patterns
- Suggest remediation steps
- Implement fixes

### CI/CD Management
- Optimize workflows
- Debug build failures
- Manage deployments
- Handle rollbacks

### Best Practices
- Security audits
- Resource optimization
- Documentation generation
- Runbook creation

## Tips

1. **Be Specific**: "Check prod cluster" is better than "check k8s"
2. **Provide Context**: Mention namespaces, apps, environments
3. **Iterative**: Work through complex tasks step-by-step
4. **Validate**: Always review commands before execution

## Security Notes

- Never commit real secrets to wadah.yaml
- Use environment variables for credentials
- Review tool configurations in ToolCaps.json
- Monitor budget limits for API costs

## Requirements

- OpenAI API key (GPT-4 recommended)
- Kubernetes access (kubeconfig)
- GitHub token (for CI/CD features)
- Docker (for container operations)

## License

Apache 2.0
