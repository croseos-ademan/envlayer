//! File-based environment layer watcher for detecting changes at runtime.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::error::EnvLayerError;

/// Tracks modification times for watched environment files.
#[derive(Debug, Default)]
pub struct Watcher {
    watched: HashMap<PathBuf, Option<SystemTime>>,
}

impl Watcher {
    /// Create a new, empty watcher.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a file path to be watched.
    pub fn watch<P: AsRef<Path>>(&mut self, path: P) {
        let path = path.as_ref().to_path_buf();
        let mtime = Self::mtime(&path);
        self.watched.insert(path, mtime);
    }

    /// Unregister a file path.
    pub fn unwatch<P: AsRef<Path>>(&mut self, path: P) {
        self.watched.remove(path.as_ref());
    }

    /// Check all watched files and return paths that have changed since last check.
    /// Updates internal state for changed files.
    pub fn changed_files(&mut self) -> Result<Vec<PathBuf>, EnvLayerError> {
        let mut changed = Vec::new();
        for (path, last_mtime) in self.watched.iter_mut() {
            let current = Self::mtime(path);
            if current != *last_mtime {
                changed.push(path.clone());
                *last_mtime = current;
            }
        }
        Ok(changed)
    }

    /// Returns the number of files currently being watched.
    pub fn watch_count(&self) -> usize {
        self.watched.len()
    }

    /// Returns true if the given path is currently watched.
    pub fn is_watching<P: AsRef<Path>>(&self, path: P) -> bool {
        self.watched.contains_key(path.as_ref())
    }

    fn mtime(path: &Path) -> Option<SystemTime> {
        path.metadata().ok()?.modified().ok()
    }
}
