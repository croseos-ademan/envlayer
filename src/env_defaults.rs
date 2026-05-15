use std::collections::HashMap;
use crate::error::EnvLayerError;

/// Manages default values for environment variables.
/// Defaults are applied when a key is absent from the resolved environment.
#[derive(Debug, Clone, Default)]
pub struct EnvDefaults {
    defaults: HashMap<String, String>,
}

impl EnvDefaults {
    pub fn new() -> Self {
        Self {
            defaults: HashMap::new(),
        }
    }

    /// Register a default value for a key.
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.defaults.insert(key.into(), value.into());
    }

    /// Retrieve the default value for a key, if any.
    pub fn get(&self, key: &str) -> Option<&String> {
        self.defaults.get(key)
    }

    /// Apply defaults to the provided map, filling in missing keys.
    pub fn apply(&self, env: &mut HashMap<String, String>) {
        for (key, value) in &self.defaults {
            env.entry(key.clone()).or_insert_with(|| value.clone());
        }
    }

    /// Return all defaults as a cloned map.
    pub fn all(&self) -> HashMap<String, String> {
        self.defaults.clone()
    }

    /// Remove a registered default.
    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.defaults.remove(key)
    }

    /// Check whether a default is registered for the given key.
    pub fn contains(&self, key: &str) -> bool {
        self.defaults.contains_key(key)
    }

    /// Merge another EnvDefaults into this one. Existing keys are NOT overwritten.
    pub fn merge_from(&mut self, other: &EnvDefaults) {
        for (key, value) in &other.defaults {
            self.defaults.entry(key.clone()).or_insert_with(|| value.clone());
        }
    }

    /// Load defaults from a slice of (key, value) pairs.
    pub fn load_pairs(
        &mut self,
        pairs: &[(&str, &str)],
    ) -> Result<(), EnvLayerError> {
        for (k, v) in pairs {
            if k.is_empty() {
                return Err(EnvLayerError::InvalidKey("Default key must not be empty".into()));
            }
            self.defaults.insert(k.to_string(), v.to_string());
        }
        Ok(())
    }
}
