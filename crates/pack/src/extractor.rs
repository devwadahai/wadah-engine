use crate::{PackageManifest, integrity::verify_digest};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use tar::Archive;

pub struct PackageExtractor {
    package_path: PathBuf,
    output_dir: PathBuf,
}

impl PackageExtractor {
    pub fn new(package_path: &Path, output_dir: &Path) -> Self {
        Self {
            package_path: package_path.to_path_buf(),
            output_dir: output_dir.to_path_buf(),
        }
    }

    pub fn extract(&self) -> crate::Result<PackageManifest> {
        // Create output directory
        std::fs::create_dir_all(&self.output_dir)?;

        // Read and decompress archive
        let file = File::open(&self.package_path)?;
        let reader = BufReader::new(file);
        let decompressed = zstd::decode_all(reader)?;

        // Extract tar archive
        let mut archive = Archive::new(&decompressed[..]);
        archive.unpack(&self.output_dir)?;

        // Read and verify manifest
        let manifest_path = self.output_dir.join(crate::MANIFEST_FILE);
        let manifest_content = std::fs::read_to_string(&manifest_path)?;
        let manifest = PackageManifest::from_json(&manifest_content)?;

        // Verify integrity
        self.verify_integrity(&manifest)?;

        Ok(manifest)
    }

    fn verify_integrity(&self, manifest: &PackageManifest) -> crate::Result<()> {
        // Verify spec digest
        let spec_path = self.output_dir.join(crate::SPEC_FILE);
        if !verify_digest(&spec_path, &manifest.spec_digest)? {
            return Err(crate::PackError::IntegrityError(
                "Spec file integrity check failed".to_string()
            ));
        }

        // Verify lockfile if present
        if let Some(ref lock_digest) = manifest.lock_digest {
            let lock_path = self.output_dir.join(crate::LOCKFILE);
            if lock_path.exists() && !verify_digest(&lock_path, lock_digest)? {
                return Err(crate::PackError::IntegrityError(
                    "Lockfile integrity check failed".to_string()
                ));
            }
        }

        // Verify ToolCaps if present
        if let Some(ref toolcaps_digest) = manifest.toolcaps_digest {
            let toolcaps_path = self.output_dir.join(crate::TOOLCAPS_FILE);
            if toolcaps_path.exists() && !verify_digest(&toolcaps_path, toolcaps_digest)? {
                return Err(crate::PackError::IntegrityError(
                    "ToolCaps integrity check failed".to_string()
                ));
            }
        }

        // Verify artifacts
        for (rel_path, entry) in &manifest.artifacts {
            let full_path = self.output_dir.join("artifacts").join(rel_path);
            if full_path.exists() && !verify_digest(&full_path, &entry.digest)? {
                return Err(crate::PackError::IntegrityError(
                    format!("Artifact integrity check failed: {}", rel_path)
                ));
            }
        }

        Ok(())
    }

    pub fn extract_manifest_only(&self) -> crate::Result<PackageManifest> {
        // Read and decompress archive
        let file = File::open(&self.package_path)?;
        let reader = BufReader::new(file);
        let decompressed = zstd::decode_all(reader)?;

        // Extract only manifest
        let mut archive = Archive::new(&decompressed[..]);
        
        for entry in archive.entries()? {
            let mut entry = entry?;
            let path = entry.path()?;
            
            if path == Path::new(crate::MANIFEST_FILE) {
                let mut content = String::new();
                entry.read_to_string(&mut content)?;
                return PackageManifest::from_json(&content);
            }
        }

        Err(crate::PackError::ExtractionError(
            "Manifest not found in package".to_string()
        ))
    }
}

#[cfg(test)]
mod tests {
    // Note: extraction tests would need a valid .wpkg file
    // These are integration tests that should be run with sample packages
}

