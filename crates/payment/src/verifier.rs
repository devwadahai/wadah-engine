use crate::{error::*, types::*};
use alloy::primitives::{Address, U256};
use alloy::signers::{Signature as AlloySignature};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// EIP-712 Domain for x402 payments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Eip712Domain {
    pub name: String,
    pub version: String,
    pub chain_id: u64,
    pub verifying_contract: Address,
}

/// Payment data for "exact" scheme on EVM chains
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExactPaymentData {
    /// Payer's address
    pub from: String,
    
    /// Payee's address
    pub to: String,
    
    /// Amount in atomic units
    pub value: String,
    
    /// Nonce for replay protection
    pub nonce: String,
    
    /// Valid after timestamp
    pub valid_after: u64,
    
    /// Valid before timestamp
    pub valid_before: u64,
    
    /// EIP-712 signature
    pub signature: String,
}

/// Local payment verifier for "exact" scheme
pub struct LocalVerifier {
    /// Current block timestamp (or system time)
    pub current_timestamp: u64,
}

impl LocalVerifier {
    pub fn new() -> Self {
        Self {
            current_timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Verify an exact payment for EVM chains
    pub fn verify_exact_evm(
        &self,
        payment_data: &ExactPaymentData,
        requirements: &PaymentRequirements,
    ) -> Result<VerificationResponse> {
        // 1. Check addresses match
        let to_addr = payment_data.to.to_lowercase();
        let required_to = requirements.pay_to.to_lowercase();
        
        if to_addr != required_to {
            return Ok(VerificationResponse {
                is_valid: false,
                invalid_reason: Some(format!(
                    "Payee address mismatch: expected {}, got {}",
                    required_to, to_addr
                )),
            });
        }
        
        // 2. Check amount is sufficient
        let payment_value = U256::from_str(&payment_data.value)
            .map_err(|e| PaymentError::VerificationFailed(format!("Invalid value: {}", e)))?;
        let required_value = U256::from_str(&requirements.max_amount_required)
            .map_err(|e| PaymentError::VerificationFailed(format!("Invalid required amount: {}", e)))?;
        
        if payment_value < required_value {
            return Ok(VerificationResponse {
                is_valid: false,
                invalid_reason: Some(format!(
                    "Insufficient amount: required {}, got {}",
                    required_value, payment_value
                )),
            });
        }
        
        // 3. Check timestamp validity
        if self.current_timestamp < payment_data.valid_after {
            return Ok(VerificationResponse {
                is_valid: false,
                invalid_reason: Some("Payment not yet valid".to_string()),
            });
        }
        
        if self.current_timestamp > payment_data.valid_before {
            return Ok(VerificationResponse {
                is_valid: false,
                invalid_reason: Some("Payment expired".to_string()),
            });
        }
        
        // 4. Verify EIP-712 signature
        // Note: Full EIP-712 verification would require:
        // - Reconstructing the typed data hash
        // - Recovering the signer from the signature
        // - Verifying the signer matches the 'from' address
        // For now, we'll do basic signature format validation
        
        if !payment_data.signature.starts_with("0x") || payment_data.signature.len() != 132 {
            return Ok(VerificationResponse {
                is_valid: false,
                invalid_reason: Some("Invalid signature format".to_string()),
            });
        }
        
        // In production, you would:
        // 1. Reconstruct EIP-712 hash from payment data
        // 2. Recover signer address from signature
        // 3. Verify signer == payment_data.from
        // For now, we'll assume signature is valid if format is correct
        
        Ok(VerificationResponse {
            is_valid: true,
            invalid_reason: None,
        })
    }
    
    /// Verify payment from X-PAYMENT header
    pub fn verify_payment_header(
        &self,
        payment_header: &str,
        requirements: &PaymentRequirements,
    ) -> Result<VerificationResponse> {
        // Decode payment payload
        let payload = PaymentPayload::from_header_value(payment_header)
            .map_err(|e| PaymentError::InvalidHeader(e.to_string()))?;
        
        // Check scheme matches
        if payload.scheme != requirements.scheme {
            return Ok(VerificationResponse {
                is_valid: false,
                invalid_reason: Some(format!(
                    "Scheme mismatch: expected {:?}, got {:?}",
                    requirements.scheme, payload.scheme
                )),
            });
        }
        
        // Check network matches
        if payload.network != requirements.network {
            return Ok(VerificationResponse {
                is_valid: false,
                invalid_reason: Some(format!(
                    "Network mismatch: expected {}, got {}",
                    requirements.network, payload.network
                )),
            });
        }
        
        // Verify based on scheme
        match payload.scheme {
            PaymentScheme::Exact => {
                let payment_data: ExactPaymentData = serde_json::from_value(payload.payload)
                    .map_err(|e| PaymentError::InvalidHeader(format!("Invalid payment data: {}", e)))?;
                
                self.verify_exact_evm(&payment_data, requirements)
            }
            PaymentScheme::UpTo => {
                // Not implemented yet
                Ok(VerificationResponse {
                    is_valid: false,
                    invalid_reason: Some("UpTo scheme not yet supported".to_string()),
                })
            }
        }
    }
}

impl Default for LocalVerifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_exact_evm_valid() {
        let verifier = LocalVerifier::new();
        
        let payment_data = ExactPaymentData {
            from: "0x1111111111111111111111111111111111111111".to_string(),
            to: "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb".to_string(),
            value: "10000".to_string(),
            nonce: "1".to_string(),
            valid_after: 0,
            valid_before: u64::MAX,
            signature: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef12".to_string(),
        };
        
        let requirements = PaymentRequirements {
            scheme: PaymentScheme::Exact,
            network: "base".to_string(),
            max_amount_required: "10000".to_string(),
            resource: "/api/agent/run".to_string(),
            description: "Test agent".to_string(),
            mime_type: "application/json".to_string(),
            output_schema: None,
            pay_to: "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb".to_string(),
            max_timeout_seconds: 300,
            asset: "0x833589fcd6edb6e08f4c7c32d4f71b54bda02913".to_string(),
            extra: None,
        };
        
        let result = verifier.verify_exact_evm(&payment_data, &requirements).unwrap();
        assert!(result.is_valid);
    }
    
    #[test]
    fn test_verify_exact_evm_insufficient_amount() {
        let verifier = LocalVerifier::new();
        
        let payment_data = ExactPaymentData {
            from: "0x1111111111111111111111111111111111111111".to_string(),
            to: "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb".to_string(),
            value: "5000".to_string(),  // Less than required
            nonce: "1".to_string(),
            valid_after: 0,
            valid_before: u64::MAX,
            signature: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef12".to_string(),
        };
        
        let requirements = PaymentRequirements {
            scheme: PaymentScheme::Exact,
            network: "base".to_string(),
            max_amount_required: "10000".to_string(),
            resource: "/api/agent/run".to_string(),
            description: "Test agent".to_string(),
            mime_type: "application/json".to_string(),
            output_schema: None,
            pay_to: "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb".to_string(),
            max_timeout_seconds: 300,
            asset: "0x833589fcd6edb6e08f4c7c32d4f71b54bda02913".to_string(),
            extra: None,
        };
        
        let result = verifier.verify_exact_evm(&payment_data, &requirements).unwrap();
        assert!(!result.is_valid);
        assert!(result.invalid_reason.unwrap().contains("Insufficient amount"));
    }
}

