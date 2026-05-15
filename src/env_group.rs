use std::collections::HashMap;
use crate::error::EnvLayerError;

/// A named group of environment variables that can be applied or removed together.
#[derive(Debug, Clone)]
pub struct EnvGroup {
    name: String,
    vars: HashMap<String, String>,
}

impl EnvGroup {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            vars: HashMap::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.vars.insert(key.into(), value.into());
        self
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.vars.get(key)
    }

    pub fn vars(&self) -> &HashMap<String, String> {
        &self.vars
    }

    pub fn len(&self) -> usize {
        self.vars.len()
    }

    pub fn is_empty(&self) -> bool {
        self.vars.is_empty()
    }

    pub fn merge(&mut self, other: &EnvGroup) {
        for (k, v) in &other.vars {
            self.vars.insert(k.clone(), v.clone());
        }
    }

    pub fn keys_with_prefix(&self, prefix: &str) -> Vec<&String> {
        self.vars.keys().filter(|k| k.starts_with(prefix)).collect()
    }

    pub fn validate_keys(&self) -> Result<(), EnvLayerError> {
        for key in self.vars.keys() {
            if key.trim().is_empty() {
                return Err(EnvLayerError::InvalidKey(
                    format!("Group '{}' contains an empty key", self.name),
                ));
            }
            if key.contains('=') {
                return Err(EnvLayerError::InvalidKey(
                    format!("Key '{}' in group '{}' contains '='", key, self.name),
                ));
            }
        }
        Ok(())
    }
}
