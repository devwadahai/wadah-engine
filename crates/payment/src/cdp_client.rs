use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

/// CDP (Coinbase Developer Platform) API Client
/// 
/// This client communicates with CDP's HTTP APIs directly (no SDK needed).
/// Works perfectly from Rust even though CDP only provides TypeScript/Python SDKs.
pub struct CDPClient {
    client: Client,
    api_key_id: String,
    private_key: String,
    facilitator_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CDPConfig {
    pub id: String,
    #[serde(rename = "privateKey")]
    pub private_key: String,
}

impl CDPClient {
    /// Create a new CDP client from configuration
    pub fn new(config: CDPConfig, facilitator_url: String) -> Self {
        Self {
            client: Client::new(),
            api_key_id: config.id,
            private_key: config.private_key,
            facilitator_url,
        }
    }

    /// Create from environment variables
    pub fn from_env() -> Result<Self> {
        let api_key_id = env::var("CDP_API_KEY_ID")?;
        let private_key = env::var("CDP_PRIVATE_KEY")?;
        let facilitator_url = env::var("X402_FACILITATOR_URL")
            .unwrap_or_else(|_| "https://facilitator.x402.coinbase.com".to_string());

        Ok(Self {
            client: Client::new(),
            api_key_id,
            private_key,
            facilitator_url,
        })
    }

    /// Create from JSON file (like the one from CDP portal)
    pub fn from_json_file(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let config: CDPConfig = serde_json::from_str(&json)?;
        
        let facilitator_url = env::var("X402_FACILITATOR_URL")
            .unwrap_or_else(|_| "https://facilitator.x402.coinbase.com".to_string());
        
        Ok(Self::new(config, facilitator_url))
    }

    /// Get authentication header value
    fn auth_header(&self) -> String {
        // CDP uses the API key ID in the Authorization header
        format!("Bearer {}", self.api_key_id)
    }

    /// Call CDP Facilitator API to verify payment
    pub async fn verify_payment(&self, payment_data: &str) -> Result<bool> {
        let url = format!("{}/verify", self.facilitator_url);
        
        let response = self.client
            .post(&url)
            .header("Authorization", self.auth_header())
            .header("Content-Type", "application/json")
            .body(payment_data.to_string())
            .send()
            .await?;

        if response.status().is_success() {
            let result: serde_json::Value = response.json().await?;
            Ok(result["valid"].as_bool().unwrap_or(false))
        } else {
            Ok(false)
        }
    }

    /// Track payment event (analytics)
    pub async fn track_payment_event(&self, event: PaymentEvent) -> Result<()> {
        // CDP Analytics API endpoint
        let url = "https://api.cdp.coinbase.com/v1/analytics/events";
        
        let _response = self.client
            .post(url)
            .header("Authorization", self.auth_header())
            .header("Content-Type", "application/json")
            .json(&event)
            .send()
            .await?;

        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct PaymentEvent {
    pub event_type: String,
    pub amount: String,
    pub currency: String,
    pub agent_id: String,
    pub user_address: String,
    pub timestamp: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cdp_config_parsing() {
        let json = r#"{
            "id": "9d48d559-ab29-47bd-8599-69bda1dc0e28",
            "privateKey": "HiZm8JMjxyB5GslzbNQ7wJOzJWGTIJ8tBt0I40OzotExevvWUjbyZ3tXPaCtsDoipruExVqZGZmaYN6sk+RN6Q=="
        }"#;
        
        let config: CDPConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.id, "9d48d559-ab29-47bd-8599-69bda1dc0e28");
        assert!(!config.private_key.is_empty());
    }
}

