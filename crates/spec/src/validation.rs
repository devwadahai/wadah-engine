use crate::{WadahSpec, ToolCaps};

pub trait Validator {
    fn validate(&self) -> crate::Result<()>;
}

impl Validator for WadahSpec {
    fn validate(&self) -> crate::Result<()> {
        // Implemented in wadah_spec.rs
        Ok(())
    }
}

impl Validator for ToolCaps {
    fn validate(&self) -> crate::Result<()> {
        // Implemented in toolcaps.rs
        Ok(())
    }
}

pub fn validate_package_name(name: &str) -> bool {
    // Package names must be lowercase alphanumeric with hyphens
    name.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        && !name.starts_with('-')
        && !name.ends_with('-')
}

pub fn validate_version(version: &str) -> bool {
    // Simple semver check
    let parts: Vec<&str> = version.split('.').collect();
    parts.len() == 3 && parts.iter().all(|p| p.parse::<u32>().is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_package_name() {
        assert!(validate_package_name("my-agent"));
        assert!(validate_package_name("agent123"));
        assert!(!validate_package_name("MyAgent"));
        assert!(!validate_package_name("-agent"));
        assert!(!validate_package_name("agent-"));
    }

    #[test]
    fn test_validate_version() {
        assert!(validate_version("0.1.0"));
        assert!(validate_version("1.2.3"));
        assert!(!validate_version("1.2"));
        assert!(!validate_version("v1.2.3"));
    }
}

