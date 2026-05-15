use std::collections::HashMap;
use crate::env_combiner::{CombineStrategy, EnvCombiner};
use crate::error::EnvLayerError;

/// Builder for constructing an [`EnvCombiner`] with staged input maps.
#[derive(Debug, Default)]
pub struct CombinerBuilder {
    maps: Vec<HashMap<String, String>>,
    strategy: Option<CombineStrategy>,
}

impl CombinerBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an environment map layer.
    pub fn layer(mut self, map: HashMap<String, String>) -> Self {
        self.maps.push(map);
        self
    }

    /// Set the combination strategy.
    pub fn strategy(mut self, strategy: CombineStrategy) -> Self {
        self.strategy = Some(strategy);
        self
    }

    /// Build and immediately combine all registered layers.
    pub fn build(self) -> Result<HashMap<String, String>, EnvLayerError> {
        let strategy = self.strategy.unwrap_or(CombineStrategy::Override);
        let combiner = EnvCombiner::new(strategy);
        combiner.combine(&self.maps)
    }

    /// Return the configured combiner without running it.
    pub fn build_combiner(self) -> EnvCombiner {
        let strategy = self.strategy.unwrap_or(CombineStrategy::Override);
        EnvCombiner::new(strategy)
    }

    pub fn layer_count(&self) -> usize {
        self.maps.len()
    }
}
