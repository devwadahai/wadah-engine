use crate::{error::*, types::*, facilitator::*};
use std::collections::HashMap;

/// X402 Payment Middleware
pub struct X402Middleware {
    facilitator: FacilitatorClient,
    supported_schemes: Vec<PaymentScheme>,
    supported_networks: Vec<String>,
    pay_to_address: String,
}

impl X402Middleware {
    /// Create a new X402 middleware
    pub fn new(
        facilitator_url: impl Into<String>,
        pay_to_address: impl Into<String>,
    ) -> Self {
        Self {
            facilitator: FacilitatorClient::new(facilitator_url),
            supported_schemes: vec![PaymentScheme::Exact],
            supported_networks: vec!["base".to_string(), "ethereum".to_string()],
            pay_to_address: pay_to_address.into(),
        }
    }
    
    /// Create payment requirements for an agent
    pub fn create_payment_requirements(
        &self,
        resource: impl Into<String>,
        description: impl Into<String>,
        amount: impl Into<String>,
        asset: impl Into<String>,
        network: impl Into<String>,
    ) -> PaymentRequirements {
        PaymentRequirements {
            scheme: PaymentScheme::Exact,
            network: network.into(),
            max_amount_required: amount.into(),
            resource: resource.into(),
            description: description.into(),
            mime_type: "application/json".to_string(),
            output_schema: None,
            pay_to: self.pay_to_address.clone(),
            max_timeout_seconds: 300,
            asset: asset.into(),
            extra: None,
        }
    }
    
    /// Create a 402 Payment Required response
    pub fn create_payment_required_response(
        &self,
        requirements: Vec<PaymentRequirements>,
    ) -> PaymentRequiredResponse {
        PaymentRequiredResponse {
            x402_version: X402_VERSION,
            accepts: requirements,
            error: None,
        }
    }
    
    /// Verify a payment from X-PAYMENT header
    pub async fn verify_payment(
        &self,
        payment_header: &str,
        requirements: &PaymentRequirements,
    ) -> Result<VerificationResponse> {
        // Decode and validate payment payload
        let payload = PaymentPayload::from_header_value(payment_header)
            .map_err(|e| PaymentError::InvalidHeader(e.to_string()))?;
        
        // Verify version
        if payload.x402_version != X402_VERSION {
            return Err(PaymentError::VerificationFailed(format!(
                "Unsupported x402 version: {}",
                payload.x402_version
            )));
        }
        
        // Verify scheme
        if payload.scheme != requirements.scheme {
            return Err(PaymentError::UnsupportedScheme(format!(
                "Scheme mismatch: expected {:?}, got {:?}",
                requirements.scheme, payload.scheme
            )));
        }
        
        // Verify network
        if payload.network != requirements.network {
            return Err(PaymentError::UnsupportedNetwork(format!(
                "Network mismatch: expected {}, got {}",
                requirements.network, payload.network
            )));
        }
        
        // Verify with facilitator
        self.facilitator.verify(payment_header, requirements).await
    }
    
    /// Settle a verified payment
    pub async fn settle_payment(
        &self,
        payment_header: &str,
        requirements: &PaymentRequirements,
    ) -> Result<SettlementResponse> {
        self.facilitator.settle(payment_header, requirements).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_middleware_creation() {
        let middleware = X402Middleware::new(
            "https://facilitator.example.com",
            "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb",
        );
        
        assert_eq!(middleware.pay_to_address, "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb");
        assert_eq!(middleware.supported_schemes.len(), 1);
        assert_eq!(middleware.supported_networks.len(), 2);
    }
    
    #[test]
    fn test_create_payment_requirements() {
        let middleware = X402Middleware::new(
            "https://facilitator.example.com",
            "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb",
        );
        
        let requirements = middleware.create_payment_requirements(
            "/api/agent/run",
            "Premium AI Agent",
            "10000",
            "0x833589fcd6edb6e08f4c7c32d4f71b54bda02913",
            "base",
        );
        
        assert_eq!(requirements.max_amount_required, "10000");
        assert_eq!(requirements.network, "base");
        assert_eq!(requirements.scheme, PaymentScheme::Exact);
    }
}

