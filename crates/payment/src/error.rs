use thiserror::Error;

#[derive(Debug, Error)]
pub enum PaymentError {
    #[error("Payment verification failed: {0}")]
    VerificationFailed(String),
    
    #[error("Payment settlement failed: {0}")]
    SettlementFailed(String),
    
    #[error("Invalid payment header: {0}")]
    InvalidHeader(String),
    
    #[error("Network not supported: {0}")]
    UnsupportedNetwork(String),
    
    #[error("Scheme not supported: {0}")]
    UnsupportedScheme(String),
    
    #[error("Insufficient payment amount: required {required}, got {actual}")]
    InsufficientAmount { required: String, actual: String },
    
    #[error("Payment expired")]
    PaymentExpired,
    
    #[error("Facilitator error: {0}")]
    FacilitatorError(String),
    
    #[error("Blockchain error: {0}")]
    BlockchainError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),
    
    #[error("Base64 decode error: {0}")]
    Base64Error(#[from] base64::DecodeError),
    
    #[error("UTF8 error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
}

pub type Result<T> = std::result::Result<T, PaymentError>;

