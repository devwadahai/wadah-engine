pub mod builder;
pub mod extractor;
pub mod integrity;
pub mod manifest;

pub use builder::PackageBuilder;
pub use extractor::PackageExtractor;
pub use integrity::{compute_digest, verify_digest, DigestAlgorithm};
pub use manifest::PackageManifest;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum PackError {
    #[error("Package build error: {0}")]
    BuildError(String),

    #[error("Package extraction error: {0}")]
    ExtractionError(String),

    #[error("Integrity check failed: {0}")]
    IntegrityError(String),

    #[error("Manifest error: {0}")]
    ManifestError(String),

    #[error("Spec error: {0}")]
    SpecError(#[from] wadah_spec::SpecError),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, PackError>;

pub const WPKG_VERSION: &str = "0.1";
pub const MANIFEST_FILE: &str = "manifest.json";
pub const SPEC_FILE: &str = "wadah.yaml";
pub const LOCKFILE: &str = "wadah.lock";
pub const TOOLCAPS_FILE: &str = "ToolCaps.json";
