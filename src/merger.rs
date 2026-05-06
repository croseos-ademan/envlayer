//! Layer merging logic for combining multiple environment layers
//! into a single resolved map with configurable conflict strategies.

use std::collections::HashMap;
use crate::layer::Layer;
use crate::error::EnvLayerError;

/// Strategy for resolving conflicts when multiple layers define the same key.
#[derive(Debug, Clone, PartialEq)]
pub enum MergeStrategy {
    /// Last layer wins (default behavior)
    LastWins,
    /// First layer wins; subsequent definitions are ignored
    FirstWins,
    /// Return an error if the same key appears in more than one layer
    ErrorOnConflict,
}

impl Default for MergeStrategy {
    fn default() -> Self {
        MergeStrategy::LastWins
    }
}

/// Merges a slice of layers into a single `HashMap<String, String>`
/// according to the provided `MergeStrategy`.
///
/// Layers are processed in order (index 0 first).
pub fn merge_layers(
    layers: &[Layer],
    strategy: &MergeStrategy,
) -> Result<HashMap<String, String>, EnvLayerError> {
    let mut result: HashMap<String, String> = HashMap::new();

    for layer in layers {
        for (key, value) in layer.entries() {
            match strategy {
                MergeStrategy::LastWins => {
                    result.insert(key.clone(), value.clone());
                }
                MergeStrategy::FirstWins => {
                    result.entry(key.clone()).or_insert_with(|| value.clone());
                }
                MergeStrategy::ErrorOnConflict => {
                    if result.contains_key(key) {
                        return Err(EnvLayerError::ConflictingKey(key.clone()));
                    }
                    result.insert(key.clone(), value.clone());
                }
            }
        }
    }

    Ok(result)
}
