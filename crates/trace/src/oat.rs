use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// OpenAgentTrace (OAT) Event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OATEvent {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub trace_id: String,
    pub span_id: String,
    pub parent_span_id: Option<String>,
    pub event_type: EventType,
    pub data: serde_json::Value,
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EventType {
    AgentStart,
    AgentEnd,
    ModelRequest {
        model: String,
        prompt: String,
        params: serde_json::Value,
    },
    ModelResponse {
        model: String,
        response: String,
        tokens: u32,
        cost: Option<f64>,
    },
    ToolCall {
        tool: String,
        action: String,
        input: serde_json::Value,
    },
    ToolResult {
        tool: String,
        action: String,
        output: serde_json::Value,
        duration_ms: u64,
    },
    MemoryRead {
        key: String,
        value: Option<serde_json::Value>,
    },
    MemoryWrite {
        key: String,
        value: serde_json::Value,
    },
    PolicyCheck {
        rule: String,
        allowed: bool,
        reason: Option<String>,
    },
    Error {
        error: String,
        context: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OATSpan {
    pub span_id: String,
    pub parent_span_id: Option<String>,
    pub trace_id: String,
    pub name: String,
    pub kind: SpanKind,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    #[serde(default)]
    pub attributes: HashMap<String, String>,
    #[serde(default)]
    pub events: Vec<OATEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpanKind {
    Agent,
    Model,
    Tool,
    Memory,
    Policy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OATTrace {
    pub trace_id: String,
    pub agent_name: String,
    pub agent_version: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub seed: Option<u64>,
    #[serde(default)]
    pub spans: Vec<OATSpan>,
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

impl OATEvent {
    pub fn new(trace_id: String, span_id: String, event_type: EventType) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            trace_id,
            span_id,
            parent_span_id: None,
            event_type,
            data: serde_json::json!({}),
            metadata: HashMap::new(),
        }
    }

    pub fn to_jsonl(&self) -> crate::Result<String> {
        Ok(serde_json::to_string(self)?)
    }

    pub fn from_jsonl(line: &str) -> crate::Result<Self> {
        Ok(serde_json::from_str(line)?)
    }
}

impl OATSpan {
    pub fn new(trace_id: String, name: String, kind: SpanKind) -> Self {
        Self {
            span_id: Uuid::new_v4().to_string(),
            parent_span_id: None,
            trace_id,
            name,
            kind,
            start_time: Utc::now(),
            end_time: None,
            attributes: HashMap::new(),
            events: Vec::new(),
        }
    }

    pub fn finish(&mut self) {
        self.end_time = Some(Utc::now());
    }

    pub fn add_event(&mut self, event: OATEvent) {
        self.events.push(event);
    }
}

impl OATTrace {
    pub fn new(agent_name: String, agent_version: String, seed: Option<u64>) -> Self {
        Self {
            trace_id: Uuid::new_v4().to_string(),
            agent_name,
            agent_version,
            start_time: Utc::now(),
            end_time: None,
            seed,
            spans: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn finish(&mut self) {
        self.end_time = Some(Utc::now());
    }

    pub fn add_span(&mut self, span: OATSpan) {
        self.spans.push(span);
    }

    pub fn to_json(&self) -> crate::Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    pub fn from_json(content: &str) -> crate::Result<Self> {
        Ok(serde_json::from_str(content)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_event() {
        let event = OATEvent::new(
            "trace-123".to_string(),
            "span-456".to_string(),
            EventType::AgentStart,
        );

        assert_eq!(event.trace_id, "trace-123");
        assert_eq!(event.span_id, "span-456");
    }

    #[test]
    fn test_create_trace() {
        let mut trace = OATTrace::new(
            "test-agent".to_string(),
            "0.1.0".to_string(),
            Some(1337),
        );

        let mut span = OATSpan::new(
            trace.trace_id.clone(),
            "main".to_string(),
            SpanKind::Agent,
        );
        span.finish();

        trace.add_span(span);
        trace.finish();

        assert_eq!(trace.agent_name, "test-agent");
        assert_eq!(trace.spans.len(), 1);
        assert!(trace.end_time.is_some());
    }
}

