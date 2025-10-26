use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Plugin configuration for extending Wadah functionality
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    pub plugins: Vec<Plugin>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plugin {
    pub id: String,
    pub enabled: bool,
    #[serde(default)]
    pub config: HashMap<String, serde_json::Value>,
}

/// Security plugins (all optional)
#[derive(Debug, Clone)]
pub enum SecurityPlugin {
    /// Tool capability enforcement
    ToolCaps,
    /// Budget and cost controls
    BudgetLimits,
    /// Network access policies
    NetworkPolicy,
    /// Filesystem restrictions
    FilesystemPolicy,
    /// Execution tracing
    Tracing,
}

impl SecurityPlugin {
    pub fn id(&self) -> &'static str {
        match self {
            Self::ToolCaps => "security.toolcaps",
            Self::BudgetLimits => "security.budgets",
            Self::NetworkPolicy => "security.network",
            Self::FilesystemPolicy => "security.filesystem",
            Self::Tracing => "observability.tracing",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::ToolCaps => "Enforce tool permission policies",
            Self::BudgetLimits => "Control token/cost/time budgets",
            Self::NetworkPolicy => "Restrict network access by domain",
            Self::FilesystemPolicy => "Control filesystem read/write access",
            Self::Tracing => "Record execution traces for audit",
        }
    }
}

impl PluginConfig {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    pub fn with_plugin(mut self, plugin: Plugin) -> Self {
        self.plugins.push(plugin);
        self
    }

    pub fn is_enabled(&self, plugin_id: &str) -> bool {
        self.plugins
            .iter()
            .find(|p| p.id == plugin_id)
            .map(|p| p.enabled)
            .unwrap_or(false)
    }

    pub fn get_config(&self, plugin_id: &str) -> Option<&HashMap<String, serde_json::Value>> {
        self.plugins
            .iter()
            .find(|p| p.id == plugin_id && p.enabled)
            .map(|p| &p.config)
    }
}

impl Default for PluginConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Preset plugin bundles
impl PluginConfig {
    /// No plugins - permissive mode
    pub fn permissive() -> Self {
        Self::new()
    }

    /// Standard bundle - basic safety
    pub fn standard() -> Self {
        Self::new()
            .with_plugin(Plugin {
                id: SecurityPlugin::BudgetLimits.id().to_string(),
                enabled: true,
                config: {
                    let mut cfg = HashMap::new();
                    cfg.insert("usd_per_day".to_string(), serde_json::json!(100.0));
                    cfg.insert("max_duration_secs".to_string(), serde_json::json!(3600));
                    cfg
                },
            })
    }

    /// Strict bundle - all security features
    pub fn strict() -> Self {
        Self::new()
            .with_plugin(Plugin {
                id: SecurityPlugin::ToolCaps.id().to_string(),
                enabled: true,
                config: HashMap::new(),
            })
            .with_plugin(Plugin {
                id: SecurityPlugin::BudgetLimits.id().to_string(),
                enabled: true,
                config: {
                    let mut cfg = HashMap::new();
                    cfg.insert("usd_per_day".to_string(), serde_json::json!(10.0));
                    cfg.insert("max_duration_secs".to_string(), serde_json::json!(600));
                    cfg
                },
            })
            .with_plugin(Plugin {
                id: SecurityPlugin::NetworkPolicy.id().to_string(),
                enabled: true,
                config: HashMap::new(),
            })
            .with_plugin(Plugin {
                id: SecurityPlugin::FilesystemPolicy.id().to_string(),
                enabled: true,
                config: HashMap::new(),
            })
            .with_plugin(Plugin {
                id: SecurityPlugin::Tracing.id().to_string(),
                enabled: true,
                config: HashMap::new(),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_config() {
        let config = PluginConfig::permissive();
        assert!(!config.is_enabled("security.toolcaps"));

        let config = PluginConfig::strict();
        assert!(config.is_enabled("security.toolcaps"));
        assert!(config.is_enabled("security.budgets"));
    }
}

