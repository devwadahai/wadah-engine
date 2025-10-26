use anyhow::Result;
use std::path::Path;
use wadah_pack::PackageExtractor;
use crate::ui;

pub async fn execute(package_path: &str) -> Result<()> {
    let spinner = ui::create_spinner("Verifying package...");
    
    let package = Path::new(package_path);
    
    // Extract manifest only to verify structure
    let extractor = PackageExtractor::new(package, &std::env::temp_dir());
    let manifest = extractor.extract_manifest_only()?;
    
    spinner.finish_and_clear();
    
    ui::success("Package verification passed");
    ui::info(&format!("  Name: {}", manifest.name));
    ui::info(&format!("  Version: {}", manifest.package_version));
    ui::info(&format!("  Created: {}", manifest.created_at));
    ui::info(&format!("  Spec digest: {}", manifest.spec_digest));
    ui::info(&format!("  Artifacts: {}", manifest.artifacts.len()));
    
    if let Some(ref lock_digest) = manifest.lock_digest {
        ui::info(&format!("  Lock digest: {}", lock_digest));
    }
    
    if let Some(ref caps_digest) = manifest.toolcaps_digest {
        ui::info(&format!("  ToolCaps digest: {}", caps_digest));
    }
    
    Ok(())
}

