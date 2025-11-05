use crate::ui;
use anyhow::Result;
use oci_distribution::secrets::RegistryAuth;
use std::path::Path;
use wadah_oci::{OCIClient, Reference};

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
            if reference.is_localhost() {
                return Ok(RegistryAuth::Anonymous);
            }
            
            // Try to read from Docker config
            if let Ok(auth) = read_docker_credentials(&reference.registry) {
                return Ok(auth);
            }
            
            ui::warning("No credentials found, using anonymous access");
            Ok(RegistryAuth::Anonymous)
        }
    }
}

fn read_docker_credentials(registry: &str) -> Result<RegistryAuth> {
    use std::fs;
    use std::path::PathBuf;
    
    let home = std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE"))?;
    let config_path = PathBuf::from(home).join(".docker").join("config.json");
    
    if !config_path.exists() {
        return Err(anyhow::anyhow!("Docker config not found"));
    }
    
    let config_content = fs::read_to_string(&config_path)?;
    let config: serde_json::Value = serde_json::from_str(&config_content)?;
    
    // Check for credential helpers first (per-registry)
    if let Some(cred_helpers) = config.get("credHelpers").and_then(|v| v.as_object()) {
        if cred_helpers.contains_key(registry) {
            return get_credentials_from_helper(registry, cred_helpers.get(registry).and_then(|v| v.as_str()).unwrap_or("desktop"));
        }
    }
    
    // Check for global credential store
    if let Some(creds_store) = config.get("credsStore").and_then(|v| v.as_str()) {
        return get_credentials_from_helper(registry, creds_store);
    }
    
    // Check for auths (base64 encoded credentials)
    if let Some(auths) = config.get("auths").and_then(|v| v.as_object()) {
        let registry_key = format!("https://{}", registry);
        if let Some(auth) = auths.get(&registry_key).or_else(|| auths.get(registry)) {
            if let Some(auth_str) = auth.get("auth").and_then(|v| v.as_str()) {
                use base64::Engine;
                let decoded = base64::engine::general_purpose::STANDARD.decode(auth_str)?;
                let auth_string = String::from_utf8(decoded)?;
                if let Some((username, password)) = auth_string.split_once(':') {
                    return Ok(RegistryAuth::Basic(username.to_string(), password.to_string()));
                }
            }
        }
    }
    
    Err(anyhow::anyhow!("No credentials found for registry"))
}

fn get_credentials_from_helper(registry: &str, helper: &str) -> Result<RegistryAuth> {
    use std::process::Command;
    use std::io::Write;
    
    let helper_cmd = format!("docker-credential-{}", helper);
    
    let mut child = Command::new(&helper_cmd)
        .arg("get")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()?;
    
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(registry.as_bytes())?;
    }
    
    let output = child.wait_with_output()?;
    
    if output.status.success() {
        let creds: serde_json::Value = serde_json::from_slice(&output.stdout)?;
        let username = creds.get("Username").and_then(|v| v.as_str()).ok_or_else(|| anyhow::anyhow!("No username"))?;
        let secret = creds.get("Secret").and_then(|v| v.as_str()).ok_or_else(|| anyhow::anyhow!("No secret"))?;
        
        return Ok(RegistryAuth::Basic(username.to_string(), secret.to_string()));
    }
    
    Err(anyhow::anyhow!("Credential helper failed"))
}
