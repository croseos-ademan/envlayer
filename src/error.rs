use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum EnvLayerError {
    #[error("layer '{0}' already exists in the registry")]
    DuplicateLayer(String),

    #[error("layer '{0}' not found")]
    LayerNotFound(String),

    #[error("key '{0}' is not present in any active layer")]
    KeyNotFound(String),
}
