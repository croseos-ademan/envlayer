//! Env freezer — snapshot and lock a set of environment variables to prevent
//! further mutation during critical sections.

use std::collections::HashMap;
use crate::error::EnvLayerError;

/// A frozen (immutable) view of a set of environment variables.
#[derive(Debug, Clone)]
pub struct EnvFreezer {
    frozen: HashMap<String, String>,
    locked: bool,
}

impl EnvFreezer {
    /// Create a new `EnvFreezer` from an existing map of variables.
    pub fn new(vars: HashMap<String, String>) -> Self {
        Self {
            frozen: vars,
            locked: false,
        }
    }

    /// Freeze the environment, preventing further updates.
    pub fn freeze(&mut self) {
        self.locked = true;
    }

    /// Returns `true` if the environment is currently frozen.
    pub fn is_frozen(&self) -> bool {
        self.locked
    }

    /// Retrieve a variable by key. Always available regardless of freeze state.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.frozen.get(key).map(String::as_str)
    }

    /// Attempt to insert or update a variable. Fails if frozen.
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) -> Result<(), EnvLayerError> {
        if self.locked {
            return Err(EnvLayerError::InvalidLayer(
                "Cannot mutate a frozen environment".to_string(),
            ));
        }
        self.frozen.insert(key.into(), value.into());
        Ok(())
    }

    /// Attempt to remove a variable. Fails if frozen.
    pub fn remove(&mut self, key: &str) -> Result<Option<String>, EnvLayerError> {
        if self.locked {
            return Err(EnvLayerError::InvalidLayer(
                "Cannot mutate a frozen environment".to_string(),
            ));
        }
        Ok(self.frozen.remove(key))
    }

    /// Return a clone of all frozen variables.
    pub fn snapshot(&self) -> HashMap<String, String> {
        self.frozen.clone()
    }

    /// Thaw the environment, allowing mutations again.
    pub fn thaw(&mut self) {
        self.locked = false;
    }
}
