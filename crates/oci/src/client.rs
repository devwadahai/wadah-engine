use crate::reference::Reference;
use oci_distribution::client::{Client, ClientConfig, ClientProtocol};
use oci_distribution::manifest::{OciDescriptor, OciImageManifest, OciManifest};
use oci_distribution::secrets::RegistryAuth;
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

        // Create an ImageLayer
        use oci_distribution::client::ImageLayer;
        let layer = ImageLayer {
            data: package_data,
            media_type: crate::WADAH_MEDIA_TYPE.to_string(),
            annotations: None,
        };

        // Create empty config
        use oci_distribution::client::Config;
        let config = Config {
            data: b"{}".to_vec(),
            media_type: "application/vnd.wadah.config.v1+json".to_string(),
            annotations: None,
        };

        // Push using the high-level push method with auth
        self.client
            .push(&oci_ref, &[layer], config, &auth, None)
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
        let (manifest, _digest) = self
            .client
            .pull_manifest(&oci_ref, &auth)
            .await
            .map_err(|e| crate::OCIError::PullError(e.to_string()))?;

        // Extract layers from manifest
        let layers = match manifest {
            OciManifest::Image(img) => img.layers,
            OciManifest::ImageIndex(_) => {
                return Err(crate::OCIError::PullError(
                    "Image index not supported yet".to_string(),
                ));
            }
        };

        // Get the first layer (the package)
        let layer = layers
            .first()
            .ok_or_else(|| crate::OCIError::PullError("No layers in manifest".to_string()))?;

        // Create output file
        let mut file = tokio::fs::File::create(output_path)
            .await
            .map_err(|e| crate::OCIError::PullError(e.to_string()))?;

        // Pull blob directly to file
        self.client
            .pull_blob(&oci_ref, &layer.digest, &mut file)
            .await
            .map_err(|e| crate::OCIError::PullError(e.to_string()))?;

        Ok(())
    }

    fn compute_digest(&self, data: &[u8]) -> String {
        use sha2::{Digest, Sha256};
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
    fn test_compute_digest() {
        let client = OCIClient::new();
        let data = b"hello world";
        let digest = client.compute_digest(data);
        assert!(digest.starts_with("sha256:"));
    }
}
