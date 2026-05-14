use std::collections::HashMap;
use crate::scoped_env::ScopedEnv;
use crate::error::EnvLayerError;

/// Registry that manages multiple named ScopedEnv instances.
#[derive(Debug, Default)]
pub struct ScopeRegistry {
    scopes: HashMap<String, ScopedEnv>,
}

impl ScopeRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a scope. Returns an error if a scope with the same prefix already exists.
    pub fn register(&mut self, scope: ScopedEnv) -> Result<(), EnvLayerError> {
        let prefix = scope.prefix().to_string();
        if self.scopes.contains_key(&prefix) {
            return Err(EnvLayerError::InvalidInput(format!(
                "Scope with prefix '{}' is already registered",
                prefix
            )));
        }
        self.scopes.insert(prefix, scope);
        Ok(())
    }

    /// Get a reference to a scope by prefix.
    pub fn get(&self, prefix: &str) -> Option<&ScopedEnv> {
        self.scopes.get(prefix)
    }

    /// Get a mutable reference to a scope by prefix.
    pub fn get_mut(&mut self, prefix: &str) -> Option<&mut ScopedEnv> {
        self.scopes.get_mut(prefix)
    }

    /// Remove a scope by prefix.
    pub fn remove(&mut self, prefix: &str) -> Option<ScopedEnv> {
        self.scopes.remove(prefix)
    }

    /// Flatten all scopes into a single variable map with prefixed keys.
    pub fn flatten(&self) -> HashMap<String, String> {
        let mut result = HashMap::new();
        for scope in self.scopes.values() {
            result.extend(scope.prefixed_vars());
        }
        result
    }

    /// Returns the number of registered scopes.
    pub fn len(&self) -> usize {
        self.scopes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.scopes.is_empty()
    }
}
