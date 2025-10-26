pub mod ollama;
pub mod openai;
pub mod tgi;

pub use ollama::OllamaAdapter;
pub use openai::OpenAIAdapter;
pub use tgi::TGIAdapter;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelRequest {
    pub prompt: String,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub max_tokens: Option<u32>,
    pub seed: Option<u64>,
    pub stop: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelResponse {
    pub text: String,
    pub tokens_used: u32,
    pub finish_reason: Option<String>,
    pub model: String,
}

#[async_trait]
pub trait ModelAdapter: Send + Sync {
    async fn generate(&self, request: ModelRequest) -> crate::Result<ModelResponse>;
    fn model_id(&self) -> &str;
    fn provider(&self) -> &str;
}

pub fn create_adapter(
    provider: &str,
    endpoint: Option<String>,
    model_id: String,
) -> crate::Result<Box<dyn ModelAdapter>> {
    match provider {
        "openai" => Ok(Box::new(OpenAIAdapter::new(endpoint, model_id)?)),
        "ollama" => Ok(Box::new(OllamaAdapter::new(endpoint, model_id)?)),
        "tgi" | "vllm" => Ok(Box::new(TGIAdapter::new(endpoint, model_id)?)),
        _ => Err(crate::RuntimeError::AdapterError(format!(
            "Unsupported provider: {}",
            provider
        ))),
    }
}
