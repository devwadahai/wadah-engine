pub mod oat;
pub mod recorder;
pub mod replay;

pub use oat::{OATEvent, OATSpan, OATTrace, EventType, SpanKind};
pub use recorder::TraceRecorder;
pub use replay::TraceReplayer;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TraceError {
    #[error("Trace recording error: {0}")]
    RecordingError(String),
    
    #[error("Trace replay error: {0}")]
    ReplayError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, TraceError>;

