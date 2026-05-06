//! Trigger callbacks when watched environment files change.

use std::path::PathBuf;

use crate::error::EnvLayerError;
use crate::watcher::Watcher;

/// A callback type invoked when a file change is detected.
pub type ChangeFn = Box<dyn Fn(&PathBuf) + Send + Sync>;

/// Combines a [`Watcher`] with user-defined callbacks for change events.
pub struct WatchTrigger {
    watcher: Watcher,
    on_change: Option<ChangeFn>,
}

impl WatchTrigger {
    /// Create a new `WatchTrigger` with no callback set.
    pub fn new() -> Self {
        Self {
            watcher: Watcher::new(),
            on_change: None,
        }
    }

    /// Set the callback to invoke when a change is detected.
    pub fn on_change<F>(mut self, f: F) -> Self
    where
        F: Fn(&PathBuf) + Send + Sync + 'static,
    {
        self.on_change = Some(Box::new(f));
        self
    }

    /// Register a file to watch.
    pub fn watch(&mut self, path: impl AsRef<std::path::Path>) {
        self.watcher.watch(path);
    }

    /// Poll for changes and invoke the callback for each changed file.
    pub fn poll(&mut self) -> Result<usize, EnvLayerError> {
        let changed = self.watcher.changed_files()?;
        let count = changed.len();
        if let Some(ref cb) = self.on_change {
            for path in &changed {
                cb(path);
            }
        }
        Ok(count)
    }

    /// Returns the number of files being watched.
    pub fn watch_count(&self) -> usize {
        self.watcher.watch_count()
    }
}

impl Default for WatchTrigger {
    fn default() -> Self {
        Self::new()
    }
}
