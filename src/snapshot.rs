//! Snapshot module for capturing and restoring environment layer states.

use std::collections::HashMap;
use crate::error::EnvLayerError;

/// A point-in-time snapshot of resolved environment variables.
#[derive(Debug, Clone, PartialEq)]
pub struct Snapshot {
    pub label: String,
    pub vars: HashMap<String, String>,
    pub created_at: u64,
}

impl Snapshot {
    /// Create a new snapshot with a label and variable map.
    pub fn new(label: impl Into<String>, vars: HashMap<String, String>) -> Self {
        let created_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        Self {
            label: label.into(),
            vars,
            created_at,
        }
    }

    /// Retrieve a variable from the snapshot.
    pub fn get(&self, key: &str) -> Option<&String> {
        self.vars.get(key)
    }

    /// Returns the number of variables captured in this snapshot.
    pub fn len(&self) -> usize {
        self.vars.len()
    }

    /// Returns true if the snapshot contains no variables.
    pub fn is_empty(&self) -> bool {
        self.vars.is_empty()
    }

    /// Compute the diff between this snapshot and another.
    /// Returns (added, removed, changed) key sets.
    pub fn diff<'a>(&'a self, other: &'a Snapshot) -> SnapshotDiff<'a> {
        let mut added = vec![];
        let mut removed = vec![];
        let mut changed = vec![];

        for (key, val) in &other.vars {
            match self.vars.get(key) {
                None => added.push(key.as_str()),
                Some(old_val) if old_val != val => changed.push(key.as_str()),
                _ => {}
            }
        }
        for key in self.vars.keys() {
            if !other.vars.contains_key(key) {
                removed.push(key.as_str());
            }
        }
        SnapshotDiff { added, removed, changed }
    }
}

/// Represents the difference between two snapshots.
#[derive(Debug)]
pub struct SnapshotDiff<'a> {
    pub added: Vec<&'a str>,
    pub removed: Vec<&'a str>,
    pub changed: Vec<&'a str>,
}

impl<'a> SnapshotDiff<'a> {
    /// Returns true if there are no differences between the two snapshots.
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.changed.is_empty()
    }

    /// Returns the total number of differences (added + removed + changed).
    pub fn total_changes(&self) -> usize {
        self.added.len() + self.removed.len() + self.changed.len()
    }
}

/// Manages a collection of named snapshots.
#[derive(Debug, Default)]
pub struct SnapshotStore {
    snapshots: Vec<Snapshot>,
}

impl SnapshotStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// Save a snapshot into the store.
    pub fn save(&mut self, snapshot: Snapshot) {
        self.snapshots.push(snapshot);
    }

    /// Retrieve a snapshot by label.
    pub fn get(&self, label: &str) -> Option<&Snapshot> {
        self.snapshots.iter().rev().find(|s| s.label == label)
    }

    /// List all snapshot labels in order.
    pub fn labels(&self) -> Vec<&str> {
        self.snapshots.iter().map(|s| s.label.as_str()).collect()
    }

    /// Remove a snapshot by label. Returns error if not found.
    pub fn remove(&mut self, label: &str) -> Result<(), EnvLayerError> {
        let pos = self.snapshots.iter().rposition(|s| s.label == label)
            .ok_or_else(|| EnvLayerError::NotFound(label.to_string()))?;
        self.snapshots.remove(pos);
        Ok(())
    }
}
