pub mod client;
pub mod reference;

pub use client::OCIClient;
pub use reference::Reference;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum OCIError {
    #[error("OCI client error: {0}")]
    ClientError(String),

    #[error("Invalid reference: {0}")]
    InvalidReference(String),

    #[error("Push failed: {0}")]
    PushError(String),

    #[error("Pull failed: {0}")]
    PullError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("OCI distribution error: {0}")]
    DistributionError(String),
}

pub type Result<T> = std::result::Result<T, OCIError>;

pub const WADAH_MEDIA_TYPE: &str = "application/vnd.wadah.package.v1+zstd";
pub const OCI_MANIFEST_MEDIA_TYPE: &str = "application/vnd.oci.image.manifest.v1+json";
