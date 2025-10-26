use super::{ModelAdapter, ModelRequest, ModelResponse};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct OpenAIAdapter {
    client: Client,
    endpoint: String,
    model_id: String,
    api_key: Option<String>,
}

#[derive(Debug, Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seed: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
    usage: Usage,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Usage {
    total_tokens: u32,
}

impl OpenAIAdapter {
    pub fn new(endpoint: Option<String>, model_id: String) -> crate::Result<Self> {
        let endpoint = endpoint.unwrap_or_else(|| "https://api.openai.com/v1".to_string());
        let api_key = std::env::var("OPENAI_API_KEY").ok();

        Ok(Self {
            client: Client::new(),
            endpoint,
            model_id,
            api_key,
        })
    }
}

#[async_trait]
impl ModelAdapter for OpenAIAdapter {
    async fn generate(&self, request: ModelRequest) -> crate::Result<ModelResponse> {
        let url = format!("{}/chat/completions", self.endpoint);

        let openai_request = OpenAIRequest {
            model: self.model_id.clone(),
            messages: vec![Message {
                role: "user".to_string(),
                content: request.prompt,
            }],
            temperature: request.temperature,
            top_p: request.top_p,
            max_tokens: request.max_tokens,
            seed: request.seed,
            stop: request.stop,
        };

        let mut req = self.client.post(&url).json(&openai_request);

        if let Some(ref api_key) = self.api_key {
            req = req.bearer_auth(api_key);
        }

        let response = req.send().await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(crate::RuntimeError::AdapterError(format!(
                "OpenAI API error: {}",
                error_text
            )));
        }

        let openai_response: OpenAIResponse = response.json().await?;

        let choice = openai_response.choices.first().ok_or_else(|| {
            crate::RuntimeError::AdapterError("No choices in OpenAI response".to_string())
        })?;

        Ok(ModelResponse {
            text: choice.message.content.clone(),
            tokens_used: openai_response.usage.total_tokens,
            finish_reason: choice.finish_reason.clone(),
            model: self.model_id.clone(),
        })
    }

    fn model_id(&self) -> &str {
        &self.model_id
    }

    fn provider(&self) -> &str {
        "openai"
    }
}
