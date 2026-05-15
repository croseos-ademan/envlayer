use std::collections::HashMap;
use crate::env_partitioner::EnvPartitioner;
use crate::error::EnvLayerError;

/// Builder for constructing and configuring an [`EnvPartitioner`].
pub struct PartitionBuilder {
    prefixes: Vec<String>,
    vars: HashMap<String, String>,
}

impl PartitionBuilder {
    pub fn new() -> Self {
        Self {
            prefixes: Vec::new(),
            vars: HashMap::new(),
        }
    }

    /// Register a prefix to partition on.
    pub fn with_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefixes.push(prefix.into());
        self
    }

    /// Register multiple prefixes at once.
    pub fn with_prefixes(mut self, prefixes: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.prefixes.extend(prefixes.into_iter().map(Into::into));
        self
    }

    /// Supply the environment variables to partition.
    pub fn with_vars(mut self, vars: HashMap<String, String>) -> Self {
        self.vars = vars;
        self
    }

    /// Insert a single variable.
    pub fn with_var(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.vars.insert(key.into(), value.into());
        self
    }

    /// Build the partitioner, executing the partitioning logic.
    pub fn build(self) -> Result<EnvPartitioner, EnvLayerError> {
        let mut partitioner = EnvPartitioner::new();
        let prefix_refs: Vec<&str> = self.prefixes.iter().map(|s| s.as_str()).collect();
        partitioner.partition_by_prefix(&self.vars, &prefix_refs)?;
        Ok(partitioner)
    }
}

impl Default for PartitionBuilder {
    fn default() -> Self {
        Self::new()
    }
}
