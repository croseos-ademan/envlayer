//! A guard that applies a lock over a set of keys for the duration of a scope.

use std::collections::HashMap;
use crate::lock::EnvLock;
use crate::error::EnvLayerError;

/// Wraps an `EnvLock` and a mutable env map to provide guarded write access.
pub struct LockGuard<'a> {
    lock: &'a EnvLock,
    env: &'a mut HashMap<String, String>,
}

impl<'a> LockGuard<'a> {
    /// Create a new `LockGuard` over the given env map with the provided lock.
    pub fn new(lock: &'a EnvLock, env: &'a mut HashMap<String, String>) -> Self {
        Self { lock, env }
    }

    /// Attempt to set a key-value pair, respecting the lock.
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) -> Result<(), EnvLayerError> {
        let key = key.into();
        self.lock.assert_unlocked(&key)?;
        self.env.insert(key, value.into());
        Ok(())
    }

    /// Get a value by key.
    pub fn get(&self, key: &str) -> Option<&String> {
        self.env.get(key)
    }

    /// Remove a key if it is not locked.
    pub fn remove(&mut self, key: &str) -> Result<Option<String>, EnvLayerError> {
        self.lock.assert_unlocked(key)?;
        Ok(self.env.remove(key))
    }

    /// Return a read-only view of the underlying env map.
    pub fn snapshot(&self) -> &HashMap<String, String> {
        self.env
    }
}
