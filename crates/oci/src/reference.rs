use std::fmt;

#[derive(Debug, Clone)]
pub struct Reference {
    pub registry: String,
    pub repository: String,
    pub tag: String,
}

impl Reference {
    pub fn parse(reference: &str) -> crate::Result<Self> {
        // Parse references like: ghcr.io/org/repo:tag or localhost:5000/repo:tag
        
        let parts: Vec<&str> = reference.split('/').collect();
        
        if parts.len() < 2 {
            return Err(crate::OCIError::InvalidReference(
                format!("Invalid reference format: {}", reference)
            ));
        }

        let registry = parts[0].to_string();
        
        // Join remaining parts except the last one
        let repo_and_tag = parts[1..].join("/");
        
        // Split by ':' to separate repository and tag
        let (repository, tag) = if let Some(pos) = repo_and_tag.rfind(':') {
            let repo = &repo_and_tag[..pos];
            let tag = &repo_and_tag[pos + 1..];
            (repo.to_string(), tag.to_string())
        } else {
            (repo_and_tag, "latest".to_string())
        };

        Ok(Self {
            registry,
            repository,
            tag,
        })
    }

    pub fn full_repository(&self) -> String {
        self.repository.clone()
    }

    pub fn is_localhost(&self) -> bool {
        self.registry.starts_with("localhost") || self.registry.starts_with("127.0.0.1")
    }
}

impl fmt::Display for Reference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}:{}", self.registry, self.repository, self.tag)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_reference() {
        let ref1 = Reference::parse("ghcr.io/zenri/wadah:0.1.0").unwrap();
        assert_eq!(ref1.registry, "ghcr.io");
        assert_eq!(ref1.repository, "zenri/wadah");
        assert_eq!(ref1.tag, "0.1.0");

        let ref2 = Reference::parse("localhost:5000/myagent").unwrap();
        assert_eq!(ref2.registry, "localhost:5000");
        assert_eq!(ref2.repository, "myagent");
        assert_eq!(ref2.tag, "latest");
    }

    #[test]
    fn test_display_reference() {
        let reference = Reference {
            registry: "ghcr.io".to_string(),
            repository: "zenri/wadah".to_string(),
            tag: "0.1.0".to_string(),
        };

        assert_eq!(reference.to_string(), "ghcr.io/zenri/wadah:0.1.0");
    }
}

