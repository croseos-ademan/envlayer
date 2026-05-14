use std::collections::HashMap;
use crate::error::EnvLayerError;

/// A scoped environment that isolates a subset of variables under a named prefix.
#[derive(Debug, Clone)]
pub struct ScopedEnv {
    prefix: String,
    vars: HashMap<String, String>,
}

impl ScopedEnv {
    /// Create a new ScopedEnv with the given prefix and variable map.
    pub fn new(prefix: impl Into<String>, vars: HashMap<String, String>) -> Self {
        Self {
            prefix: prefix.into(),
            vars,
        }
    }

    /// Return the prefix for this scope.
    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    /// Get a variable by its unqualified key (without prefix).
    pub fn get(&self, key: &str) -> Option<&String> {
        self.vars.get(key)
    }

    /// Set a variable by its unqualified key.
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.vars.insert(key.into(), value.into());
    }

    /// Remove a variable by its unqualified key.
    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.vars.remove(key)
    }

    /// Return all variables with their keys prefixed (e.g. `APP_KEY`).
    pub fn prefixed_vars(&self) -> HashMap<String, String> {
        self.vars
            .iter()
            .map(|(k, v)| (format!("{}_{}", self.prefix, k), v.clone()))
            .collect()
    }

    /// Merge another ScopedEnv into this one. Conflicting keys are overwritten.
    pub fn merge(&mut self, other: &ScopedEnv) -> Result<(), EnvLayerError> {
        if other.prefix != self.prefix {
            return Err(EnvLayerError::InvalidInput(format!(
                "Cannot merge scopes with different prefixes: '{}' vs '{}'",
                self.prefix, other.prefix
            )));
        }
        for (k, v) in &other.vars {
            self.vars.insert(k.clone(), v.clone());
        }
        Ok(())
    }

    /// Returns the number of variables in this scope.
    pub fn len(&self) -> usize {
        self.vars.len()
    }

    /// Returns true if there are no variables.
    pub fn is_empty(&self) -> bool {
        self.vars.is_empty()
    }
}
