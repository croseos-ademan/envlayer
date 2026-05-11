use std::collections::HashMap;
use crate::env_context::{ContextKind, EnvContext};
use crate::error::EnvLayerError;

/// Resolves environment variables based on the active context,
/// allowing different values per context kind.
#[derive(Debug, Default)]
pub struct ContextResolver {
    overrides: HashMap<String, HashMap<String, String>>,
    defaults: HashMap<String, String>,
}

impl ContextResolver {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a value for a specific key under a given context kind.
    pub fn set_for_context(
        &mut self,
        context: &ContextKind,
        key: impl Into<String>,
        value: impl Into<String>,
    ) {
        self.overrides
            .entry(context.as_str().to_string())
            .or_default()
            .insert(key.into(), value.into());
    }

    /// Register a fallback default value for a key.
    pub fn set_default(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.defaults.insert(key.into(), value.into());
    }

    /// Resolve a key's value for the given context, falling back to defaults.
    pub fn resolve(
        &self,
        key: &str,
        context: &EnvContext,
    ) -> Result<Option<String>, EnvLayerError> {
        let ctx_key = context.kind.as_str();
        if let Some(ctx_map) = self.overrides.get(ctx_key) {
            if let Some(val) = ctx_map.get(key) {
                return Ok(Some(val.clone()));
            }
        }
        Ok(self.defaults.get(key).cloned())
    }

    /// Resolve all keys for the given context, merging defaults with context overrides.
    pub fn resolve_all(
        &self,
        context: &EnvContext,
    ) -> Result<HashMap<String, String>, EnvLayerError> {
        let mut result = self.defaults.clone();
        let ctx_key = context.kind.as_str();
        if let Some(ctx_map) = self.overrides.get(ctx_key) {
            for (k, v) in ctx_map {
                result.insert(k.clone(), v.clone());
            }
        }
        Ok(result)
    }
}
