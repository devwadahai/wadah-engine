use anyhow::Result;
use std::path::Path;
use wadah_oci::{OCIClient, Reference};
use oci_distribution::secrets::RegistryAuth;
use crate::ui;

pub async fn execute(reference_str: &str, output_path: &str, insecure: bool) -> Result<()> {
    let spinner = ui::create_spinner("Pulling package...");
    
    let reference = Reference::parse(reference_str)?;
    let output = Path::new(output_path);
    
    // Create parent directory if needed
    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    // Create OCI client
    let mut client = if insecure || reference.is_localhost() {
        OCIClient::with_insecure()
    } else {
        OCIClient::new()
    };
    
    // Get authentication
    let auth = get_registry_auth(&reference)?;
    
    // Pull package
    client.pull(&reference, output, auth).await?;
    
    spinner.finish_and_clear();
    
    ui::success(&format!("Pulled: {}", reference));
    ui::info(&format!("  Saved to: {}", output.display()));
    
    // Show package info
    let metadata = std::fs::metadata(output)?;
    ui::info(&format!("  Size: {} bytes", metadata.len()));
    
    Ok(())
}

fn get_registry_auth(reference: &Reference) -> Result<RegistryAuth> {
    let username = std::env::var(format!("{}_USERNAME", reference.registry.to_uppercase().replace('.', "_")))
        .or_else(|_| std::env::var("REGISTRY_USERNAME"))
        .ok();
    
    let password = std::env::var(format!("{}_PASSWORD", reference.registry.to_uppercase().replace('.', "_")))
        .or_else(|_| std::env::var("REGISTRY_PASSWORD"))
        .ok();
    
    match (username, password) {
        (Some(u), Some(p)) => Ok(RegistryAuth::Basic(u, p)),
        _ => {
            if reference.is_localhost() {
                Ok(RegistryAuth::Anonymous)
            } else {
                ui::warning("No credentials found, using anonymous access");
                Ok(RegistryAuth::Anonymous)
            }
        }
    }
}

