use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WadahSpec {
    pub api_version: String,
    pub kind: String,
    pub metadata: Metadata,
    pub runtime: Runtime,
    
    // Security plugins are now optional
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<Policy>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugins: Option<crate::plugins::PluginConfig>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<Artifacts>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock: Option<crate::lockfile::Lockfile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub authors: Vec<String>,
    pub license: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Runtime {
    pub model: ModelConfig,
    pub memory: Option<MemoryConfig>,
    #[serde(default)]
    pub tools: Vec<ToolConfig>,
    #[serde(default)]
    pub env: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub provider: String,
    pub endpoint: Option<String>,
    pub model_id: String,
    #[serde(default)]
    pub params: ModelParams,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModelParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u64>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    #[serde(rename = "type")]
    pub memory_type: String,
    pub index: Option<PathBuf>,
    #[serde(default)]
    pub params: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolConfig {
    pub id: String,
    #[serde(rename = "type")]
    pub tool_type: String,
    pub manifest: Option<PathBuf>,
    #[serde(default)]
    pub config: HashMap<String, serde_json::Value>,
}

/// Policy is now optional - for backwards compatibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub toolcaps: Option<PathBuf>,
    pub budgets: Option<Budgets>,
    pub network: Option<NetworkPolicy>,
    #[serde(default)]
    pub filesystem: FilesystemPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Budgets {
    pub tokens_per_minute: Option<u64>,
    pub usd_per_day: Option<f64>,
    pub max_duration_secs: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkPolicy {
    #[serde(default)]
    pub allow_domains: Vec<String>,
    #[serde(default)]
    pub deny_domains: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FilesystemPolicy {
    #[serde(default)]
    pub allow_paths: Vec<PathBuf>,
    #[serde(default)]
    pub deny_paths: Vec<PathBuf>,
    #[serde(default)]
    pub read_only: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifacts {
    #[serde(default)]
    pub include: Vec<String>,
    #[serde(default)]
    pub exclude: Vec<String>,
}

impl WadahSpec {
    pub fn from_yaml(content: &str) -> crate::Result<Self> {
        let spec: WadahSpec = serde_yaml::from_str(content)?;
        spec.validate()?;
        Ok(spec)
    }

    pub fn to_yaml(&self) -> crate::Result<String> {
        Ok(serde_yaml::to_string(self)?)
    }

    pub fn from_file(path: &std::path::Path) -> crate::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Self::from_yaml(&content)
    }

    pub fn to_file(&self, path: &std::path::Path) -> crate::Result<()> {
        let content = self.to_yaml()?;
        std::fs::write(path, content)?;
        Ok(())
    }

    fn validate(&self) -> crate::Result<()> {
        if self.api_version != "wadah.ai/v0.1" {
            return Err(crate::SpecError::ValidationFailed(
                format!("Unsupported API version: {}", self.api_version)
            ));
        }

        if self.kind != "Agent" {
            return Err(crate::SpecError::ValidationFailed(
                format!("Invalid kind: {}, expected 'Agent'", self.kind)
            ));
        }

        if self.metadata.name.is_empty() {
            return Err(crate::SpecError::ValidationFailed(
                "Agent name cannot be empty".to_string()
            ));
        }

        let valid_providers = ["openai", "tgi", "vllm", "ollama"];
        if !valid_providers.contains(&self.runtime.model.provider.as_str()) {
            return Err(crate::SpecError::ValidationFailed(
                format!("Invalid model provider: {}", self.runtime.model.provider)
            ));
        }

        Ok(())
    }

    /// Check if security features are enabled
    pub fn has_security(&self) -> bool {
        self.policy.is_some() || self.plugins.as_ref().map(|p| !p.plugins.is_empty()).unwrap_or(false)
    }

    /// Get security level based on configuration
    pub fn security_level(&self) -> crate::SecurityLevel {
        if self.policy.is_none() && self.plugins.is_none() {
            crate::SecurityLevel::Permissive
        } else if self.plugins.as_ref().map(|p| p.plugins.len()).unwrap_or(0) >= 3 {
            crate::SecurityLevel::Strict
        } else {
            crate::SecurityLevel::Standard
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_minimal_spec() {
        let yaml = r#"
apiVersion: wadah.ai/v0.1
kind: Agent
metadata:
  name: test-agent
  version: 0.1.0
  authors: ["Test Author"]
runtime:
  model:
    provider: openai
    modelId: gpt-4o-mini
"#;

        let spec = WadahSpec::from_yaml(yaml).unwrap();
        assert_eq!(spec.metadata.name, "test-agent");
        assert_eq!(spec.runtime.model.provider, "openai");
        assert_eq!(spec.security_level(), crate::SecurityLevel::Permissive);
    }

    #[test]
    fn test_security_level_detection() {
        let yaml = r#"
apiVersion: wadah.ai/v0.1
kind: Agent
metadata:
  name: test-agent
  version: 0.1.0
  authors: ["Test"]
runtime:
  model:
    provider: openai
    modelId: gpt-4o-mini
"#;
        let spec = WadahSpec::from_yaml(yaml).unwrap();
        assert_eq!(spec.security_level(), crate::SecurityLevel::Permissive);
    }
}
