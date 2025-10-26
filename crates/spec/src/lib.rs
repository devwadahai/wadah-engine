pub mod wadah_spec;
pub mod toolcaps;
pub mod lockfile;
pub mod validation;
pub mod plugins;

pub use wadah_spec::{WadahSpec, Metadata, Runtime, ModelConfig, MemoryConfig, ToolConfig, Policy, Budgets, NetworkPolicy, Artifacts};
pub use toolcaps::{ToolCaps, ToolCapRule, ActionLimit};
pub use lockfile::{Lockfile, ModelLock, ToolLock};
pub use plugins::{PluginConfig, SecurityPlugin};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SpecError {
    #[error("Invalid specification: {0}")]
    InvalidSpec(String),
    
    #[error("Validation failed: {0}")]
    ValidationFailed(String),
    
    #[error("YAML parsing error: {0}")]
    YamlError(#[from] serde_yaml::Error),
    
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, SpecError>;

/// Security level for Wadah runtime
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityLevel {
    /// No security restrictions (dev mode)
    Permissive,
    /// Basic safety checks only
    Standard,
    /// Full security enforcement
    Strict,
}

impl Default for SecurityLevel {
    fn default() -> Self {
        Self::Standard
    }
}
