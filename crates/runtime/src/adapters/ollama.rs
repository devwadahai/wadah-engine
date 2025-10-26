use super::{ModelAdapter, ModelRequest, ModelResponse};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct OllamaAdapter {
    client: Client,
    endpoint: String,
    model_id: String,
}

#[derive(Debug, Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<OllamaOptions>,
    stream: bool,
}

#[derive(Debug, Serialize)]
struct OllamaOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_predict: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seed: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct OllamaResponse {
    response: String,
    done: bool,
    #[serde(default)]
    eval_count: u32,
    #[serde(default)]
    prompt_eval_count: u32,
}

impl OllamaAdapter {
    pub fn new(endpoint: Option<String>, model_id: String) -> crate::Result<Self> {
        let endpoint = endpoint.unwrap_or_else(|| "http://localhost:11434".to_string());

        Ok(Self {
            client: Client::new(),
            endpoint,
            model_id,
        })
    }
}

#[async_trait]
impl ModelAdapter for OllamaAdapter {
    async fn generate(&self, request: ModelRequest) -> crate::Result<ModelResponse> {
        let url = format!("{}/api/generate", self.endpoint);

        let options = if request.temperature.is_some() 
            || request.top_p.is_some() 
            || request.max_tokens.is_some() 
            || request.seed.is_some() 
        {
            Some(OllamaOptions {
                temperature: request.temperature,
                top_p: request.top_p,
                num_predict: request.max_tokens,
                seed: request.seed,
                stop: request.stop,
            })
        } else {
            None
        };

        let ollama_request = OllamaRequest {
            model: self.model_id.clone(),
            prompt: request.prompt,
            options,
            stream: false,
        };

        let response = self.client
            .post(&url)
            .json(&ollama_request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(crate::RuntimeError::AdapterError(
                format!("Ollama API error: {}", error_text)
            ));
        }

        let ollama_response: OllamaResponse = response.json().await?;

        Ok(ModelResponse {
            text: ollama_response.response,
            tokens_used: ollama_response.eval_count + ollama_response.prompt_eval_count,
            finish_reason: if ollama_response.done { Some("stop".to_string()) } else { None },
            model: self.model_id.clone(),
        })
    }

    fn model_id(&self) -> &str {
        &self.model_id
    }

    fn provider(&self) -> &str {
        "ollama"
    }
}

