use std::fmt;

/// Central error type for envlayer.
#[derive(Debug)]
pub enum EnvLayerError {
    /// A required environment variable was not found in any layer.
    NotFound(String),
    /// An I/O error occurred while reading a source.
    Io(std::io::Error),
    /// Parsing a source file failed.
    ParseError(String),
    /// A merge conflict could not be resolved.
    MergeConflict(String),
    /// One or more validation rules failed.
    ValidationFailed(String),
    /// A layer with the given name was already registered.
    DuplicateLayer(String),
}

impl fmt::Display for EnvLayerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnvLayerError::NotFound(key) => write!(f, "Variable not found: {}", key),
            EnvLayerError::Io(e) => write!(f, "I/O error: {}", e),
            EnvLayerError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            EnvLayerError::MergeConflict(msg) => write!(f, "Merge conflict: {}", msg),
            EnvLayerError::ValidationFailed(msg) => write!(f, "Validation failed: {}", msg),
            EnvLayerError::DuplicateLayer(name) => {
                write!(f, "Duplicate layer name: {}", name)
            }
        }
    }
}

impl std::error::Error for EnvLayerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            EnvLayerError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for EnvLayerError {
    fn from(e: std::io::Error) -> Self {
        EnvLayerError::Io(e)
    }
}

pub type Result<T> = std::result::Result<T, EnvLayerError>;
