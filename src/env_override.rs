use std::collections::HashMap;
use crate::error::EnvLayerError;

/// Represents a single environment variable override with an optional reason.
#[derive(Debug, Clone, PartialEq)]
pub struct EnvOverride {
    pub key: String,
    pub value: String,
    pub reason: Option<String>,
}

impl EnvOverride {
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
            reason: None,
        }
    }

    pub fn with_reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }
}

/// A collection of overrides that can be applied on top of an existing environment map.
#[derive(Debug, Clone, Default)]
pub struct OverrideSet {
    overrides: Vec<EnvOverride>,
}

impl OverrideSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, entry: EnvOverride) -> &mut Self {
        self.overrides.push(entry);
        self
    }

    pub fn from_map(map: HashMap<String, String>) -> Self {
        let overrides = map
            .into_iter()
            .map(|(k, v)| EnvOverride::new(k, v))
            .collect();
        Self { overrides }
    }

    /// Apply the override set onto a base environment map, returning the merged result.
    pub fn apply(&self, base: &HashMap<String, String>) -> Result<HashMap<String, String>, EnvLayerError> {
        let mut result = base.clone();
        for entry in &self.overrides {
            if entry.key.is_empty() {
                return Err(EnvLayerError::InvalidKey("Override key must not be empty".to_string()));
            }
            result.insert(entry.key.clone(), entry.value.clone());
        }
        Ok(result)
    }

    /// Returns only the overrides that differ from the base map.
    pub fn effective<'a>(&'a self, base: &HashMap<String, String>) -> Vec<&'a EnvOverride> {
        self.overrides
            .iter()
            .filter(|e| base.get(&e.key).map(|v| v != &e.value).unwrap_or(true))
            .collect()
    }

    pub fn len(&self) -> usize {
        self.overrides.len()
    }

    pub fn is_empty(&self) -> bool {
        self.overrides.is_empty()
    }
}
