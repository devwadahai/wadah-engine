use crate::ui;
use anyhow::Result;
use oci_distribution::secrets::RegistryAuth;
use std::path::Path;
use wadah_oci::{OCIClient, Reference};

pub async fn execute(reference_str: &str, package_path: &str, insecure: bool) -> Result<()> {
    let spinner = ui::create_spinner("Pushing package...");

    let reference = Reference::parse(reference_str)?;
    let package = Path::new(package_path);

    // Create OCI client
    let mut client = if insecure || reference.is_localhost() {
        OCIClient::with_insecure()
    } else {
        OCIClient::new()
    };

    // Get authentication (from env or Docker config)
    let auth = get_registry_auth(&reference)?;

    // Push package
    let digest = client.push(&reference, package, auth).await?;

    spinner.finish_and_clear();

    ui::success(&format!("Pushed: {}", reference));
    ui::info(&format!("  Digest: {}", digest));

    Ok(())
}

fn get_registry_auth(reference: &Reference) -> Result<RegistryAuth> {
    // Try to get from environment variables first
    let username = std::env::var(format!(
        "{}_USERNAME",
        reference.registry.to_uppercase().replace('.', "_")
    ))
    .or_else(|_| std::env::var("REGISTRY_USERNAME"))
    .ok();

    let password = std::env::var(format!(
        "{}_PASSWORD",
        reference.registry.to_uppercase().replace('.', "_")
    ))
    .or_else(|_| std::env::var("REGISTRY_PASSWORD"))
    .ok();

    match (username, password) {
        (Some(u), Some(p)) => Ok(RegistryAuth::Basic(u, p)),
        _ => {
            // Try anonymous for localhost
            if reference.is_localhost() {
                Ok(RegistryAuth::Anonymous)
            } else {
                // Try to use default Docker config
                ui::warning("No credentials found, using anonymous access");
                Ok(RegistryAuth::Anonymous)
            }
        }
    }
}
