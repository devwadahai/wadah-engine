use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCaps {
    pub version: String,
    #[serde(default)]
    pub allow: Vec<ToolCapRule>,
    #[serde(default)]
    pub deny: Vec<DenyRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCapRule {
    pub tool: String,
    #[serde(default)]
    pub actions: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<ActionLimit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs: Option<FsPolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionLimit {
    pub per_min: Option<u64>,
    pub per_hour: Option<u64>,
    pub per_day: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FsPolicy {
    #[serde(default)]
    pub paths: Vec<String>,
    #[serde(default = "default_true")]
    pub write: bool,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DenyRule {
    pub tool: String,
    #[serde(default)]
    pub actions: Vec<String>,
    pub reason: Option<String>,
}

impl ToolCaps {
    pub fn from_json(content: &str) -> crate::Result<Self> {
        let caps: ToolCaps = serde_json::from_str(content)?;
        caps.validate()?;
        Ok(caps)
    }

    pub fn to_json(&self) -> crate::Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    pub fn from_file(path: &Path) -> crate::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Self::from_json(&content)
    }

    pub fn to_file(&self, path: &Path) -> crate::Result<()> {
        let content = self.to_json()?;
        std::fs::write(path, content)?;
        Ok(())
    }

    fn validate(&self) -> crate::Result<()> {
        if self.version != "0.1" {
            return Err(crate::SpecError::ValidationFailed(
                format!("Unsupported ToolCaps version: {}", self.version)
            ));
        }
        Ok(())
    }

    pub fn is_action_allowed(&self, tool: &str, action: &str) -> bool {
        // Check deny rules first
        for deny in &self.deny {
            if deny.tool == tool && (deny.actions.is_empty() || deny.actions.contains(&action.to_string())) {
                return false;
            }
        }

        // Check allow rules
        for allow in &self.allow {
            if allow.tool == tool && (allow.actions.is_empty() || allow.actions.contains(&action.to_string())) {
                return true;
            }
        }

        // Default deny
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_toolcaps() {
        let json = r#"
{
  "version": "0.1",
  "allow": [
    {
      "tool": "github",
      "actions": ["read_issues", "create_issue"],
      "limits": {"per_min": 10}
    }
  ],
  "deny": [
    {
      "tool": "wallet",
      "actions": ["transfer"],
      "reason": "not allowed in dev"
    }
  ]
}
"#;

        let caps = ToolCaps::from_json(json).unwrap();
        assert_eq!(caps.version, "0.1");
        assert_eq!(caps.allow.len(), 1);
        assert_eq!(caps.deny.len(), 1);
    }

    #[test]
    fn test_action_allowed() {
        let caps = ToolCaps {
            version: "0.1".to_string(),
            allow: vec![
                ToolCapRule {
                    tool: "github".to_string(),
                    actions: vec!["read_issues".to_string()],
                    limits: None,
                    fs: None,
                    domains: None,
                    extra: HashMap::new(),
                }
            ],
            deny: vec![
                DenyRule {
                    tool: "wallet".to_string(),
                    actions: vec!["transfer".to_string()],
                    reason: Some("security".to_string()),
                }
            ],
        };

        assert!(caps.is_action_allowed("github", "read_issues"));
        assert!(!caps.is_action_allowed("github", "delete_repo"));
        assert!(!caps.is_action_allowed("wallet", "transfer"));
    }
}

