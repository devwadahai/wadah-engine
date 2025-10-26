use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageManifest {
    pub version: String,
    pub name: String,
    pub package_version: String,
    pub created_at: String,
    pub spec_digest: String,
    pub lock_digest: Option<String>,
    pub toolcaps_digest: Option<String>,
    #[serde(default)]
    pub artifacts: HashMap<String, ArtifactEntry>,
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactEntry {
    pub path: String,
    pub digest: String,
    pub size: u64,
}

impl PackageManifest {
    pub fn new(name: String, package_version: String, spec_digest: String) -> Self {
        Self {
            version: crate::WPKG_VERSION.to_string(),
            name,
            package_version,
            created_at: chrono::Utc::now().to_rfc3339(),
            spec_digest,
            lock_digest: None,
            toolcaps_digest: None,
            artifacts: HashMap::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn add_artifact(&mut self, path: String, entry: ArtifactEntry) {
        self.artifacts.insert(path, entry);
    }

    pub fn to_json(&self) -> crate::Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    pub fn from_json(content: &str) -> crate::Result<Self> {
        Ok(serde_json::from_str(content)?)
    }
}

