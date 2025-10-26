use wadah_spec::ToolCaps;

pub struct PolicyEnforcer {
    toolcaps: Option<ToolCaps>,
}

impl PolicyEnforcer {
    pub fn new(toolcaps: Option<ToolCaps>) -> Self {
        Self { toolcaps }
    }

    pub fn check_tool_action(&self, tool: &str, action: &str) -> crate::Result<()> {
        if let Some(ref caps) = self.toolcaps {
            if !caps.is_action_allowed(tool, action) {
                return Err(crate::RuntimeError::PolicyViolation(format!(
                    "Tool action not allowed: {}::{}",
                    tool, action
                )));
            }
        }
        Ok(())
    }

    pub fn check_network_domain(
        &self,
        domain: &str,
        policy: &wadah_spec::Policy,
    ) -> crate::Result<()> {
        if let Some(ref network_policy) = policy.network {
            // Check deny list first
            if network_policy
                .deny_domains
                .iter()
                .any(|d| domain.contains(d))
            {
                return Err(crate::RuntimeError::PolicyViolation(format!(
                    "Domain denied: {}",
                    domain
                )));
            }

            // Check allow list
            if !network_policy.allow_domains.is_empty()
                && !network_policy
                    .allow_domains
                    .iter()
                    .any(|d| domain.contains(d))
            {
                return Err(crate::RuntimeError::PolicyViolation(format!(
                    "Domain not in allow list: {}",
                    domain
                )));
            }
        }
        Ok(())
    }

    pub fn check_filesystem_path(
        &self,
        path: &std::path::Path,
        is_write: bool,
        policy: &wadah_spec::Policy,
    ) -> crate::Result<()> {
        // Check deny paths
        for deny_path in &policy.filesystem.deny_paths {
            if path.starts_with(deny_path) {
                return Err(crate::RuntimeError::PolicyViolation(format!(
                    "Path denied: {}",
                    path.display()
                )));
            }
        }

        // Check allow paths
        if !policy.filesystem.allow_paths.is_empty() {
            let mut allowed = false;
            for allow_path in &policy.filesystem.allow_paths {
                if path.starts_with(allow_path) {
                    allowed = true;
                    break;
                }
            }
            if !allowed {
                return Err(crate::RuntimeError::PolicyViolation(format!(
                    "Path not in allow list: {}",
                    path.display()
                )));
            }
        }

        // Check read-only mode
        if is_write && policy.filesystem.read_only {
            return Err(crate::RuntimeError::PolicyViolation(format!(
                "Write not allowed in read-only mode: {}",
                path.display()
            )));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_policy_enforcer() {
        let enforcer = PolicyEnforcer::new(None);

        // Without toolcaps, all actions should be allowed
        assert!(enforcer.check_tool_action("github", "read").is_ok());
    }

    #[test]
    fn test_filesystem_policy() {
        let policy = wadah_spec::Policy {
            toolcaps: None,
            budgets: None,
            network: None,
            filesystem: wadah_spec::FilesystemPolicy {
                allow_paths: vec![PathBuf::from("/workspace")],
                deny_paths: vec![],
                read_only: true,
            },
        };

        let enforcer = PolicyEnforcer::new(None);

        // Allow path should be OK for read
        assert!(enforcer
            .check_filesystem_path(&PathBuf::from("/workspace/file.txt"), false, &policy)
            .is_ok());

        // Write should fail in read-only mode
        assert!(enforcer
            .check_filesystem_path(&PathBuf::from("/workspace/file.txt"), true, &policy)
            .is_err());
    }
}
