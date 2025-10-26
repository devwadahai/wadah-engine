use super::{ModelAdapter, ModelRequest, ModelResponse};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct TGIAdapter {
    client: Client,
    endpoint: String,
    model_id: String,
}

#[derive(Debug, Serialize)]
struct TGIRequest {
    inputs: String,
    parameters: TGIParameters,
}

#[derive(Debug, Serialize)]
struct TGIParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_new_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seed: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct TGIResponse {
    generated_text: String,
    details: Option<TGIDetails>,
}

#[derive(Debug, Deserialize)]
struct TGIDetails {
    finish_reason: Option<String>,
    generated_tokens: u32,
}

impl TGIAdapter {
    pub fn new(endpoint: Option<String>, model_id: String) -> crate::Result<Self> {
        let endpoint = endpoint.ok_or_else(|| crate::RuntimeError::AdapterError(
            "TGI/vLLM adapter requires an endpoint".to_string()
        ))?;

        Ok(Self {
            client: Client::new(),
            endpoint,
            model_id,
        })
    }
}

#[async_trait]
impl ModelAdapter for TGIAdapter {
    async fn generate(&self, request: ModelRequest) -> crate::Result<ModelResponse> {
        let url = format!("{}/generate", self.endpoint);

        let tgi_request = TGIRequest {
            inputs: request.prompt.clone(),
            parameters: TGIParameters {
                temperature: request.temperature,
                top_p: request.top_p,
                max_new_tokens: request.max_tokens,
                seed: request.seed,
                stop: request.stop,
            },
        };

        let response = self.client
            .post(&url)
            .json(&tgi_request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(crate::RuntimeError::AdapterError(
                format!("TGI API error: {}", error_text)
            ));
        }

        let tgi_response: TGIResponse = response.json().await?;

        let tokens_used = tgi_response.details
            .as_ref()
            .map(|d| d.generated_tokens)
            .unwrap_or(0);

        let finish_reason = tgi_response.details
            .as_ref()
            .and_then(|d| d.finish_reason.clone());

        Ok(ModelResponse {
            text: tgi_response.generated_text,
            tokens_used,
            finish_reason,
            model: self.model_id.clone(),
        })
    }

    fn model_id(&self) -> &str {
        &self.model_id
    }

    fn provider(&self) -> &str {
        "tgi"
    }
}

