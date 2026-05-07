//! Masking support for sensitive environment variable values.
//!
//! Provides functionality to redact or partially obscure values
//! for secrets, tokens, and other sensitive variables.

use std::collections::HashSet;

/// Strategy for masking sensitive values.
#[derive(Debug, Clone, PartialEq)]
pub enum MaskStrategy {
    /// Replace entire value with asterisks.
    Full,
    /// Show only the last N characters, mask the rest.
    Partial(usize),
    /// Replace with a fixed placeholder string.
    Placeholder(String),
}

impl Default for MaskStrategy {
    fn default() -> Self {
        MaskStrategy::Full
    }
}

/// Masks sensitive environment variable values.
#[derive(Debug, Default)]
pub struct Masker {
    sensitive_keys: HashSet<String>,
    strategy: MaskStrategy,
}

impl Masker {
    /// Create a new `Masker` with the given strategy.
    pub fn new(strategy: MaskStrategy) -> Self {
        Self {
            sensitive_keys: HashSet::new(),
            strategy,
        }
    }

    /// Register a key as sensitive so its value will be masked.
    pub fn add_sensitive_key(&mut self, key: impl Into<String>) {
        self.sensitive_keys.insert(key.into());
    }

    /// Returns `true` if the key is registered as sensitive.
    pub fn is_sensitive(&self, key: &str) -> bool {
        self.sensitive_keys.contains(key)
    }

    /// Mask a value according to the configured strategy.
    pub fn mask_value(&self, value: &str) -> String {
        match &self.strategy {
            MaskStrategy::Full => "*".repeat(value.len().max(8)),
            MaskStrategy::Partial(show) => {
                if value.len() <= *show {
                    return "*".repeat(8);
                }
                let visible = &value[value.len() - show..];
                format!("{}{}", "*".repeat(value.len() - show), visible)
            }
            MaskStrategy::Placeholder(p) => p.clone(),
        }
    }

    /// Mask a value if the key is sensitive, otherwise return as-is.
    pub fn maybe_mask(&self, key: &str, value: &str) -> String {
        if self.is_sensitive(key) {
            self.mask_value(value)
        } else {
            value.to_string()
        }
    }
}
