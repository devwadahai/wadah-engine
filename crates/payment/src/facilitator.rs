use crate::{error::*, types::*};
use async_trait::async_trait;
use reqwest::Client;

/// Facilitator client for x402 payment verification and settlement
pub struct FacilitatorClient {
    base_url: String,
    client: Client,
}

impl FacilitatorClient {
    /// Create a new facilitator client
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            client: Client::new(),
        }
    }
    
    /// Verify a payment with the facilitator
    pub async fn verify(
        &self,
        payment_header: &str,
        requirements: &PaymentRequirements,
    ) -> Result<VerificationResponse> {
        let request = VerificationRequest {
            x402_version: X402_VERSION,
            payment_header: payment_header.to_string(),
            payment_requirements: requirements.clone(),
        };
        
        let url = format!("{}/verify", self.base_url);
        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(PaymentError::FacilitatorError(format!(
                "Verification request failed with status: {}",
                response.status()
            )));
        }
        
        let verification: VerificationResponse = response.json().await?;
        Ok(verification)
    }
    
    /// Settle a payment with the facilitator
    pub async fn settle(
        &self,
        payment_header: &str,
        requirements: &PaymentRequirements,
    ) -> Result<SettlementResponse> {
        let request = SettlementRequest {
            x402_version: X402_VERSION,
            payment_header: payment_header.to_string(),
            payment_requirements: requirements.clone(),
        };
        
        let url = format!("{}/settle", self.base_url);
        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(PaymentError::FacilitatorError(format!(
                "Settlement request failed with status: {}",
                response.status()
            )));
        }
        
        let settlement: SettlementResponse = response.json().await?;
        Ok(settlement)
    }
    
    /// Get supported schemes and networks from the facilitator
    pub async fn get_supported(&self) -> Result<SupportedSchemesResponse> {
        let url = format!("{}/supported", self.base_url);
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(PaymentError::FacilitatorError(format!(
                "Get supported request failed with status: {}",
                response.status()
            )));
        }
        
        let supported: SupportedSchemesResponse = response.json().await?;
        Ok(supported)
    }
}

/// Payment verifier trait
#[async_trait]
pub trait PaymentVerifier: Send + Sync {
    /// Verify a payment payload
    async fn verify(
        &self,
        payment_header: &str,
        requirements: &PaymentRequirements,
    ) -> Result<VerificationResponse>;
}

/// Payment settler trait
#[async_trait]
pub trait PaymentSettler: Send + Sync {
    /// Settle a verified payment
    async fn settle(
        &self,
        payment_header: &str,
        requirements: &PaymentRequirements,
    ) -> Result<SettlementResponse>;
}

#[async_trait]
impl PaymentVerifier for FacilitatorClient {
    async fn verify(
        &self,
        payment_header: &str,
        requirements: &PaymentRequirements,
    ) -> Result<VerificationResponse> {
        self.verify(payment_header, requirements).await
    }
}

#[async_trait]
impl PaymentSettler for FacilitatorClient {
    async fn settle(
        &self,
        payment_header: &str,
        requirements: &PaymentRequirements,
    ) -> Result<SettlementResponse> {
        self.settle(payment_header, requirements).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_facilitator_client_creation() {
        let client = FacilitatorClient::new("https://facilitator.example.com");
        assert_eq!(client.base_url, "https://facilitator.example.com");
    }
}

