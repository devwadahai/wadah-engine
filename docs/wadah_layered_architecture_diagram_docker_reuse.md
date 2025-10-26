# Wadah Layered Architecture (with Docker/K8s Reuse)

## Visual Map (High-Level)
```
+────────────────────────────────────────────────────────────────────+
|                          User Interfaces                           |
|  Wadah CLI  •  Web Dashboard  •  SDKs (Rust/Python/TS)             |
+────────────────────────────────────────────────────────────────────+
|                Orchestration & Observability Layer                 |
|  Schedulers (Kubernetes/Nomad) • Autoscale • OAT Tracing/OTLP      |
|  Cost/Token Metering • Prometheus/Grafana Integrations             |
+────────────────────────────────────────────────────────────────────+
|                       Intelligence Orchestrator                    |
|  Multi-agent graphs • Routing • Eval • Guardrails/Policies         |
+────────────────────────────────────────────────────────────────────+
|                         Runtime (Wadahd Core)                       |
|  Agent Lifecycle • Policy VM (ToolCaps) • Sandboxing (WASM/Proc)   |
|  I/O Bridges (HTTP/gRPC/WebSocket/MQTT) • Secrets/Vault            |
+────────────────────────────────────────────────────────────────────+
|                          Memory / Data Layer                        |
|  Vector DB (Qdrant/LanceDB/pgvector) • Redis Cache • RAG Stores    |
|  Knowledge Graph (Neo4j/Vespa optional) • Local Volumes            |
+────────────────────────────────────────────────────────────────────+
|                 Model Serving & Adapters (Pluggable)               |
|  OpenAI API • vLLM • TGI • Ollama • Llama.cpp • Speech/Vision      |
+────────────────────────────────────────────────────────────────────+
|                 System & Distribution (REUSED STACK)               |
|  Docker/Podman (OCI) • Container Registry (GHCR/ECR/Hub)           |
|  Kubernetes/Nomad • Linux Namespaces/Cgroups • GPUs (CUDA/ROCm)    |
+────────────────────────────────────────────────────────────────────+
```

## Where Docker/K8s Fit (We’re *Not* Re‑Inventing)
- **OCI Artifacts & Registries**: `.wpkg` can be published as an **OCI artifact** and pushed/pulled via **Docker‑compatible registries** (Docker Hub, GHCR, ECR). We reuse **OCI Distribution Spec** and tooling.
- **Container Isolation**: Wadah agents can **run inside Docker/Podman containers**. We leverage Linux namespaces/cgroups, GPU drivers, and existing image build pipelines.
- **Cluster Orchestration**: Wadah nodes/agents can be **scheduled by Kubernetes/Nomad**. We reuse autoscaling, node pools, secrets, and service meshes.
- **Observability Plumbing**: We emit **OpenTelemetry (OTLP)** spans/metrics so you can reuse **Prometheus/Grafana/Jaeger/Datadog**.

> In short: Wadah rides on the same rails as Docker/K8s. We do **not** replace them; we add a higher‑level, AI‑native abstraction.

## What Wadah Adds (The New, AI‑Native Layer)
- **Agent Package (`.wpkg`)**: A portable unit that bundles prompts, policies, model routing config, tests, and memory schema — *semantic packaging*, not just binaries.
- **Deterministic Reasoning & Replay**: **OAT** traces + lockfiles (seeds, params, digests) to reproduce and audit agent runs.
- **Policy/Capability Model (ToolCaps)**: Fine‑grained, AI‑level permissions (tools, domains, FS paths, budgets) beyond OS isolation.
- **Cognitive Memory Primitives**: Built‑in support for vector/hybrid stores, RAG indexes, and knowledge graphs per agent.
- **Multi‑Agent Graph Orchestration**: First‑class execution graphs, retries, eval hooks, and safety rails tailored to LLM/agent workflows.

## Deployment Patterns
1) **Wadah inside Docker (single host)**
```
[ Docker Container ] → runs `wadahd` + agents
   • Mount /agents and /data volumes
   • GPU passthrough when needed
```
2) **Wadah on Kubernetes (cluster)**
```
[ K8s Deployment ] → `wadahd` Pods
[ K8s Service ]    → exposes gRPC/Web UI
[ PVC/CSI ]        → vector stores / caches
[ Operators ]      → (future) Wadah CRDs for .wpkg
```
3) **Hybrid (edge + cloud)**
```
Edge node: Wadah for low‑latency tools + cache
Cloud: vLLM/TGI, heavy vector DB, centralized tracing
```

## Data & Security Flow (Simplified)
```
User/API → Wadah CLI/SDK → Wadahd → (Policies enforced)
  → Model Adapter(s) → Memory/RAG → Tools (WASM/Proc)
  → OAT Trace Export (OTLP/JSONL) → Observability Stack
```

## FAQ: “Why not just Docker?”
- **Docker** standardizes *process* packaging and isolation; it knows nothing about prompts, seeds, token budgets, tool permissions, or RAG.
- **Wadah** standardizes *agent* packaging and execution semantics — the *reasoning* layer. It complements Docker by adding reproducibility, observability, and policy at the AI level.

## Quick Glossary
- **`.wpkg`**: Wadah Package (OCI‑compatible artifact) for agents.
- **OAT**: OpenAgentTrace — structured traces for prompts, tokens, costs, timings.
- **ToolCaps**: Capability/policy manifest (domains, FS paths, budgets, tool actions).
- **Wadahd**: Core runtime daemon/engine.

---
**Bottom line:** Wadah composes with Docker/K8s — reusing their proven plumbing — while adding the missing AI‑native layer for packaging, running, and governing *intelligence* itself.

