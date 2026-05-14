use std::collections::HashMap;
use crate::diff::{Diff, DiffKind};

/// Tracks environment variable changes over time and produces diffs between snapshots.
#[derive(Debug, Clone, Default)]
pub struct EnvDiffTracker {
    baseline: HashMap<String, String>,
    current: HashMap<String, String>,
}

impl EnvDiffTracker {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the baseline snapshot to compare against.
    pub fn set_baseline(&mut self, vars: HashMap<String, String>) {
        self.baseline = vars;
    }

    /// Update the current state of environment variables.
    pub fn update_current(&mut self, vars: HashMap<String, String>) {
        self.current = vars;
    }

    /// Compute the diff between baseline and current.
    pub fn compute_diff(&self) -> Vec<Diff> {
        let mut diffs = Vec::new();

        for (key, baseline_val) in &self.baseline {
            match self.current.get(key) {
                Some(current_val) if current_val != baseline_val => {
                    diffs.push(Diff {
                        key: key.clone(),
                        kind: DiffKind::Modified,
                        old_value: Some(baseline_val.clone()),
                        new_value: Some(current_val.clone()),
                    });
                }
                None => {
                    diffs.push(Diff {
                        key: key.clone(),
                        kind: DiffKind::Removed,
                        old_value: Some(baseline_val.clone()),
                        new_value: None,
                    });
                }
                _ => {}
            }
        }

        for (key, current_val) in &self.current {
            if !self.baseline.contains_key(key) {
                diffs.push(Diff {
                    key: key.clone(),
                    kind: DiffKind::Added,
                    old_value: None,
                    new_value: Some(current_val.clone()),
                });
            }
        }

        diffs.sort_by(|a, b| a.key.cmp(&b.key));
        diffs
    }

    /// Promote current state to the new baseline.
    pub fn commit(&mut self) {
        self.baseline = self.current.clone();
    }

    /// Returns true if there are any differences between baseline and current.
    pub fn has_changes(&self) -> bool {
        !self.compute_diff().is_empty()
    }
}
