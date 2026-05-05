use std::fmt;

#[derive(Debug, PartialEq)]
pub enum EnvLayerError {
    /// A single required key was not found in any layer.
    MissingKey(String),
    /// Multiple required keys were not found in any layer.
    MissingKeys(Vec<String>),
    /// A layer with the given name already exists in the registry.
    DuplicateLayer(String),
    /// A generic I/O or parse error when loading a layer from a source.
    LoadError(String),
}

impl fmt::Display for EnvLayerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnvLayerError::MissingKey(key) => {
                write!(f, "required environment variable '{}' not found", key)
            }
            EnvLayerError::MissingKeys(keys) => {
                write!(f, "required environment variables not found: {}", keys.join(", "))
            }
            EnvLayerError::DuplicateLayer(name) => {
                write!(f, "layer '{}' already exists in the registry", name)
            }
            EnvLayerError::LoadError(msg) => {
                write!(f, "failed to load layer: {}", msg)
            }
        }
    }
}

impl std::error::Error for EnvLayerError {}
