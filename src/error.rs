//! Error types for envlayer.

use std::fmt;

/// All errors that can be produced by envlayer operations.
#[derive(Debug, PartialEq)]
pub enum EnvLayerError {
    /// A required environment variable was not found.
    MissingVariable(String),
    /// A variable value failed validation.
    ValidationFailed { key: String, reason: String },
    /// An I/O error occurred while reading a source.
    IoError(String),
    /// A layer with the given name was not found.
    LayerNotFound(String),
    /// A circular variable reference was detected during interpolation.
    CircularReference(String),
    /// A source could not be parsed.
    ParseError(String),
    /// An export operation failed.
    ExportError(String),
    /// A snapshot operation failed.
    SnapshotError(String),
}

impl fmt::Display for EnvLayerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnvLayerError::MissingVariable(key) => {
                write!(f, "Missing environment variable: '{key}'")
            }
            EnvLayerError::ValidationFailed { key, reason } => {
                write!(f, "Validation failed for '{key}': {reason}")
            }
            EnvLayerError::IoError(msg) => write!(f, "I/O error: {msg}"),
            EnvLayerError::LayerNotFound(name) => {
                write!(f, "Layer not found: '{name}'")
            }
            EnvLayerError::CircularReference(key) => {
                write!(f, "Circular reference detected for variable: '{key}'")
            }
            EnvLayerError::ParseError(msg) => write!(f, "Parse error: {msg}"),
            EnvLayerError::ExportError(msg) => write!(f, "Export error: {msg}"),
            EnvLayerError::SnapshotError(msg) => write!(f, "Snapshot error: {msg}"),
        }
    }
}

impl std::error::Error for EnvLayerError {}
