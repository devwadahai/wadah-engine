# Deployment Patterns - Wadah on Docker/K8s

This guide shows how to deploy Wadah agents using Docker and Kubernetes infrastructure you already know.

## Overview

**Wadah rides on Docker/K8s rails** - we reuse:
- ✅ OCI registries (GHCR, DockerHub, ECR)
- ✅ Container isolation (namespaces, cgroups)
- ✅ K8s orchestration (scheduling, autoscaling)
- ✅ Networking (Services, Ingress)
- ✅ Storage (Volumes, PVCs)
- ✅ Observability (Prometheus, Grafana)

**Wadah adds** the intelligence layer:
- 🆕 Agent packaging (`.wpkg`)
- 🆕 Reasoning traces (OAT)
- 🆕 AI-level policies (ToolCaps)
- 🆕 Multi-agent orchestration

---

## Pattern 1: Local Development (No Container)

**Use case**: Quick iteration, debugging

```bash
wadah init myagent --security minimal
cd myagent
wadah run wadah.yaml --prompt "test"
```

**Pros**:
- Fastest iteration
- Easy debugging
- No Docker overhead

**Cons**:
- Not production-ready
- No isolation

---

## Pattern 2: Wadah in Docker (Single Host)

**Use case**: Isolated execution, reproducible environments

### Basic Docker Run

```bash
# Build agent package
wadah pack -m wadah.yaml -o agent.wpkg

# Run in Docker
docker run -it --rm \
  -e OPENAI_API_KEY="sk-..." \
  -v $(pwd)/agent.wpkg:/app/agent.wpkg:ro \
  wadah/runtime:latest \
  wadah run /app/agent.wpkg --prompt "Hello from Docker!"
```

### With GPU Support

```bash
docker run -it --rm \
  --gpus all \
  -e OPENAI_API_KEY="sk-..." \
  -v $(pwd)/agent.wpkg:/app/agent.wpkg \
  wadah/runtime:latest-gpu \
  wadah run /app/agent.wpkg
```

### Custom Dockerfile

Create `Dockerfile`:
```dockerfile
FROM wadah/runtime:0.1.0

# Copy agent
COPY build/agent.wpkg /app/agent.wpkg

# Copy data volumes
COPY data/ /app/data/

# Set working directory
WORKDIR /app

# Entrypoint
ENTRYPOINT ["wadah", "run", "/app/agent.wpkg"]
CMD ["--interactive"]
```

Build and run:
```bash
docker build -t my-agent:v1 .
docker run -e OPENAI_API_KEY="sk-..." my-agent:v1
```

### Docker Compose (Multiple Services)

`docker-compose.yml`:
```yaml
version: '3.8'

services:
  agent:
    image: wadah/runtime:latest
    command: wadah run /app/agent.wpkg --interactive
    environment:
      - OPENAI_API_KEY=${OPENAI_API_KEY}
    volumes:
      - ./agent.wpkg:/app/agent.wpkg:ro
      - agent-data:/data
    depends_on:
      - vector-db
  
  vector-db:
    image: qdrant/qdrant:latest
    ports:
      - "6333:6333"
    volumes:
      - qdrant-data:/qdrant/storage
  
  redis:
    image: redis:7-alpine
    volumes:
      - redis-data:/data

volumes:
  agent-data:
  qdrant-data:
  redis-data:
```

Run:
```bash
docker-compose up
```

---

## Pattern 3: Wadah on Kubernetes (Cluster)

**Use case**: Production, autoscaling, multi-agent

### 3.1 Basic Deployment

`agent-deployment.yaml`:
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: wadah-agent
  namespace: agents
spec:
  replicas: 3
  selector:
    matchLabels:
      app: wadah-agent
  template:
    metadata:
      labels:
        app: wadah-agent
    spec:
      containers:
      - name: wadahd
        image: wadah/runtime:0.1.0
        command:
          - wadah
          - run
          - /app/agent.wpkg
        env:
        - name: OPENAI_API_KEY
          valueFrom:
            secretKeyRef:
              name: openai-secret
              key: api-key
        - name: OTEL_EXPORTER_OTLP_ENDPOINT
          value: "http://otel-collector:4317"
        volumeMounts:
        - name: agent
          mountPath: /app
          readOnly: true
        - name: data
          mountPath: /data
        resources:
          requests:
            memory: "512Mi"
            cpu: "500m"
          limits:
            memory: "2Gi"
            cpu: "2000m"
      volumes:
      - name: agent
        configMap:
          name: wadah-agent-config
      - name: data
        persistentVolumeClaim:
          claimName: agent-data-pvc
```

Deploy:
```bash
# Create namespace
kubectl create namespace agents

# Create secret
kubectl create secret generic openai-secret \
  -n agents \
  --from-literal=api-key="sk-..."

# Create ConfigMap
kubectl create configmap wadah-agent-config \
  -n agents \
  --from-file=agent.wpkg=build/agent.wpkg

# Deploy
kubectl apply -f agent-deployment.yaml

# Check status
kubectl get pods -n agents -l app=wadah-agent
```

### 3.2 With Autoscaling

`hpa.yaml`:
```yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: wadah-agent-hpa
  namespace: agents
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: wadah-agent
  minReplicas: 2
  maxReplicas: 10
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
  - type: Pods
    pods:
      metric:
        name: wadah_tokens_per_second
      target:
        type: AverageValue
        averageValue: "1000"
```

### 3.3 With Service & Ingress

`service.yaml`:
```yaml
apiVersion: v1
kind: Service
metadata:
  name: wadah-agent-service
  namespace: agents
spec:
  selector:
    app: wadah-agent
  ports:
  - port: 8080
    targetPort: 8080
    name: http
  - port: 9090
    targetPort: 9090
    name: metrics
  type: ClusterIP
```

`ingress.yaml`:
```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: wadah-agent-ingress
  namespace: agents
  annotations:
    cert-manager.io/cluster-issuer: letsencrypt-prod
spec:
  tls:
  - hosts:
    - agent.example.com
    secretName: agent-tls
  rules:
  - host: agent.example.com
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: wadah-agent-service
            port:
              number: 8080
```

### 3.4 With GPU Node Pool

`gpu-deployment.yaml`:
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: wadah-agent-gpu
spec:
  replicas: 1
  template:
    spec:
      nodeSelector:
        accelerator: nvidia-tesla-v100
      containers:
      - name: wadahd
        image: wadah/runtime:latest-gpu
        resources:
          limits:
            nvidia.com/gpu: 1
```

### 3.5 With Vector Database

Complete stack with Qdrant:

`qdrant-deployment.yaml`:
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: qdrant
spec:
  replicas: 1
  template:
    spec:
      containers:
      - name: qdrant
        image: qdrant/qdrant:latest
        ports:
        - containerPort: 6333
        volumeMounts:
        - name: qdrant-storage
          mountPath: /qdrant/storage
      volumes:
      - name: qdrant-storage
        persistentVolumeClaim:
          claimName: qdrant-pvc
---
apiVersion: v1
kind: Service
metadata:
  name: qdrant-service
spec:
  selector:
    app: qdrant
  ports:
  - port: 6333
    targetPort: 6333
```

Update agent to use Qdrant:
```yaml
# In wadah.yaml
runtime:
  memory:
    type: rag
    index: http://qdrant-service:6333
```

---

## Pattern 4: Hybrid (Edge + Cloud)

**Use case**: Low-latency local tools + cloud model inference

```
┌────────────────┐          ┌─────────────────┐
│  Edge Device   │          │  Cloud K8s      │
│                │          │                 │
│  • Wadah       │  ←───→   │  • vLLM/TGI     │
│  • Local tools │          │  • Vector DB    │
│  • Cache       │          │  • Monitoring   │
└────────────────┘          └─────────────────┘
```

Edge `wadah.yaml`:
```yaml
runtime:
  model:
    provider: tgi
    endpoint: https://cloud-tgi.example.com
    modelId: llama-70b
  tools:
    - id: local-sensors
      type: process
  memory:
    type: kv
    index: ./local-cache
```

---

## Pattern 5: CI/CD Integration

### GitHub Actions

`.github/workflows/deploy-agent.yml`:
```yaml
name: Deploy Agent

on:
  push:
    branches: [main]

jobs:
  build-and-deploy:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Wadah
      run: cargo install wadah
    
    - name: Build agent package
      run: wadah pack -m wadah.yaml -o agent.wpkg
    
    - name: Login to GHCR
      uses: docker/login-action@v2
      with:
        registry: ghcr.io
        username: ${{ github.actor }}
        password: ${{ secrets.GITHUB_TOKEN }}
    
    - name: Push to registry
      run: |
        wadah push ghcr.io/${{ github.repository }}:${{ github.sha }}
        wadah push ghcr.io/${{ github.repository }}:latest
    
    - name: Deploy to K8s
      uses: azure/k8s-deploy@v4
      with:
        manifests: |
          k8s/deployment.yaml
        images: |
          ghcr.io/${{ github.repository }}:${{ github.sha }}
```

### GitLab CI

`.gitlab-ci.yml`:
```yaml
stages:
  - build
  - deploy

build:
  stage: build
  script:
    - cargo install wadah
    - wadah pack -m wadah.yaml -o agent.wpkg
    - wadah push $CI_REGISTRY_IMAGE:$CI_COMMIT_SHA
  artifacts:
    paths:
      - agent.wpkg

deploy:
  stage: deploy
  script:
    - kubectl set image deployment/wadah-agent \
        agent=$CI_REGISTRY_IMAGE:$CI_COMMIT_SHA
  only:
    - main
```

---

## Observability Integration

### Prometheus Metrics

Wadah exposes metrics at `:9090/metrics`:
```
wadah_tokens_total{agent="myagent",model="gpt-4"}
wadah_cost_usd{agent="myagent"}
wadah_execution_duration_seconds{agent="myagent"}
wadah_policy_violations_total{agent="myagent",rule="toolcaps"}
```

`ServiceMonitor`:
```yaml
apiVersion: monitoring.coreos.com/v1
kind: ServiceMonitor
metadata:
  name: wadah-agent
spec:
  selector:
    matchLabels:
      app: wadah-agent
  endpoints:
  - port: metrics
    interval: 30s
```

### Grafana Dashboard

Import dashboard ID: `wadah-agent-overview` (coming soon)

Panels:
- Tokens per minute
- Cost per day
- Execution latency (p50, p95, p99)
- Policy violations
- Active agents

### Jaeger Tracing

Enable OTLP export:
```yaml
# In K8s deployment
env:
- name: OTEL_EXPORTER_OTLP_ENDPOINT
  value: "http://jaeger-collector:4317"
- name: OTEL_SERVICE_NAME
  value: "wadah-agent"
```

View traces in Jaeger UI showing:
- Full agent execution flow
- Model API calls (timing, tokens)
- Tool invocations
- Memory operations

---

## Storage Patterns

### 1. Agent Package Storage

**Option A: ConfigMap** (< 1MB)
```bash
kubectl create configmap agent-pkg --from-file=agent.wpkg
```

**Option B: PVC** (larger packages)
```yaml
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: agent-storage
spec:
  accessModes:
    - ReadWriteMany
  resources:
    requests:
      storage: 10Gi
```

**Option C: Pull from Registry** (recommended)
```yaml
initContainers:
- name: pull-agent
  image: wadah/cli:latest
  command:
    - wadah
    - pull
    - ghcr.io/org/agent:latest
    - --output
    - /app/agent.wpkg
  volumeMounts:
  - name: agent-storage
    mountPath: /app
```

### 2. Data/Memory Storage

```yaml
volumes:
- name: vector-db
  persistentVolumeClaim:
    claimName: qdrant-pvc
- name: redis-cache
  emptyDir: {}
- name: agent-data
  persistentVolumeClaim:
    claimName: agent-data-pvc
    storageClassName: fast-ssd
```

---

## Security Best Practices

### 1. Non-Root User

```dockerfile
FROM wadah/runtime:latest
USER 1000:1000
```

### 2. Read-Only Root Filesystem

```yaml
securityContext:
  readOnlyRootFilesystem: true
  runAsNonRoot: true
  runAsUser: 1000
```

### 3. Network Policies

```yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: wadah-agent-policy
spec:
  podSelector:
    matchLabels:
      app: wadah-agent
  policyTypes:
  - Egress
  egress:
  - to:
    - namespaceSelector:
        matchLabels:
          name: kube-system
    ports:
    - protocol: TCP
      port: 53  # DNS
  - to:
    - podSelector:
        matchLabels:
          app: qdrant
    ports:
    - protocol: TCP
      port: 6333
  - to:  # OpenAI API
    - ipBlock:
        cidr: 0.0.0.0/0
    ports:
    - protocol: TCP
      port: 443
```

### 4. Pod Security Standards

```yaml
apiVersion: v1
kind: Namespace
metadata:
  name: agents
  labels:
    pod-security.kubernetes.io/enforce: restricted
    pod-security.kubernetes.io/audit: restricted
    pod-security.kubernetes.io/warn: restricted
```

---

## Cost Optimization

### 1. Spot Instances

```yaml
nodeSelector:
  workload-type: spot
tolerations:
- key: spot
  operator: Equal
  value: "true"
  effect: NoSchedule
```

### 2. Cluster Autoscaler

Automatically scale nodes based on Wadah workload.

### 3. Budget Controls

```yaml
# In wadah.yaml
plugins:
  - id: security.budgets
    config:
      usd_per_day: 50.0
      tokens_per_minute: 100000
```

---

## Troubleshooting

### Check Logs

```bash
kubectl logs -l app=wadah-agent -n agents --tail=100 -f
```

### Debug Pod

```bash
kubectl run -it --rm debug \
  --image=wadah/runtime:latest \
  --restart=Never \
  -- /bin/sh
```

### Check Events

```bash
kubectl get events -n agents --sort-by='.lastTimestamp'
```

---

## Next Steps

- [Architecture Deep Dive](wadah_layered_architecture_diagram_docker_reuse.md)
- [Security Hardening](ThreatModel.md)
- [Monitoring Guide](Observability.md)

---

**Wadah integrates seamlessly with Docker/K8s** - use the tools you know, add the intelligence layer. 🌊

