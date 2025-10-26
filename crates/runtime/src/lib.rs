pub mod adapters;
pub mod executor;
pub mod policy;
pub mod budget;

pub use adapters::{ModelAdapter, OpenAIAdapter, OllamaAdapter, TGIAdapter};
pub use executor::AgentExecutor;
pub use policy::PolicyEnforcer;
pub use budget::BudgetTracker;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("Model adapter error: {0}")]
    AdapterError(String),
    
    #[error("Policy violation: {0}")]
    PolicyViolation(String),
    
    #[error("Budget exceeded: {0}")]
    BudgetExceeded(String),
    
    #[error("Execution error: {0}")]
    ExecutionError(String),
    
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),
    
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("Spec error: {0}")]
    SpecError(#[from] wadah_spec::SpecError),
    
    #[error("Trace error: {0}")]
    TraceError(#[from] wadah_trace::TraceError),
}

pub type Result<T> = std::result::Result<T, RuntimeError>;

