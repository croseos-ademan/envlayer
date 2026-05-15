use std::collections::HashMap;
use crate::env_combiner::{CombineStrategy, EnvCombiner};
use crate::error::EnvLayerError;

/// Registry that stores named combiner configurations.
#[derive(Debug, Default)]
pub struct CombineRegistry {
    entries: HashMap<String, EnvCombiner>,
}

impl CombineRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a named combiner.
    pub fn register(&mut self, name: impl Into<String>, combiner: EnvCombiner) {
        self.entries.insert(name.into(), combiner);
    }

    /// Register a combiner by name and strategy shorthand.
    pub fn register_strategy(
        &mut self,
        name: impl Into<String>,
        strategy: CombineStrategy,
    ) {
        self.entries.insert(name.into(), EnvCombiner::new(strategy));
    }

    /// Retrieve a combiner by name.
    pub fn get(&self, name: &str) -> Option<&EnvCombiner> {
        self.entries.get(name)
    }

    /// Apply a named combiner to a set of maps.
    pub fn apply(
        &self,
        name: &str,
        maps: &[HashMap<String, String>],
    ) -> Result<HashMap<String, String>, EnvLayerError> {
        match self.entries.get(name) {
            Some(combiner) => combiner.combine(maps),
            None => Err(EnvLayerError::NotFound(format!(
                "combiner '{}' not registered",
                name
            ))),
        }
    }

    pub fn names(&self) -> Vec<&str> {
        self.entries.keys().map(String::as_str).collect()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
