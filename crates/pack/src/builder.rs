use crate::{PackageManifest, manifest::ArtifactEntry};
use crate::integrity::{compute_digest, compute_digest_bytes, DigestAlgorithm};
use std::fs::File;
use std::io::{Write, BufReader};
use std::path::{Path, PathBuf};
use tar::{Builder as TarBuilder, Header};
use wadah_spec::WadahSpec;
use walkdir::WalkDir;

pub struct PackageBuilder {
    spec: WadahSpec,
    base_dir: PathBuf,
    output_path: PathBuf,
    manifest: PackageManifest,
}

impl PackageBuilder {
    pub fn new(spec_path: &Path, output_path: &Path) -> crate::Result<Self> {
        let spec = WadahSpec::from_file(spec_path)?;
        let base_dir = spec_path.parent()
            .ok_or_else(|| crate::PackError::BuildError("Invalid spec path".to_string()))?
            .to_path_buf();

        // Compute spec digest
        let spec_digest = compute_digest(spec_path, DigestAlgorithm::Sha256)?;

        let manifest = PackageManifest::new(
            spec.metadata.name.clone(),
            spec.metadata.version.clone(),
            spec_digest,
        );

        Ok(Self {
            spec,
            base_dir,
            output_path: output_path.to_path_buf(),
            manifest,
        })
    }

    pub fn build(mut self) -> crate::Result<PathBuf> {
        // Create temporary directory for staging
        let temp_dir = tempfile::tempdir()?;
        let staging_path = temp_dir.path();

        // Copy spec file
        self.add_spec_to_staging(staging_path)?;

        // Copy lockfile if exists
        self.add_lockfile_to_staging(staging_path)?;

        // Copy ToolCaps if specified
        self.add_toolcaps_to_staging(staging_path)?;

        // Copy artifacts
        self.add_artifacts_to_staging(staging_path)?;

        // Write manifest
        self.write_manifest(staging_path)?;

        // Create tar.zst archive
        self.create_archive(staging_path)?;

        Ok(self.output_path.clone())
    }

    fn add_spec_to_staging(&mut self, staging_path: &Path) -> crate::Result<()> {
        let spec_content = self.spec.to_yaml()?;
        let spec_file = staging_path.join(crate::SPEC_FILE);
        std::fs::write(&spec_file, &spec_content)?;

        let digest = compute_digest_bytes(spec_content.as_bytes(), DigestAlgorithm::Sha256);
        self.manifest.spec_digest = digest;

        Ok(())
    }

    fn add_lockfile_to_staging(&mut self, staging_path: &Path) -> crate::Result<()> {
        let lockfile_path = self.base_dir.join(crate::LOCKFILE);
        if lockfile_path.exists() {
            let lock_file = staging_path.join(crate::LOCKFILE);
            std::fs::copy(&lockfile_path, &lock_file)?;

            let digest = compute_digest(&lock_file, DigestAlgorithm::Sha256)?;
            self.manifest.lock_digest = Some(digest);
        }
        Ok(())
    }

    fn add_toolcaps_to_staging(&mut self, staging_path: &Path) -> crate::Result<()> {
        if let Some(ref policy) = self.spec.policy {
            if let Some(ref toolcaps_path) = policy.toolcaps {
                let full_path = self.base_dir.join(toolcaps_path);
                if full_path.exists() {
                    let dest_file = staging_path.join(crate::TOOLCAPS_FILE);
                    std::fs::copy(&full_path, &dest_file)?;

                    let digest = compute_digest(&dest_file, DigestAlgorithm::Sha256)?;
                    self.manifest.toolcaps_digest = Some(digest);
                }
            }
        }
        Ok(())
    }

    fn add_artifacts_to_staging(&mut self, staging_path: &Path) -> crate::Result<()> {
        let artifacts_dir = staging_path.join("artifacts");
        std::fs::create_dir_all(&artifacts_dir)?;

        if let Some(ref artifacts) = self.spec.artifacts {
            for pattern in &artifacts.include {
                self.copy_matching_files(pattern, &artifacts_dir)?;
            }
        }

        Ok(())
    }

    fn copy_matching_files(&mut self, pattern: &str, dest_dir: &Path) -> crate::Result<()> {
        // Simple glob-like pattern matching
        let pattern_parts: Vec<&str> = pattern.split('/').collect();
        
        for entry in WalkDir::new(&self.base_dir)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                let rel_path = entry.path().strip_prefix(&self.base_dir)
                    .map_err(|e| crate::PackError::BuildError(e.to_string()))?;

                if self.matches_pattern(rel_path, pattern) {
                    let dest_path = dest_dir.join(rel_path);
                    if let Some(parent) = dest_path.parent() {
                        std::fs::create_dir_all(parent)?;
                    }
                    std::fs::copy(entry.path(), &dest_path)?;

                    // Add to manifest
                    let metadata = std::fs::metadata(&dest_path)?;
                    let digest = compute_digest(&dest_path, DigestAlgorithm::Sha256)?;
                    
                    self.manifest.add_artifact(
                        rel_path.to_string_lossy().to_string(),
                        ArtifactEntry {
                            path: rel_path.to_string_lossy().to_string(),
                            digest,
                            size: metadata.len(),
                        },
                    );
                }
            }
        }

        Ok(())
    }

    fn matches_pattern(&self, path: &Path, pattern: &str) -> bool {
        let path_str = path.to_string_lossy();
        
        // Handle ** for recursive matching
        if pattern.contains("**") {
            let parts: Vec<&str> = pattern.split("**").collect();
            if parts.len() == 2 {
                let prefix = parts[0].trim_end_matches('/');
                let suffix = parts[1].trim_start_matches('/');
                
                return (prefix.is_empty() || path_str.starts_with(prefix))
                    && (suffix.is_empty() || path_str.ends_with(suffix));
            }
        }

        // Handle * for single directory component
        if pattern.contains('*') {
            // Simple wildcard matching
            let regex_pattern = pattern
                .replace(".", "\\.")
                .replace("*", ".*");
            
            if let Ok(re) = regex::Regex::new(&format!("^{}$", regex_pattern)) {
                return re.is_match(&path_str);
            }
        }

        path_str == pattern
    }

    fn write_manifest(&self, staging_path: &Path) -> crate::Result<()> {
        let manifest_content = self.manifest.to_json()?;
        let manifest_file = staging_path.join(crate::MANIFEST_FILE);
        std::fs::write(manifest_file, manifest_content)?;
        Ok(())
    }

    fn create_archive(&self, staging_path: &Path) -> crate::Result<()> {
        // Create parent directory if it doesn't exist
        if let Some(parent) = self.output_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Create tar archive in memory first
        let tar_data = Vec::new();
        let mut tar_builder = TarBuilder::new(tar_data);

        // Add all files from staging
        for entry in WalkDir::new(staging_path)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                let rel_path = entry.path().strip_prefix(staging_path)
                    .map_err(|e| crate::PackError::BuildError(e.to_string()))?;
                
                let file = File::open(entry.path())?;
                let metadata = file.metadata()?;
                let mut header = Header::new_gnu();
                header.set_size(metadata.len());
                header.set_mode(0o644);
                header.set_cksum();

                tar_builder.append_data(&mut header, rel_path, BufReader::new(file))?;
            }
        }

        let tar_data = tar_builder.into_inner()
            .map_err(|e| crate::PackError::BuildError(e.to_string()))?;

        // Compress with zstd
        let compressed = zstd::encode_all(&tar_data[..], 3)?;
        
        // Write to output file
        std::fs::write(&self.output_path, compressed)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use wadah_spec::{Metadata, Runtime, ModelConfig};

    #[test]
    fn test_package_builder() -> crate::Result<()> {
        let temp_dir = TempDir::new()?;
        let spec_path = temp_dir.path().join("wadah.yaml");
        
        let spec = WadahSpec {
            api_version: "wadah.ai/v0.1".to_string(),
            kind: "Agent".to_string(),
            metadata: Metadata {
                name: "test-agent".to_string(),
                version: "0.1.0".to_string(),
                description: None,
                authors: vec!["Test".to_string()],
                license: None,
                tags: vec![],
            },
            runtime: Runtime {
                model: ModelConfig {
                    provider: "openai".to_string(),
                    endpoint: None,
                    model_id: "gpt-4o-mini".to_string(),
                    params: Default::default(),
                },
                memory: None,
                tools: vec![],
                env: Default::default(),
            },
            policy: None,
            artifacts: None,
            lock: None,
        };
        
        spec.to_file(&spec_path)?;
        
        let output_path = temp_dir.path().join("test.wpkg");
        let builder = PackageBuilder::new(&spec_path, &output_path)?;
        let result = builder.build()?;
        
        assert!(result.exists());
        Ok(())
    }
}

