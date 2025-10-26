use crate::oat::{OATEvent, OATTrace};
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct TraceReplayer {
    events: Vec<OATEvent>,
    current_index: usize,
}

impl TraceReplayer {
    pub fn from_jsonl_file(path: &Path) -> crate::Result<Self> {
        let file = std::fs::File::open(path)?;
        let reader = BufReader::new(file);

        let mut events = Vec::new();
        for line in reader.lines() {
            let line = line?;
            if !line.trim().is_empty() {
                let event = OATEvent::from_jsonl(&line)?;
                events.push(event);
            }
        }

        Ok(Self {
            events,
            current_index: 0,
        })
    }

    pub fn from_trace(trace: &OATTrace) -> Self {
        let mut events = Vec::new();
        for span in &trace.spans {
            events.extend(span.events.clone());
        }

        // Sort by timestamp
        events.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

        Self {
            events,
            current_index: 0,
        }
    }

    pub fn next_event(&mut self) -> Option<&OATEvent> {
        if self.current_index < self.events.len() {
            let event = &self.events[self.current_index];
            self.current_index += 1;
            Some(event)
        } else {
            None
        }
    }

    pub fn peek_event(&self) -> Option<&OATEvent> {
        if self.current_index < self.events.len() {
            Some(&self.events[self.current_index])
        } else {
            None
        }
    }

    pub fn reset(&mut self) {
        self.current_index = 0;
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn verify_determinism(&self, other: &TraceReplayer) -> bool {
        if self.events.len() != other.events.len() {
            return false;
        }

        for (a, b) in self.events.iter().zip(other.events.iter()) {
            // Compare event types and data, ignoring IDs and timestamps
            if std::mem::discriminant(&a.event_type) != std::mem::discriminant(&b.event_type) {
                return false;
            }
            if a.data != b.data {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::oat::{EventType, OATTrace};

    #[test]
    fn test_replayer() {
        let mut trace = OATTrace::new("test".to_string(), "0.1.0".to_string(), None);
        let event1 = OATEvent::new(
            trace.trace_id.clone(),
            "span1".to_string(),
            EventType::AgentStart,
        );
        let event2 = OATEvent::new(
            trace.trace_id.clone(),
            "span1".to_string(),
            EventType::AgentEnd,
        );

        let mut span = crate::oat::OATSpan::new(
            trace.trace_id.clone(),
            "main".to_string(),
            crate::oat::SpanKind::Agent,
        );
        span.add_event(event1);
        span.add_event(event2);
        trace.add_span(span);

        let mut replayer = TraceReplayer::from_trace(&trace);
        assert_eq!(replayer.event_count(), 2);
        assert!(replayer.next_event().is_some());
        assert!(replayer.next_event().is_some());
        assert!(replayer.next_event().is_none());
    }
}
