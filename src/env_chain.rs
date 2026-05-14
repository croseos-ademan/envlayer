//! Ordered chain of environment layers with priority-based resolution.

use std::collections::HashMap;
use crate::layer::Layer;
use crate::error::EnvLayerError;

/// Represents an ordered chain of layers, resolved in priority order.
#[derive(Debug, Clone)]
pub struct EnvChain {
    layers: Vec<(u8, Layer)>,
}

impl EnvChain {
    /// Create a new empty chain.
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }

    /// Add a layer with a given priority (higher = wins).
    pub fn push(&mut self, priority: u8, layer: Layer) {
        self.layers.push((priority, layer));
        self.layers.sort_by_key(|(p, _)| std::cmp::Reverse(*p));
    }

    /// Resolve a key by walking the chain from highest to lowest priority.
    pub fn resolve(&self, key: &str) -> Option<String> {
        for (_, layer) in &self.layers {
            if let Some(val) = layer.get(key) {
                return Some(val.to_string());
            }
        }
        None
    }

    /// Flatten all layers into a single map (highest priority wins).
    pub fn flatten(&self) -> HashMap<String, String> {
        let mut result: HashMap<String, String> = HashMap::new();
        // Iterate lowest priority first so higher priority overwrites.
        for (_, layer) in self.layers.iter().rev() {
            for (k, v) in layer.entries() {
                result.insert(k.to_string(), v.to_string());
            }
        }
        result
    }

    /// Return the number of layers in the chain.
    pub fn len(&self) -> usize {
        self.layers.len()
    }

    /// Return true if no layers are present.
    pub fn is_empty(&self) -> bool {
        self.layers.is_empty()
    }

    /// Remove all layers with the given priority.
    pub fn remove_priority(&mut self, priority: u8) -> Result<(), EnvLayerError> {
        let before = self.layers.len();
        self.layers.retain(|(p, _)| *p != priority);
        if self.layers.len() == before {
            return Err(EnvLayerError::NotFound(format!("No layer with priority {priority}")));
        }
        Ok(())
    }
}

impl Default for EnvChain {
    fn default() -> Self {
        Self::new()
    }
}
