use std::collections::HashMap;
use crate::error::EnvLayerError;
use crate::layer::Layer;
use crate::registry::Registry;

/// Resolves environment variables by walking through layers in priority order.
pub struct Resolver<'a> {
    registry: &'a Registry,
}

impl<'a> Resolver<'a> {
    pub fn new(registry: &'a Registry) -> Self {
        Self { registry }
    }

    /// Resolve a single key across all layers, returning the highest-priority value found.
    pub fn resolve(&self, key: &str) -> Option<String> {
        for layer in self.registry.layers_by_priority() {
            if let Some(value) = layer.get(key) {
                return Some(value.clone());
            }
        }
        None
    }

    /// Resolve a key, returning an error if it is not found in any layer.
    pub fn require(&self, key: &str) -> Result<String, EnvLayerError> {
        self.resolve(key)
            .ok_or_else(|| EnvLayerError::MissingKey(key.to_string()))
    }

    /// Resolve all keys present across every layer, with higher-priority layers winning.
    pub fn resolve_all(&self) -> HashMap<String, String> {
        let mut result: HashMap<String, String> = HashMap::new();
        // Iterate in reverse priority so higher-priority layers overwrite.
        let mut layers: Vec<&Layer> = self.registry.layers_by_priority().collect();
        layers.reverse();
        for layer in layers {
            for (key, value) in layer.entries() {
                result.insert(key.clone(), value.clone());
            }
        }
        result
    }

    /// Resolve a set of required keys, returning all values or an aggregated error.
    pub fn require_all(&self, keys: &[&str]) -> Result<HashMap<String, String>, EnvLayerError> {
        let mut missing: Vec<String> = Vec::new();
        let mut result = HashMap::new();
        for &key in keys {
            match self.resolve(key) {
                Some(value) => {
                    result.insert(key.to_string(), value);
                }
                None => missing.push(key.to_string()),
            }
        }
        if missing.is_empty() {
            Ok(result)
        } else {
            Err(EnvLayerError::MissingKeys(missing))
        }
    }
}
