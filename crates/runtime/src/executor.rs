use crate::adapters::{ModelAdapter, ModelRequest};
use crate::policy::PolicyEnforcer;
use crate::budget::BudgetTracker;
use wadah_spec::WadahSpec;
use wadah_trace::{TraceRecorder, EventType, SpanKind};

pub struct AgentExecutor {
    spec: WadahSpec,
    adapter: Box<dyn ModelAdapter>,
    policy_enforcer: PolicyEnforcer,
    budget_tracker: BudgetTracker,
    recorder: Option<TraceRecorder>,
}

impl AgentExecutor {
    pub fn new(
        spec: WadahSpec,
        adapter: Box<dyn ModelAdapter>,
    ) -> crate::Result<Self> {
        let policy_enforcer = PolicyEnforcer::new(None); // Will load from spec if needed
        let budget_tracker = BudgetTracker::new(
            spec.policy.as_ref().and_then(|p| p.budgets.clone())
        );

        Ok(Self {
            spec,
            adapter,
            policy_enforcer,
            budget_tracker,
            recorder: None,
        })
    }

    pub fn with_tracing(mut self, recorder: TraceRecorder) -> Self {
        self.recorder = Some(recorder);
        self
    }

    pub async fn execute(&mut self, prompt: String) -> crate::Result<String> {
        // Start execution span
        if let Some(ref mut recorder) = self.recorder {
            recorder.start_span("execution".to_string(), SpanKind::Agent);
            recorder.record_event(EventType::AgentStart)?;
        }

        // Check budget
        self.budget_tracker.check_duration()?;

        // Build model request
        let request = ModelRequest {
            prompt: prompt.clone(),
            temperature: self.spec.runtime.model.params.temperature,
            top_p: self.spec.runtime.model.params.top_p,
            max_tokens: self.spec.runtime.model.params.max_tokens,
            seed: self.spec.runtime.model.params.seed,
            stop: None,
        };

        // Record model request
        if let Some(ref mut recorder) = self.recorder {
            recorder.start_span("model_request".to_string(), SpanKind::Model);
            recorder.record_event(EventType::ModelRequest {
                model: self.adapter.model_id().to_string(),
                prompt: prompt.clone(),
                params: serde_json::to_value(&request)?,
            })?;
        }

        // Call model
        let response = self.adapter.generate(request).await?;

        // Check token budget
        self.budget_tracker.check_tokens(response.tokens_used as u64)?;

        // Record model response
        if let Some(ref mut recorder) = self.recorder {
            recorder.record_event(EventType::ModelResponse {
                model: response.model.clone(),
                response: response.text.clone(),
                tokens: response.tokens_used,
                cost: None, // TODO: Calculate based on model pricing
            })?;
        }

        // Finish execution
        if let Some(ref mut recorder) = self.recorder {
            recorder.record_event(EventType::AgentEnd)?;
        }

        Ok(response.text)
    }

    pub fn finish(self) -> Option<wadah_trace::OATTrace> {
        self.recorder.map(|r| r.finish())
    }

    pub fn get_budget_stats(&self) -> crate::budget::BudgetStats {
        self.budget_tracker.get_stats()
    }
}

#[cfg(test)]
mod tests {
    // Integration tests would require actual model endpoints
    // These should be tested with mock adapters
}

