//! Environment variable sanitization — strips unsafe characters and normalizes keys/values.

use crate::error::EnvLayerError;
use std::collections::HashMap;

/// Rules controlling sanitization behavior.
#[derive(Debug, Clone)]
pub struct SanitizerConfig {
    /// Replace non-alphanumeric/underscore characters in keys with `_`.
    pub normalize_keys: bool,
    /// Strip leading/trailing whitespace from values.
    pub trim_values: bool,
    /// Remove entries whose values are empty after sanitization.
    pub drop_empty_values: bool,
    /// Convert all keys to uppercase.
    pub uppercase_keys: bool,
}

impl Default for SanitizerConfig {
    fn default() -> Self {
        Self {
            normalize_keys: true,
            trim_values: true,
            drop_empty_values: false,
            uppercase_keys: true,
        }
    }
}

/// Sanitizes a map of environment variables according to the provided config.
pub struct EnvSanitizer {
    config: SanitizerConfig,
}

impl EnvSanitizer {
    pub fn new(config: SanitizerConfig) -> Self {
        Self { config }
    }

    /// Sanitize a single key according to config rules.
    pub fn sanitize_key(&self, key: &str) -> Result<String, EnvLayerError> {
        if key.is_empty() {
            return Err(EnvLayerError::InvalidKey("key must not be empty".into()));
        }
        let mut k = if self.config.normalize_keys {
            key.chars()
                .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
                .collect::<String>()
        } else {
            key.to_string()
        };
        if self.config.uppercase_keys {
            k = k.to_uppercase();
        }
        Ok(k)
    }

    /// Sanitize a single value according to config rules.
    pub fn sanitize_value(&self, value: &str) -> String {
        if self.config.trim_values {
            value.trim().to_string()
        } else {
            value.to_string()
        }
    }

    /// Sanitize an entire environment map, returning a cleaned map.
    pub fn sanitize(
        &self,
        env: &HashMap<String, String>,
    ) -> Result<HashMap<String, String>, EnvLayerError> {
        let mut result = HashMap::new();
        for (k, v) in env {
            let clean_key = self.sanitize_key(k)?;
            let clean_val = self.sanitize_value(v);
            if self.config.drop_empty_values && clean_val.is_empty() {
                continue;
            }
            result.insert(clean_key, clean_val);
        }
        Ok(result)
    }
}
