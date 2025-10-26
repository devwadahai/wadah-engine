# OpenAgentTrace (OAT) Specification

## Overview

OpenAgentTrace (OAT) is Wadah's standard format for capturing, storing, and replaying agent execution traces. It provides deterministic observability for AI reasoning, enabling audit, debugging, and reproducibility.

## Goals

1. **Observability**: Complete visibility into agent decision-making
2. **Determinism**: Reproducible execution given the same inputs
3. **Audit**: Compliance-grade trace records
4. **Debugging**: Step-through agent reasoning
5. **Interoperability**: Standard format for agent traces

## Format

OAT traces are stored as:
- **JSONL** (JSON Lines) for streaming and incremental writes
- **JSON** for complete trace archives

## Schema

### Trace Structure

A complete trace consists of:

```json
{
  "trace_id": "uuid",
  "agent_name": "string",
  "agent_version": "string",
  "start_time": "ISO 8601",
  "end_time": "ISO 8601",
  "seed": 1337,
  "spans": [...],
  "metadata": {...}
}
```

### Spans

Spans represent logical units of work:

```json
{
  "span_id": "uuid",
  "parent_span_id": "uuid",
  "trace_id": "uuid",
  "name": "string",
  "kind": "agent|model|tool|memory|policy",
  "start_time": "ISO 8601",
  "end_time": "ISO 8601",
  "attributes": {...},
  "events": [...]
}
```

#### Span Kinds

- `agent`: Top-level agent execution
- `model`: LLM API calls
- `tool`: Tool invocations
- `memory`: Memory operations (RAG, KV)
- `policy`: Policy checks (ToolCaps)

### Events

Events are timestamped occurrences within spans:

```json
{
  "id": "uuid",
  "timestamp": "ISO 8601",
  "trace_id": "uuid",
  "span_id": "uuid",
  "parent_span_id": "uuid",
  "event_type": {...},
  "data": {...},
  "metadata": {...}
}
```

## Event Types

### Agent Events

#### AgentStart

```json
{
  "event_type": "agent_start",
  "data": {
    "agent_name": "my-agent",
    "version": "0.1.0",
    "config": {...}
  }
}
```

#### AgentEnd

```json
{
  "event_type": "agent_end",
  "data": {
    "status": "success|error",
    "duration_ms": 1234
  }
}
```

### Model Events

#### ModelRequest

```json
{
  "event_type": {
    "type": "model_request",
    "model": "gpt-4o-mini",
    "prompt": "User input text",
    "params": {
      "temperature": 0.2,
      "max_tokens": 2048,
      "seed": 1337
    }
  }
}
```

#### ModelResponse

```json
{
  "event_type": {
    "type": "model_response",
    "model": "gpt-4o-mini",
    "response": "Model output text",
    "tokens": 150,
    "cost": 0.0023
  }
}
```

### Tool Events

#### ToolCall

```json
{
  "event_type": {
    "type": "tool_call",
    "tool": "github",
    "action": "create_issue",
    "input": {
      "title": "Bug report",
      "body": "Description"
    }
  }
}
```

#### ToolResult

```json
{
  "event_type": {
    "type": "tool_result",
    "tool": "github",
    "action": "create_issue",
    "output": {
      "issue_number": 123,
      "url": "https://..."
    },
    "duration_ms": 456
  }
}
```

### Memory Events

#### MemoryRead

```json
{
  "event_type": {
    "type": "memory_read",
    "key": "user_preferences",
    "value": {...}
  }
}
```

#### MemoryWrite

```json
{
  "event_type": {
    "type": "memory_write",
    "key": "conversation_history",
    "value": [...]
  }
}
```

### Policy Events

#### PolicyCheck

```json
{
  "event_type": {
    "type": "policy_check",
    "rule": "github::create_issue",
    "allowed": true,
    "reason": null
  }
}
```

### Error Events

#### Error

```json
{
  "event_type": {
    "type": "error",
    "error": "Connection timeout",
    "context": "Calling OpenAI API"
  }
}
```

## JSONL Format

Each line is a complete JSON event:

```jsonl
{"id":"evt-1","timestamp":"2025-01-01T10:00:00Z","trace_id":"trace-123","span_id":"span-1","event_type":"agent_start","data":{...}}
{"id":"evt-2","timestamp":"2025-01-01T10:00:01Z","trace_id":"trace-123","span_id":"span-2","event_type":{"type":"model_request",...},"data":{...}}
{"id":"evt-3","timestamp":"2025-01-01T10:00:02Z","trace_id":"trace-123","span_id":"span-2","event_type":{"type":"model_response",...},"data":{...}}
```

## Deterministic Replay

### Requirements for Determinism

1. **Fixed Seed**: Model generation seed must be set
2. **Lockfile**: All dependencies must be locked (model versions, tool versions)
3. **Idempotent Tools**: Tools must produce same output for same input

### Replay Process

```bash
# Capture trace
wadah run agent.wpkg --trace trace.jsonl

# Replay trace
wadah trace replay trace.jsonl --lock wadah.lock
```

### Verification

```rust
let original = TraceReplayer::from_jsonl_file("trace1.jsonl")?;
let replayed = TraceReplayer::from_jsonl_file("trace2.jsonl")?;

assert!(original.verify_determinism(&replayed));
```

## Usage

### Recording

```rust
use wadah_trace::TraceRecorder;

let mut recorder = TraceRecorder::new(
    "my-agent".to_string(),
    "0.1.0".to_string(),
    Some(1337), // seed
)
.with_output(Path::new("trace.jsonl"))?;

recorder.start_span("main".to_string(), SpanKind::Agent);
recorder.record_event(EventType::AgentStart)?;

// ... execution ...

let trace = recorder.finish();
```

### Replay

```rust
use wadah_trace::TraceReplayer;

let mut replayer = TraceReplayer::from_jsonl_file("trace.jsonl")?;

while let Some(event) = replayer.next_event() {
    println!("Event: {:?}", event.event_type);
}
```

### Analysis

```bash
# Event count
wadah trace stats trace.jsonl

# Cost analysis
wadah trace stats trace.jsonl --show-costs

# Timeline view
wadah trace timeline trace.jsonl
```

## Export Formats

### OpenTelemetry

Export to OTLP for integration with observability platforms:

```bash
wadah trace export trace.jsonl --format otlp --endpoint http://collector:4317
```

### Visualization

Generate visual trace:

```bash
wadah trace visualize trace.jsonl --output trace.html
```

## Privacy & Security

### Sensitive Data

**Redaction**: Sensitive data should be redacted in traces:

```json
{
  "event_type": {
    "type": "model_request",
    "prompt": "[REDACTED: PII]",
    "tokens": 150
  }
}
```

### Configuration

```yaml
policy:
  trace:
    redact_patterns:
      - "api_key"
      - "password"
      - "token"
    pii_detection: true
```

## Sampling

For high-throughput agents, enable sampling:

```yaml
policy:
  trace:
    sample_rate: 0.1  # Trace 10% of requests
    always_sample_errors: true
```

## Compliance

### Audit Requirements

OAT traces provide:
- Complete execution history
- Immutable event log (append-only)
- Cryptographic integrity (optional signatures)
- Retention policies

### Signing

```bash
# Sign trace
wadah trace sign trace.jsonl --key private.pem

# Verify signature
wadah trace verify trace.jsonl --key public.pem
```

## Best Practices

### 1. Always Use Seeds

For reproducibility:

```yaml
runtime:
  model:
    params:
      seed: 1337  # Fixed seed
```

### 2. Lockfiles

Generate and commit lockfiles:

```bash
wadah pack -m wadah.yaml -o agent.wpkg
# Creates wadah.lock automatically
```

### 3. Trace Rotation

For long-running agents:

```bash
wadah run agent.wpkg --trace traces/%Y%m%d_%H%M%S.jsonl
```

### 4. Cost Tracking

Include cost metadata:

```json
{
  "event_type": {
    "type": "model_response",
    "cost": 0.0023
  },
  "metadata": {
    "cost_model": "gpt-4o-mini-input-$0.015/1M"
  }
}
```

## Reference Implementation

- Recorder: `crates/trace/src/recorder.rs`
- Replayer: `crates/trace/src/replay.rs`
- Schema: `crates/trace/src/oat.rs`

## Future Extensions

- **Diff Traces**: Compare two trace files
- **Merge Traces**: Combine traces from distributed agents
- **Trace Database**: Store and query traces at scale
- **ML on Traces**: Learn from agent behavior patterns

## See Also

- [WadahSpec](WadahSpec-v0.1.md) - Agent configuration
- [ToolCaps](ToolCaps.md) - Policy system
- [Quickstart](Quickstart.md) - Getting started

