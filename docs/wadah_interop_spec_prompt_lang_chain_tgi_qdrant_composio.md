# Wadah Interop Spec Prompt — LangChain + TGI + Qdrant + Composio

> **Goal:** Package and run a LangChain-based agent inside **Wadah** using **TGI** for model serving, **Qdrant** for vector memory, and **Composio (GitHub)** as a governed tool — with deterministic replay, OCI distribution, and OTLP tracing.

---

## 0) Repository Skeleton
```
repo/
 ├─ wadah.yaml                 # WadahSpec v0.1 manifest
 ├─ ToolCaps.json              # Policy/capability spec
 ├─ wadah.lock                 # Auto-generated lockfile (on pack)
 ├─ prompts/
 │   └─ system.md
 ├─ app/
 │   ├─ main.py                # LangChain entrypoint `agent`
 │   └─ requirements.txt
 ├─ tools/
 │   └─ composio_github.yaml   # Tool manifest (scopes, base URL)
 ├─ tests/
 │   └─ conformance.yaml       # Golden trace assertions
 ├─ data/
 │   └─ (qdrant disk index)
 └─ Makefile
```

---

## 1) **WadahSpec v0.1** (`wadah.yaml`)
```yaml
apiVersion: wadah.ai/v0.1
kind: Agent
metadata:
  name: repo-assistant
  version: 0.1.0
  description: "LangChain agent that summarizes repos and opens issues via Composio (GitHub)."
  authors: ["Your Name <you@example.com>"]
  license: Apache-2.0

runtime:
  entrypoint: python -m app.main:agent
  env:
    - name: OPENAI_API_KEY           # or leave unset if using TGI only
    - name: COMPOSIO_API_KEY
    - name: QDRANT_URL
  model:
    provider: tgi
    endpoint: http://tgi:8080
    modelId: mistral-7b-instruct
    params:
      temperature: 0.2
      top_p: 0.95
      max_tokens: 1024
      seed: 42
  memory:
    type: vector
    backend: qdrant
    settings:
      url: ${QDRANT_URL:-http://localhost:6333}
      collection: repo_assistant
      persist: true
  tools:
    - id: composio_github
      type: http
      manifest: tools/composio_github.yaml

policy:
  toolcaps: ToolCaps.json
  budgets:
    tokens_per_minute: 200000
    usd_per_day: 25
  network:
    allow_domains: ["api.github.com", "raw.githubusercontent.com"]

artifacts:
  include:
    - prompts/**
    - app/**
    - tools/**
    - tests/**
    - requirements.txt

lock:
  model:
    id: mistral-7b-instruct
    provider: tgi
    endpoint: http://tgi:8080
    digest: sha256:<filled by pack>
  tools:
    - id: composio_github
      version: 1.0.0
      digest: sha256:<filled by pack>
  memory:
    backend: qdrant
```

---

## 2) **ToolCaps** Policy (`ToolCaps.json`)
```json
{
  "version": "0.1",
  "allow": [
    {"tool": "http", "actions": ["GET"], "domains": ["api.github.com", "raw.githubusercontent.com"], "rate": {"rpm": 60}},
    {"tool": "composio_github", "actions": ["read_issues", "create_issue"], "limits": {"per_min": 5}},
    {"tool": "fs", "actions": ["read"], "paths": ["/workspace/repo", "/workspace/prompts"]}
  ],
  "deny": [
    {"tool": "shell", "actions": ["exec"]},
    {"tool": "wallet", "actions": ["transfer"]}
  ],
  "budgets": {"usd_per_day": 25, "tokens_per_minute": 200000},
  "redaction": {"secrets": ["OPENAI_API_KEY", "COMPOSIO_API_KEY"]}
}
```

---

## 3) **Tool Manifest** (Composio → `tools/composio_github.yaml`)
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

---

## 4) **LangChain Entrypoint** (`app/main.py`)
```python
import os
from langchain_openai import ChatOpenAI
from langchain_core.prompts import ChatPromptTemplate
from langchain_core.runnables import RunnableLambda

# Model adapter: if you want TGI (OpenAI-compatible), point OPENAI_BASE to TGI
# export OPENAI_BASE=http://tgi:8080/v1  and use a model name exposed by TGI

SYSTEM_PROMPT = open("prompts/system.md").read()

def summarize_repo(context):
    # `context` should include repo text blobs injected by the runner or fetched via tool
    return f"Summary (tokens omitted): {context[:500]}..."

llm = ChatOpenAI(model=os.getenv("OPENAI_MODEL", "mistral-7b-instruct"))
prompt = ChatPromptTemplate.from_messages([
    ("system", SYSTEM_PROMPT),
    ("human", "Summarize this repository and suggest one actionable issue: {repo_text}")
])

chain = prompt | llm | RunnableLambda(lambda x: x.content)

def agent(repo_text: str = ""):
    result = chain.invoke({"repo_text": repo_text})
    return {"summary": result}

if __name__ == "__main__":
    print(agent(repo_text="..."))
```

---

## 5) **Prompts** (`prompts/system.md`)
```
You are RepoAssistant. Provide concise summaries and propose one actionable GitHub issue title and description.
Constrain suggestions to read-only analysis unless policy grants write permissions.
```

---

## 6) **Conformance Test** (`tests/conformance.yaml`)
```yaml
name: repo-assistant-conformance
checks:
  - type: run
    seed: 42
    input:
      repo_text: "This is a demo repo with two modules..."
    expect:
      contains: ["Summary", "actionable", "issue"]
  - type: policy
    deny_actions: ["shell.exec", "wallet.transfer"]
  - type: budget
    max_usd: 1.00
```

---

## 7) **Makefile** (developer UX)
```makefile
init:
	python -m venv .venv && . .venv/bin/activate && pip install -r app/requirements.txt

pack:
	wadah pack -m wadah.yaml -o build/repo-assistant.wpkg

run:
	wadah run build/repo-assistant.wpkg --trace traces/run1.jsonl

replay:
	wadah trace replay traces/run1.jsonl --lock wadah.lock

push:
	wadah push ghcr.io/yourorg/repo-assistant:0.1.0
```

---

## 8) **Adapter Notes**
- **Model backends:** Swap `runtime.model.provider` between `tgi|vllm|ollama|openai` without code changes. Ensure `OPENAI_BASE` is set if using OpenAI-compatible routes.
- **Memory:** For Qdrant local, mount a volume and set `QDRANT_URL`. For LanceDB, replace backend and path in `wadah.yaml`.
- **Tracing:** Wadah emits **OAT** → configure OTLP endpoint (Jaeger/Tempo/Datadog) via env: `OTEL_EXPORTER_OTLP_ENDPOINT`.
- **Security:** ToolCaps enforces least privilege. For stronger isolation, run tools in WASM sandboxes.

---

## 9) **Acceptance Criteria**
- Pack/unpack `.wpkg` with digests and auto-generated `wadah.lock`.
- Run agent against TGI with deterministic seed; produce trace.
- Enforce ToolCaps (deny shell.exec, wallet.transfer).
- Qdrant receives and retrieves embeddings for repo content.
- Push/pull `.wpkg` as OCI artifact to GHCR.

---

## 10) **Next Steps (stretch)**
- Add **GitHub create_issue** call path gated by ToolCaps.
- Provide **Helm chart** for running `wadahd` + TGI + Qdrant on Kubernetes.
- Add **policy linter** that diff-checks ToolCaps against traces and suggests tighter scopes.

