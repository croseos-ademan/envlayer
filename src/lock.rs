//! Environment variable locking — prevents mutation of finalized layers.

use std::collections::HashSet;
use crate::error::EnvLayerError;

/// Tracks which keys are locked and prevents further modification.
#[derive(Debug, Default, Clone)]
pub struct EnvLock {
    locked: HashSet<String>,
}

impl EnvLock {
    /// Create a new, empty lock set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Lock a specific key so it cannot be overwritten.
    pub fn lock(&mut self, key: impl Into<String>) {
        self.locked.insert(key.into());
    }

    /// Lock multiple keys at once.
    pub fn lock_all(&mut self, keys: impl IntoIterator<Item = impl Into<String>>) {
        for key in keys {
            self.locked.insert(key.into());
        }
    }

    /// Returns `true` if the given key is locked.
    pub fn is_locked(&self, key: &str) -> bool {
        self.locked.contains(key)
    }

    /// Assert that a key is not locked; return an error if it is.
    pub fn assert_unlocked(&self, key: &str) -> Result<(), EnvLayerError> {
        if self.is_locked(key) {
            Err(EnvLayerError::ValidationError(format!(
                "Key '{}' is locked and cannot be modified",
                key
            )))
        } else {
            Ok(())
        }
    }

    /// Unlock a previously locked key.
    pub fn unlock(&mut self, key: &str) -> bool {
        self.locked.remove(key)
    }

    /// Return the set of all currently locked keys.
    pub fn locked_keys(&self) -> &HashSet<String> {
        &self.locked
    }

    /// Return the number of locked keys.
    pub fn len(&self) -> usize {
        self.locked.len()
    }

    /// Return `true` if no keys are locked.
    pub fn is_empty(&self) -> bool {
        self.locked.is_empty()
    }
}
