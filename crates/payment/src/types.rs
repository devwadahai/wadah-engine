use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use base64::Engine;

/// x402 Protocol Version
pub const X402_VERSION: u32 = 1;

/// Payment scheme types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PaymentScheme {
    /// Exact payment scheme - fixed amount
    Exact,
    /// UpTo payment scheme - variable amount up to max (future)
    UpTo,
}

/// Blockchain network identifier
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Network {
    /// Network ID (e.g., "1" for Ethereum mainnet, "8453" for Base)
    pub id: String,
    /// Network name (e.g., "ethereum", "base", "polygon")
    pub name: String,
}

/// Payment Required Response (402 status response body)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentRequiredResponse {
    /// Version of the x402 payment protocol
    pub x402_version: u32,
    
    /// List of payment requirements that the resource server accepts
    pub accepts: Vec<PaymentRequirements>,
    
    /// Error message from the resource server (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Payment Requirements for a resource
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentRequirements {
    /// Scheme of the payment protocol to use
    pub scheme: PaymentScheme,
    
    /// Network of the blockchain to send payment on
    pub network: String,
    
    /// Maximum amount required in atomic units (e.g., wei, smallest unit)
    pub max_amount_required: String,
    
    /// URL of resource to pay for
    pub resource: String,
    
    /// Description of the resource
    pub description: String,
    
    /// MIME type of the resource response
    pub mime_type: String,
    
    /// Output schema of the resource response (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schema: Option<serde_json::Value>,
    
    /// Address to pay value to
    pub pay_to: String,
    
    /// Maximum time in seconds for the resource server to respond
    pub max_timeout_seconds: u64,
    
    /// Address of the ERC20 contract (for token payments)
    pub asset: String,
    
    /// Extra information specific to the scheme
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<HashMap<String, serde_json::Value>>,
}

/// Payment Payload (sent in X-PAYMENT header as base64 encoded JSON)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentPayload {
    /// Version of the x402 payment protocol
    pub x402_version: u32,
    
    /// Scheme value of the accepted paymentRequirements
    pub scheme: PaymentScheme,
    
    /// Network id of the accepted paymentRequirements
    pub network: String,
    
    /// Scheme-dependent payload
    pub payload: serde_json::Value,
}

/// Verification Request (POST /verify)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationRequest {
    pub x402_version: u32,
    pub payment_header: String,
    pub payment_requirements: PaymentRequirements,
}

/// Verification Response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationResponse {
    pub is_valid: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_reason: Option<String>,
}

/// Settlement Request (POST /settle)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettlementRequest {
    pub x402_version: u32,
    pub payment_header: String,
    pub payment_requirements: PaymentRequirements,
}

/// Settlement Response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettlementResponse {
    /// Whether the payment was successful
    pub success: bool,
    
    /// Error message from the facilitator server
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    
    /// Transaction hash of the settled payment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    
    /// Network id of the blockchain the payment was settled on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_id: Option<String>,
}

/// Supported Schemes Response (GET /supported)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportedSchemesResponse {
    pub kinds: Vec<SchemeNetwork>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemeNetwork {
    pub scheme: PaymentScheme,
    pub network: String,
}

impl PaymentPayload {
    /// Encode payment payload to base64 JSON for X-PAYMENT header
    pub fn to_header_value(&self) -> Result<String, serde_json::Error> {
        let json = serde_json::to_string(self)?;
        Ok(base64::engine::general_purpose::STANDARD.encode(json.as_bytes()))
    }
    
    /// Decode payment payload from X-PAYMENT header
    pub fn from_header_value(header: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let decoded = base64::engine::general_purpose::STANDARD.decode(header)?;
        let json = String::from_utf8(decoded)?;
        let payload = serde_json::from_str(&json)?;
        Ok(payload)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payment_payload_encoding() {
        let payload = PaymentPayload {
            x402_version: 1,
            scheme: PaymentScheme::Exact,
            network: "base".to_string(),
            payload: serde_json::json!({"signature": "0x1234"}),
        };
        
        let encoded = payload.to_header_value().unwrap();
        let decoded = PaymentPayload::from_header_value(&encoded).unwrap();
        
        assert_eq!(payload.x402_version, decoded.x402_version);
        assert_eq!(payload.scheme, decoded.scheme);
        assert_eq!(payload.network, decoded.network);
    }
    
    #[test]
    fn test_payment_requirements_serialization() {
        let requirements = PaymentRequirements {
            scheme: PaymentScheme::Exact,
            network: "base".to_string(),
            max_amount_required: "10000".to_string(),
            resource: "/api/agent/run".to_string(),
            description: "Premium AI Agent".to_string(),
            mime_type: "application/json".to_string(),
            output_schema: None,
            pay_to: "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb".to_string(),
            max_timeout_seconds: 300,
            asset: "0x833589fcd6edb6e08f4c7c32d4f71b54bda02913".to_string(),
            extra: None,
        };
        
        let json = serde_json::to_string(&requirements).unwrap();
        let deserialized: PaymentRequirements = serde_json::from_str(&json).unwrap();
        
        assert_eq!(requirements.scheme, deserialized.scheme);
        assert_eq!(requirements.max_amount_required, deserialized.max_amount_required);
    }
}

