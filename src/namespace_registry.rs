use std::collections::HashMap;
use crate::env_namespace::EnvNamespace;
use crate::error::EnvLayerError;

/// Registry that manages multiple named namespaces.
#[derive(Debug, Default)]
pub struct NamespaceRegistry {
    namespaces: HashMap<String, EnvNamespace>,
}

impl NamespaceRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, name: impl Into<String>, ns: EnvNamespace) {
        self.namespaces.insert(name.into(), ns);
    }

    pub fn get(&self, name: &str) -> Option<&EnvNamespace> {
        self.namespaces.get(name)
    }

    pub fn remove(&mut self, name: &str) -> Option<EnvNamespace> {
        self.namespaces.remove(name)
    }

    pub fn list(&self) -> Vec<&str> {
        self.namespaces.keys().map(|k| k.as_str()).collect()
    }

    pub fn resolve(
        &self,
        name: &str,
        key: &str,
    ) -> Result<String, EnvLayerError> {
        self.namespaces
            .get(name)
            .map(|ns| ns.qualify(key))
            .ok_or_else(|| EnvLayerError::NotFound(format!("Namespace '{}' not found", name)))
    }

    pub fn contains(&self, name: &str) -> bool {
        self.namespaces.contains_key(name)
    }
}
