use std::collections::HashMap;
use crate::error::EnvLayerError;

/// A filter that can include or exclude environment variables by prefix, suffix, or exact key.
#[derive(Debug, Clone, Default)]
pub struct Filter {
    pub include_prefixes: Vec<String>,
    pub exclude_prefixes: Vec<String>,
    pub include_keys: Vec<String>,
    pub exclude_keys: Vec<String>,
}

impl Filter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_include_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.include_prefixes.push(prefix.into());
        self
    }

    pub fn with_exclude_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.exclude_prefixes.push(prefix.into());
        self
    }

    pub fn with_include_key(mut self, key: impl Into<String>) -> Self {
        self.include_keys.push(key.into());
        self
    }

    pub fn with_exclude_key(mut self, key: impl Into<String>) -> Self {
        self.exclude_keys.push(key.into());
        self
    }

    /// Apply the filter to a map of environment variables, returning only matching entries.
    pub fn apply(&self, vars: &HashMap<String, String>) -> Result<HashMap<String, String>, EnvLayerError> {
        let mut result = HashMap::new();

        for (key, value) in vars {
            if self.is_excluded(key) {
                continue;
            }
            if self.is_included(key) {
                result.insert(key.clone(), value.clone());
            }
        }

        Ok(result)
    }

    /// Returns `true` if this filter has no include or exclude rules defined.
    pub fn is_empty(&self) -> bool {
        self.include_prefixes.is_empty()
            && self.exclude_prefixes.is_empty()
            && self.include_keys.is_empty()
            && self.exclude_keys.is_empty()
    }

    fn is_excluded(&self, key: &str) -> bool {
        if self.exclude_keys.iter().any(|k| k == key) {
            return true;
        }
        self.exclude_prefixes.iter().any(|p| key.starts_with(p.as_str()))
    }

    fn is_included(&self, key: &str) -> bool {
        // If no include rules are defined, include everything
        if self.include_prefixes.is_empty() && self.include_keys.is_empty() {
            return true;
        }
        if self.include_keys.iter().any(|k| k == key) {
            return true;
        }
        self.include_prefixes.iter().any(|p| key.starts_with(p.as_str()))
    }
}
