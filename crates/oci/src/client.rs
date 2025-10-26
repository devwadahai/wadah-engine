use crate::reference::Reference;
use oci_distribution::client::{Client, ClientConfig, ClientProtocol};
use oci_distribution::secrets::RegistryAuth;
use oci_distribution::manifest::{OciDescriptor, OciImageManifest};
use oci_distribution::Reference as OciReference;
use std::path::Path;
use tokio::fs;

pub struct OCIClient {
    client: Client,
}

impl OCIClient {
    pub fn new() -> Self {
        let config = ClientConfig {
            protocol: ClientProtocol::Https,
            ..Default::default()
        };

        Self {
            client: Client::new(config),
        }
    }

    pub fn with_insecure() -> Self {
        let config = ClientConfig {
            protocol: ClientProtocol::Http,
            ..Default::default()
        };

        Self {
            client: Client::new(config),
        }
    }

    pub async fn push(
        &mut self,
        reference: &Reference,
        package_path: &Path,
        auth: RegistryAuth,
    ) -> crate::Result<String> {
        // Read package file
        let package_data = fs::read(package_path)
            .await
            .map_err(|e| crate::OCIError::PushError(e.to_string()))?;

        // Create OCI reference
        let oci_ref = OciReference::try_from(reference.to_string())
            .map_err(|e| crate::OCIError::InvalidReference(e.to_string()))?;

        // Compute digest
        let digest = self.compute_digest(&package_data);

        // Create layer descriptor
        let layer = OciDescriptor {
            media_type: crate::WADAH_MEDIA_TYPE.to_string(),
            digest: digest.clone(),
            size: package_data.len() as i64,
            urls: None,
            annotations: None,
            data: None,
            platform: None,
            artifact_type: None,
        };

        // Create manifest
        let manifest = OciImageManifest {
            schema_version: 2,
            media_type: Some(crate::OCI_MANIFEST_MEDIA_TYPE.to_string()),
            config: OciDescriptor {
                media_type: "application/vnd.wadah.config.v1+json".to_string(),
                digest: "sha256:44136fa355b3678a1146ad16f7e8649e94fb4fc21fe77e8310c060f61caaff8a".to_string(), // empty json
                size: 2,
                urls: None,
                annotations: None,
                data: None,
                platform: None,
                artifact_type: None,
            },
            layers: vec![layer],
            subject: None,
            annotations: None,
        };

        // Push layers
        self.client
            .push_blob(&oci_ref, &package_data, &auth)
            .await
            .map_err(|e| crate::OCIError::PushError(e.to_string()))?;

        // Push manifest
        self.client
            .push_manifest(&oci_ref, &manifest, &auth)
            .await
            .map_err(|e| crate::OCIError::PushError(e.to_string()))?;

        Ok(digest)
    }

    pub async fn pull(
        &mut self,
        reference: &Reference,
        output_path: &Path,
        auth: RegistryAuth,
    ) -> crate::Result<()> {
        // Create OCI reference
        let oci_ref = OciReference::try_from(reference.to_string())
            .map_err(|e| crate::OCIError::InvalidReference(e.to_string()))?;

        // Pull manifest
        let (manifest, _digest) = self.client
            .pull_manifest(&oci_ref, &auth)
            .await
            .map_err(|e| crate::OCIError::PullError(e.to_string()))?;

        // Get the first layer (should be the package)
        let layer = manifest.layers.first()
            .ok_or_else(|| crate::OCIError::PullError("No layers in manifest".to_string()))?;

        // Pull layer
        let layer_data = self.client
            .pull_blob(&oci_ref, &layer.digest, &auth)
            .await
            .map_err(|e| crate::OCIError::PullError(e.to_string()))?;

        // Write to file
        fs::write(output_path, &layer_data)
            .await
            .map_err(|e| crate::OCIError::PullError(e.to_string()))?;

        Ok(())
    }

    fn compute_digest(&self, data: &[u8]) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        format!("sha256:{}", hex::encode(result))
    }
}

impl Default for OCIClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_client() {
        let _client = OCIClient::new();
        // Basic instantiation test
    }
}

