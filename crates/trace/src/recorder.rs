use crate::oat::{OATEvent, OATSpan, OATTrace};
use std::io::Write;
use std::path::Path;
use tokio::sync::mpsc;

pub struct TraceRecorder {
    trace: OATTrace,
    current_span: Option<OATSpan>,
    output: Option<std::fs::File>,
    event_tx: Option<mpsc::UnboundedSender<OATEvent>>,
}

impl TraceRecorder {
    pub fn new(agent_name: String, agent_version: String, seed: Option<u64>) -> Self {
        Self {
            trace: OATTrace::new(agent_name, agent_version, seed),
            current_span: None,
            output: None,
            event_tx: None,
        }
    }

    pub fn with_output(mut self, path: &Path) -> crate::Result<Self> {
        let file = std::fs::File::create(path)?;
        self.output = Some(file);
        Ok(self)
    }

    pub fn with_channel(mut self) -> (Self, mpsc::UnboundedReceiver<OATEvent>) {
        let (tx, rx) = mpsc::unbounded_channel();
        self.event_tx = Some(tx);
        (self, rx)
    }

    pub fn start_span(&mut self, name: String, kind: crate::oat::SpanKind) {
        if let Some(mut span) = self.current_span.take() {
            span.finish();
            self.trace.add_span(span);
        }

        let mut span = OATSpan::new(self.trace.trace_id.clone(), name, kind);
        if let Some(last_span) = self.trace.spans.last() {
            span.parent_span_id = Some(last_span.span_id.clone());
        }
        self.current_span = Some(span);
    }

    pub fn record_event(&mut self, event_type: crate::oat::EventType) -> crate::Result<()> {
        let span_id = self
            .current_span
            .as_ref()
            .map(|s| s.span_id.clone())
            .unwrap_or_else(|| "root".to_string());

        let event = OATEvent::new(self.trace.trace_id.clone(), span_id, event_type);

        // Write to file if configured
        if let Some(ref mut file) = self.output {
            let line = event.to_jsonl()?;
            writeln!(file, "{}", line)?;
            file.flush()?;
        }

        // Send to channel if configured
        if let Some(ref tx) = self.event_tx {
            let _ = tx.send(event.clone());
        }

        // Add to current span
        if let Some(ref mut span) = self.current_span {
            span.add_event(event);
        }

        Ok(())
    }

    pub fn finish(mut self) -> OATTrace {
        if let Some(mut span) = self.current_span.take() {
            span.finish();
            self.trace.add_span(span);
        }
        self.trace.finish();
        self.trace
    }

    pub fn get_trace(&self) -> &OATTrace {
        &self.trace
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::oat::{EventType, SpanKind};

    #[tokio::test]
    async fn test_recorder() {
        let mut recorder =
            TraceRecorder::new("test-agent".to_string(), "0.1.0".to_string(), Some(42));

        recorder.start_span("main".to_string(), SpanKind::Agent);
        recorder.record_event(EventType::AgentStart).unwrap();
        recorder.record_event(EventType::AgentEnd).unwrap();

        let trace = recorder.finish();
        assert_eq!(trace.agent_name, "test-agent");
        assert_eq!(trace.spans.len(), 1);
        assert_eq!(trace.spans[0].events.len(), 2);
    }
}
