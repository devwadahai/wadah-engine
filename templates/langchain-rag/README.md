# LangChain + TGI + Qdrant RAG Agent

A complete RAG (Retrieval-Augmented Generation) agent built with LangChain, using TGI for model inference and Qdrant for vector search.

## Features

- **LangChain** framework for agent orchestration
- **TGI** (Text Generation Inference) for fast model serving
- **Qdrant** vector database for semantic search
- **Deterministic execution** with seed parameter
- **OAT tracing** for observability

## Architecture

```
User Query → LangChain Agent
    ↓
Qdrant Vector Search → Retrieve Context
    ↓
TGI (Mistral-7B) → Generate Answer
    ↓
Response + Trace
```

## Quick Start

### 1. Start Dependencies

```bash
# Start Qdrant
docker run -d -p 6333:6333 qdrant/qdrant:latest

# Start TGI
docker run -d -p 8080:8080 \
  ghcr.io/huggingface/text-generation-inference:latest \
  --model-id mistralai/Mistral-7B-Instruct-v0.2
```

### 2. Install Dependencies

```bash
python -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt
```

### 3. Run with Wadah

```bash
# Build package
wadah pack -m wadah.yaml -o build/langchain-rag.wpkg

# Run agent
wadah run build/langchain-rag.wpkg \
  --prompt "What is retrieval-augmented generation?"
```

## Usage

### Single Query

```bash
wadah run wadah.yaml --prompt "Your question here"
```

### Interactive Mode

```bash
wadah run wadah.yaml --interactive
```

### With Tracing

```bash
wadah run wadah.yaml \
  --prompt "What is RAG?" \
  --trace traces/session.jsonl
```

## Configuration

### Change Model Backend

```yaml
# Use OpenAI instead
runtime:
  model:
    provider: openai
    modelId: gpt-4o-mini

# Use Ollama (local)
runtime:
  model:
    provider: ollama
    endpoint: http://localhost:11434
    modelId: llama3
```

### Change Vector Database

```yaml
# Use LanceDB
runtime:
  memory:
    type: vector
    backend: lancedb
    settings:
      path: ./data/lancedb

# Use pgvector
runtime:
  memory:
    type: vector
    backend: pgvector
    settings:
      connection_string: postgresql://...
```

## Docker Compose

```yaml
version: '3.8'

services:
  agent:
    image: wadah/runtime:latest
    command: wadah run /app/agent.wpkg
    environment:
      - QDRANT_URL=http://qdrant:6333
      - OPENAI_BASE=http://tgi:8080/v1
    volumes:
      - ./build/langchain-rag.wpkg:/app/agent.wpkg:ro
    depends_on:
      - tgi
      - qdrant
  
  tgi:
    image: ghcr.io/huggingface/text-generation-inference:latest
    command: --model-id mistralai/Mistral-7B-Instruct-v0.2
    ports:
      - "8080:8080"
  
  qdrant:
    image: qdrant/qdrant:latest
    ports:
      - "6333:6333"
    volumes:
      - qdrant-data:/qdrant/storage

volumes:
  qdrant-data:
```

Run:
```bash
docker-compose up
```

## Customization

### Add Your Documents

```python
# In app/main.py
from langchain.document_loaders import TextLoader
from langchain.text_splitter import RecursiveCharacterTextSplitter

# Load documents
loader = TextLoader("your_docs.txt")
documents = loader.load()

# Split
text_splitter = RecursiveCharacterTextSplitter(
    chunk_size=1000,
    chunk_overlap=200
)
splits = text_splitter.split_documents(documents)

# Index in Qdrant
vector_store = Qdrant.from_documents(
    documents=splits,
    embedding=embeddings,
    url=os.getenv("QDRANT_URL"),
    collection_name="langchain_docs"
)
```

### Add Tools

```python
from langchain.agents import create_openai_functions_agent
from langchain.tools import Tool

# Define tools
tools = [
    Tool(
        name="search",
        func=vector_store.similarity_search,
        description="Search documentation"
    )
]

# Create agent with tools
agent = create_openai_functions_agent(
    llm=llm,
    tools=tools,
    prompt=prompt
)
```

## Observability

### View Traces

```bash
# Stats
wadah trace stats traces/session.jsonl

# Replay
wadah trace replay traces/session.jsonl --lock wadah.lock
```

### Prometheus Metrics

Wadah exposes:
- `wadah_tokens_total` - Total tokens used
- `wadah_cost_usd` - Total cost
- `wadah_retrieval_latency_seconds` - Vector search latency

## Production Deployment

See [Interoperability Guide](../../docs/Interoperability.md) for:
- Kubernetes deployment
- OTLP integration (Jaeger/Grafana)
- Multi-agent patterns
- Security best practices

## Learn More

- [LangChain Documentation](https://python.langchain.com/)
- [TGI Documentation](https://huggingface.co/docs/text-generation-inference)
- [Qdrant Documentation](https://qdrant.tech/documentation/)
- [Wadah Interoperability](../../docs/Interoperability.md)

## License

Apache 2.0

