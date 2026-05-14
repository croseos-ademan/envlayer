use std::collections::HashMap;
use crate::error::EnvLayerError;
use crate::layer::Layer;

/// Loads environment variables from multiple sources into a unified map,
/// respecting layer priority ordering.
#[derive(Debug, Default)]
pub struct EnvLoader {
    layers: Vec<Layer>,
    strict: bool,
}

impl EnvLoader {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_layer(mut self, layer: Layer) -> Self {
        self.layers.push(layer);
        self
    }

    pub fn strict(mut self, strict: bool) -> Self {
        self.strict = strict;
        self
    }

    /// Load all layers and merge into a single map.
    /// Later layers override earlier ones.
    pub fn load(&self) -> Result<HashMap<String, String>, EnvLayerError> {
        let mut result: HashMap<String, String> = HashMap::new();

        for layer in &self.layers {
            let vars = layer.vars();
            for (key, value) in vars {
                if self.strict && key.trim().is_empty() {
                    return Err(EnvLayerError::InvalidKey(
                        "Empty key encountered in strict mode".to_string(),
                    ));
                }
                result.insert(key.clone(), value.clone());
            }
        }

        Ok(result)
    }

    /// Load and return only keys that appear in all layers (intersection).
    pub fn load_common(&self) -> Result<HashMap<String, String>, EnvLayerError> {
        if self.layers.is_empty() {
            return Ok(HashMap::new());
        }

        let first_keys: std::collections::HashSet<String> =
            self.layers[0].vars().keys().cloned().collect();

        let common_keys = self.layers.iter().skip(1).fold(first_keys, |acc, layer| {
            let keys: std::collections::HashSet<String> =
                layer.vars().keys().cloned().collect();
            acc.intersection(&keys).cloned().collect()
        });

        let merged = self.load()?;
        Ok(merged
            .into_iter()
            .filter(|(k, _)| common_keys.contains(k))
            .collect())
    }

    pub fn layer_count(&self) -> usize {
        self.layers.len()
    }
}
