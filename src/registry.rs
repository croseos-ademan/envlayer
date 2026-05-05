use crate::{EnvLayer, EnvLayerError, LayerSource};
use indexmap::IndexMap;
use std::collections::HashMap;

/// Ordered collection of layers. Later layers shadow earlier ones.
#[derive(Debug, Default)]
pub struct LayerRegistry {
    /// Ordered list of layer names (defines priority: last wins).
    order: Vec<String>,
    layers: HashMap<String, EnvLayer>,
}

impl LayerRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Push a new layer on top (highest priority so far).
    pub fn push(&mut self, layer: EnvLayer) -> Result<(), EnvLayerError> {
        if self.layers.contains_key(&layer.name) {
            return Err(EnvLayerError::DuplicateLayer(layer.name.clone()));
        }
        self.order.push(layer.name.clone());
        self.layers.insert(layer.name.clone(), layer);
        Ok(())
    }

    /// Remove a layer by name.
    pub fn remove(&mut self, name: &str) -> Result<EnvLayer, EnvLayerError> {
        if !self.layers.contains_key(name) {
            return Err(EnvLayerError::LayerNotFound(name.to_string()));
        }
        self.order.retain(|n| n != name);
        Ok(self.layers.remove(name).unwrap())
    }

    /// Resolve a key by scanning layers from last (highest priority) to first.
    pub fn get(&self, key: &str) -> Result<&str, EnvLayerError> {
        for name in self.order.iter().rev() {
            if let Some(v) = self.layers[name].get(key) {
                return Ok(v);
            }
        }
        Err(EnvLayerError::KeyNotFound(key.to_string()))
    }

    /// Merge all layers into a single flat map (later layers win).
    pub fn resolve_all(&self) -> IndexMap<String, String> {
        let mut merged: IndexMap<String, String> = IndexMap::new();
        for name in &self.order {
            for (k, v) in self.layers[name].iter() {
                merged.insert(k.to_string(), v.to_string());
            }
        }
        merged
    }

    /// Number of registered layers.
    pub fn layer_count(&self) -> usize {
        self.layers.len()
    }

    /// Ordered layer names (lowest → highest priority).
    pub fn layer_names(&self) -> &[String] {
        &self.order
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_layer(name: &str, pairs: &[(&str, &str)]) -> EnvLayer {
        let mut layer = EnvLayer::new(name, LayerSource::Inline);
        for (k, v) in pairs {
            layer.set(*k, *v);
        }
        layer
    }

    #[test]
    fn push_and_resolve_single_layer() {
        let mut reg = LayerRegistry::new();
        reg.push(make_layer("base", &[("APP_ENV", "development")])).unwrap();
        assert_eq!(reg.get("APP_ENV").unwrap(), "development");
    }

    #[test]
    fn later_layer_overrides_earlier() {
        let mut reg = LayerRegistry::new();
        reg.push(make_layer("base", &[("LOG_LEVEL", "info"), ("PORT", "8080")])).unwrap();
        reg.push(make_layer("ci", &[("LOG_LEVEL", "debug")])).unwrap();
        assert_eq!(reg.get("LOG_LEVEL").unwrap(), "debug");
        assert_eq!(reg.get("PORT").unwrap(), "8080");
    }

    #[test]
    fn duplicate_layer_returns_error() {
        let mut reg = LayerRegistry::new();
        reg.push(make_layer("base", &[])).unwrap();
        let err = reg.push(make_layer("base", &[])).unwrap_err();
        assert_eq!(err, EnvLayerError::DuplicateLayer("base".into()));
    }

    #[test]
    fn missing_key_returns_error() {
        let reg = LayerRegistry::new();
        assert_eq!(reg.get("MISSING").unwrap_err(), EnvLayerError::KeyNotFound("MISSING".into()));
    }

    #[test]
    fn remove_layer_then_key_gone() {
        let mut reg = LayerRegistry::new();
        reg.push(make_layer("base", &[("X", "1")])).unwrap();
        reg.push(make_layer("override", &[("X", "2")])).unwrap();
        reg.remove("override").unwrap();
        assert_eq!(reg.get("X").unwrap(), "1");
    }

    #[test]
    fn resolve_all_merges_correctly() {
        let mut reg = LayerRegistry::new();
        reg.push(make_layer("base", &[("A", "1"), ("B", "2")])).unwrap();
        reg.push(make_layer("ci", &[("B", "99"), ("C", "3")])).unwrap();
        let map = reg.resolve_all();
        assert_eq!(map["A"], "1");
        assert_eq!(map["B"], "99");
        assert_eq!(map["C"], "3");
    }
}
