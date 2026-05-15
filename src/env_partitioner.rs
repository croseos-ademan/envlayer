use std::collections::HashMap;
use crate::error::EnvLayerError;

/// Partitions environment variables into named groups based on key prefixes or predicates.
#[derive(Debug, Clone)]
pub struct EnvPartitioner {
    partitions: HashMap<String, Vec<(String, String)>>,
    unmatched: Vec<(String, String)>,
}

impl EnvPartitioner {
    pub fn new() -> Self {
        Self {
            partitions: HashMap::new(),
            unmatched: Vec::new(),
        }
    }

    /// Partition a map of env vars by prefix. Keys are stripped of the prefix.
    pub fn partition_by_prefix(
        &mut self,
        vars: &HashMap<String, String>,
        prefixes: &[&str],
    ) -> Result<(), EnvLayerError> {
        self.partitions.clear();
        self.unmatched.clear();

        'outer: for (key, val) in vars {
            for prefix in prefixes {
                if key.starts_with(prefix) {
                    let stripped = key[prefix.len()..].to_string();
                    self.partitions
                        .entry(prefix.to_string())
                        .or_default()
                        .push((stripped, val.clone()));
                    continue 'outer;
                }
            }
            self.unmatched.push((key.clone(), val.clone()));
        }
        Ok(())
    }

    /// Get all entries for a given partition prefix.
    pub fn get_partition(&self, prefix: &str) -> Option<&Vec<(String, String)>> {
        self.partitions.get(prefix)
    }

    /// Get entries that did not match any prefix.
    pub fn get_unmatched(&self) -> &Vec<(String, String)> {
        &self.unmatched
    }

    /// Flatten all partitions back into a single map, re-applying prefixes.
    pub fn flatten(&self) -> HashMap<String, String> {
        let mut result = HashMap::new();
        for (prefix, entries) in &self.partitions {
            for (key, val) in entries {
                result.insert(format!("{}{}", prefix, key), val.clone());
            }
        }
        for (key, val) in &self.unmatched {
            result.insert(key.clone(), val.clone());
        }
        result
    }

    /// Returns the names of all non-empty partitions.
    pub fn partition_names(&self) -> Vec<&str> {
        self.partitions.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for EnvPartitioner {
    fn default() -> Self {
        Self::new()
    }
}
