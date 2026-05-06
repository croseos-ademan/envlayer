use std::collections::{HashMap, HashSet};

/// Represents the difference between two environment snapshots or layers.
#[derive(Debug, Clone, PartialEq)]
pub struct EnvDiff {
    pub added: HashMap<String, String>,
    pub removed: HashMap<String, String>,
    pub changed: HashMap<String, (String, String)>, // key -> (old, new)
}

impl EnvDiff {
    /// Compute the diff between two env maps (before -> after).
    pub fn compute(
        before: &HashMap<String, String>,
        after: &HashMap<String, String>,
    ) -> Self {
        let mut added = HashMap::new();
        let mut removed = HashMap::new();
        let mut changed = HashMap::new();

        let before_keys: HashSet<&String> = before.keys().collect();
        let after_keys: HashSet<&String> = after.keys().collect();

        for key in after_keys.difference(&before_keys) {
            added.insert((*key).clone(), after[*key].clone());
        }

        for key in before_keys.difference(&after_keys) {
            removed.insert((*key).clone(), before[*key].clone());
        }

        for key in before_keys.intersection(&after_keys) {
            let old_val = &before[*key];
            let new_val = &after[*key];
            if old_val != new_val {
                changed.insert((*key).clone(), (old_val.clone(), new_val.clone()));
            }
        }

        EnvDiff { added, removed, changed }
    }

    /// Returns true if there are no differences.
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.changed.is_empty()
    }

    /// Summarise the diff as a human-readable string.
    pub fn summary(&self) -> String {
        let mut lines = Vec::new();
        let mut added_keys: Vec<&String> = self.added.keys().collect();
        added_keys.sort();
        for k in added_keys {
            lines.push(format!("+ {}={}", k, self.added[k]));
        }
        let mut removed_keys: Vec<&String> = self.removed.keys().collect();
        removed_keys.sort();
        for k in removed_keys {
            lines.push(format!("- {}={}", k, self.removed[k]));
        }
        let mut changed_keys: Vec<&String> = self.changed.keys().collect();
        changed_keys.sort();
        for k in changed_keys {
            let (old, new) = &self.changed[k];
            lines.push(format!("~ {}: {} -> {}", k, old, new));
        }
        lines.join("\n")
    }
}
