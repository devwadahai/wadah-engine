use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lockfile {
    pub version: String,
    pub generated_at: String,
    pub model: ModelLock,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeddings: Option<ModelLock>,
    #[serde(default)]
    pub tools: Vec<ToolLock>,
    #[serde(default)]
    pub artifacts: HashMap<String, String>, // path -> digest
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelLock {
    pub provider: String,
    pub model_id: String,
    pub digest: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolLock {
    pub id: String,
    pub version: String,
    pub digest: String,
}

impl Lockfile {
    pub fn new(model: ModelLock) -> Self {
        Self {
            version: "0.1".to_string(),
            generated_at: chrono::Utc::now().to_rfc3339(),
            model,
            embeddings: None,
            tools: Vec::new(),
            artifacts: HashMap::new(),
        }
    }

    pub fn from_yaml(content: &str) -> crate::Result<Self> {
        Ok(serde_yaml::from_str(content)?)
    }

    pub fn to_yaml(&self) -> crate::Result<String> {
        Ok(serde_yaml::to_string(self)?)
    }

    pub fn from_file(path: &Path) -> crate::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Self::from_yaml(&content)
    }

    pub fn to_file(&self, path: &Path) -> crate::Result<()> {
        let content = self.to_yaml()?;
        std::fs::write(path, content)?;
        Ok(())
    }

    pub fn add_tool(&mut self, tool: ToolLock) {
        self.tools.push(tool);
    }

    pub fn add_artifact(&mut self, path: String, digest: String) {
        self.artifacts.insert(path, digest);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_lockfile() {
        let model_lock = ModelLock {
            provider: "openai".to_string(),
            model_id: "gpt-4o-mini".to_string(),
            digest: "sha256:abc123".to_string(),
            snapshot: None,
        };

        let mut lockfile = Lockfile::new(model_lock);
        lockfile.add_tool(ToolLock {
            id: "github".to_string(),
            version: "1.0.0".to_string(),
            digest: "sha256:def456".to_string(),
        });

        assert_eq!(lockfile.version, "0.1");
        assert_eq!(lockfile.tools.len(), 1);
    }
}

