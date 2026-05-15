use std::collections::HashMap;
use crate::error::EnvLayerError;

/// Represents a namespace that prefixes environment variable keys.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnvNamespace {
    prefix: String,
    separator: String,
}

impl EnvNamespace {
    pub fn new(prefix: impl Into<String>) -> Self {
        Self {
            prefix: prefix.into(),
            separator: "_".to_string(),
        }
    }

    pub fn with_separator(mut self, sep: impl Into<String>) -> Self {
        self.separator = sep.into();
        self
    }

    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    pub fn separator(&self) -> &str {
        &self.separator
    }

    /// Qualify a key with the namespace prefix.
    pub fn qualify(&self, key: &str) -> String {
        format!("{}{}{}", self.prefix, self.separator, key)
    }

    /// Strip the namespace prefix from a key, returning None if it doesn't match.
    pub fn strip(&self, key: &str) -> Option<String> {
        let full_prefix = format!("{}{}", self.prefix, self.separator);
        key.strip_prefix(&full_prefix).map(|s| s.to_string())
    }

    /// Apply namespace prefix to all keys in a map.
    pub fn apply_to_map(&self, map: &HashMap<String, String>) -> HashMap<String, String> {
        map.iter()
            .map(|(k, v)| (self.qualify(k), v.clone()))
            .collect()
    }

    /// Extract only keys belonging to this namespace, stripping the prefix.
    pub fn extract_from_map(
        &self,
        map: &HashMap<String, String>,
    ) -> Result<HashMap<String, String>, EnvLayerError> {
        let result = map
            .iter()
            .filter_map(|(k, v)| self.strip(k).map(|stripped| (stripped, v.clone())))
            .collect();
        Ok(result)
    }
}
