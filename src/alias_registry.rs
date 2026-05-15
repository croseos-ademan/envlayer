use std::collections::HashMap;
use crate::env_alias::EnvAlias;
use crate::error::EnvLayerError;

/// Registry that manages multiple named alias sets (e.g., per-profile or per-scope).
#[derive(Debug, Default)]
pub struct AliasRegistry {
    entries: HashMap<String, EnvAlias>,
}

impl AliasRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert or replace the alias set for a given namespace.
    pub fn register(&mut self, namespace: impl Into<String>, alias: EnvAlias) {
        self.entries.insert(namespace.into(), alias);
    }

    /// Retrieve the alias set for a namespace.
    pub fn get(&self, namespace: &str) -> Option<&EnvAlias> {
        self.entries.get(namespace)
    }

    /// Resolve a key within a specific namespace.
    pub fn resolve(&self, namespace: &str, key: &str) -> Result<String, EnvLayerError> {
        match self.entries.get(namespace) {
            Some(alias) => Ok(alias.resolve(key).to_string()),
            None => Err(EnvLayerError::NotFound(format!(
                "namespace '{}' not found in alias registry",
                namespace
            ))),
        }
    }

    /// Return all registered namespace names.
    pub fn namespaces(&self) -> Vec<&str> {
        self.entries.keys().map(|s| s.as_str()).collect()
    }

    /// Remove a namespace from the registry.
    pub fn remove(&mut self, namespace: &str) -> Option<EnvAlias> {
        self.entries.remove(namespace)
    }
}
