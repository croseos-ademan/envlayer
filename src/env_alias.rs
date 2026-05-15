use std::collections::HashMap;
use crate::error::EnvLayerError;

/// Maps alias names to canonical environment variable keys.
#[derive(Debug, Clone, Default)]
pub struct EnvAlias {
    aliases: HashMap<String, String>,
}

impl EnvAlias {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register an alias pointing to a canonical key.
    pub fn register(&mut self, alias: impl Into<String>, canonical: impl Into<String>) {
        self.aliases.insert(alias.into(), canonical.into());
    }

    /// Resolve an alias to its canonical key, or return the input unchanged.
    pub fn resolve<'a>(&'a self, key: &'a str) -> &'a str {
        self.aliases.get(key).map(|s| s.as_str()).unwrap_or(key)
    }

    /// Return the canonical key for an alias, or an error if not found.
    pub fn resolve_strict(&self, alias: &str) -> Result<&str, EnvLayerError> {
        self.aliases
            .get(alias)
            .map(|s| s.as_str())
            .ok_or_else(|| EnvLayerError::NotFound(alias.to_string()))
    }

    /// Check whether a given key is a registered alias.
    pub fn is_alias(&self, key: &str) -> bool {
        self.aliases.contains_key(key)
    }

    /// Return all registered aliases.
    pub fn all_aliases(&self) -> &HashMap<String, String> {
        &self.aliases
    }

    /// Remove an alias entry.
    pub fn remove(&mut self, alias: &str) -> Option<String> {
        self.aliases.remove(alias)
    }

    /// Resolve all keys in a map, replacing alias keys with canonical keys.
    pub fn apply_to_map(&self, map: &HashMap<String, String>) -> HashMap<String, String> {
        map.iter()
            .map(|(k, v)| (self.resolve(k).to_string(), v.clone()))
            .collect()
    }
}
