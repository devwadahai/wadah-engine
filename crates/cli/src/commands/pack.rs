use crate::ui;
use anyhow::Result;
use std::path::Path;
use wadah_pack::PackageBuilder;

pub async fn execute(manifest_path: &str, output_path: &str) -> Result<()> {
    let spinner = ui::create_spinner("Packaging agent...");

    let manifest = Path::new(manifest_path);
    let output = Path::new(output_path);

    // Build package
    let builder = PackageBuilder::new(manifest, output)?;
    let result = builder.build()?;

    spinner.finish_and_clear();

    ui::success(&format!("Package created: {}", result.display()));

    // Show package info
    let metadata = std::fs::metadata(&result)?;
    ui::info(&format!("  Size: {} bytes", metadata.len()));

    Ok(())
}
