# RAG Service Agent

A Retrieval-Augmented Generation (RAG) service for document Q&A using Wadah.

## Features

- **Document Search**: Semantic search over indexed documents
- **Context-Aware Answers**: Uses retrieved context to answer questions
- **Budget Control**: Token and cost limits
- **Read-Only**: Secure, read-only filesystem access

## Quick Start

```bash
# Set your OpenAI API key
export OPENAI_API_KEY="your-key-here"

# Initialize with your documents
# (In production, you'd index your documents first)
mkdir -p data/index

# Run the agent
wadah run wadah.yaml --prompt "What is Wadah?"

# Or package and run
wadah pack -m wadah.yaml -o build/rag-service.wpkg
wadah run build/rag-service.wpkg
```

## Configuration

Edit `wadah.yaml` to:
- Change the model provider (OpenAI, Ollama, etc.)
- Adjust temperature and parameters
- Configure retrieval settings (top_k, threshold)
- Set budget limits

## Example Usage

```bash
# Single query
wadah run wadah.yaml --prompt "Explain the concept of containers"

# Interactive mode
wadah run wadah.yaml --interactive

# With tracing
wadah run wadah.yaml --prompt "Your question" --trace traces/session.jsonl
```

## Use Cases

- Document Q&A systems
- Knowledge base assistants
- Research paper analysis
- Technical documentation helpers
- Customer support bots

## Architecture

```
User Query → RAG Agent → Document Search → Context Retrieval
                ↓
              LLM (with context) → Response
```

## Security

- Read-only filesystem access
- Network restricted to OpenAI API
- Token and cost budgets enforced
- All operations traced for audit

## License

Apache 2.0

