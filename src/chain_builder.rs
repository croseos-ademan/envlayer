//! Fluent builder for constructing an [`EnvChain`].

use crate::env_chain::EnvChain;
use crate::layer::Layer;
use crate::error::EnvLayerError;

/// Builder that assembles an [`EnvChain`] step by step.
#[derive(Debug, Default)]
pub struct ChainBuilder {
    chain: EnvChain,
}

impl ChainBuilder {
    /// Create a new builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a layer with an explicit priority.
    pub fn with_layer(mut self, priority: u8, layer: Layer) -> Self {
        self.chain.push(priority, layer);
        self
    }

    /// Add a layer from a `HashMap<String, String>` with an explicit priority.
    pub fn with_map(
        mut self,
        priority: u8,
        map: std::collections::HashMap<String, String>,
    ) -> Self {
        let layer = Layer::from_map(map);
        self.chain.push(priority, layer);
        self
    }

    /// Add a layer parsed from a `KEY=VALUE` formatted string slice.
    pub fn with_dotenv_str(
        mut self,
        priority: u8,
        raw: &str,
    ) -> Result<Self, EnvLayerError> {
        let layer = Layer::from_dotenv_str(raw)?;
        self.chain.push(priority, layer);
        Ok(self)
    }

    /// Consume the builder and return the assembled chain.
    pub fn build(self) -> EnvChain {
        self.chain
    }
}
