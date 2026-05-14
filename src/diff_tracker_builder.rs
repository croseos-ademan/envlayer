use std::collections::HashMap;
use crate::env_diff_tracker::EnvDiffTracker;

/// Builder for constructing an [`EnvDiffTracker`] with initial state.
#[derive(Debug, Default)]
pub struct DiffTrackerBuilder {
    baseline: HashMap<String, String>,
    current: HashMap<String, String>,
}

impl DiffTrackerBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the baseline from an iterator of key-value pairs.
    pub fn baseline<I, K, V>(mut self, vars: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        self.baseline = vars.into_iter().map(|(k, v)| (k.into(), v.into())).collect();
        self
    }

    /// Set the current state from an iterator of key-value pairs.
    pub fn current<I, K, V>(mut self, vars: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        self.current = vars.into_iter().map(|(k, v)| (k.into(), v.into())).collect();
        self
    }

    /// Build the [`EnvDiffTracker`] with the configured state.
    pub fn build(self) -> EnvDiffTracker {
        let mut tracker = EnvDiffTracker::new();
        tracker.set_baseline(self.baseline);
        tracker.update_current(self.current);
        tracker
    }
}
