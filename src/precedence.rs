//! Layer precedence resolution — determines the winning value when multiple
//! layers define the same environment variable key.

use std::collections::HashMap;
use crate::layer::Layer;

/// Strategy used to resolve conflicts when multiple layers define the same key.
#[derive(Debug, Clone, PartialEq)]
pub enum PrecedenceStrategy {
    /// Last layer wins (default — highest index takes priority).
    LastWins,
    /// First layer wins (earliest definition is preserved).
    FirstWins,
    /// Explicit priority order: layers are ranked by name.
    Ranked(Vec<String>),
}

impl Default for PrecedenceStrategy {
    fn default() -> Self {
        PrecedenceStrategy::LastWins
    }
}

/// Resolves environment variable values across multiple layers using a strategy.
pub struct PrecedenceResolver {
    strategy: PrecedenceStrategy,
}

impl PrecedenceResolver {
    pub fn new(strategy: PrecedenceStrategy) -> Self {
        Self { strategy }
    }

    /// Given an ordered slice of layers, produce a merged map where each key
    /// is assigned the value dictated by the configured strategy.
    pub fn resolve(&self, layers: &[Layer]) -> HashMap<String, String> {
        match &self.strategy {
            PrecedenceStrategy::LastWins => {
                let mut result = HashMap::new();
                for layer in layers {
                    for (k, v) in layer.entries() {
                        result.insert(k.clone(), v.clone());
                    }
                }
                result
            }
            PrecedenceStrategy::FirstWins => {
                let mut result = HashMap::new();
                for layer in layers {
                    for (k, v) in layer.entries() {
                        result.entry(k.clone()).or_insert_with(|| v.clone());
                    }
                }
                result
            }
            PrecedenceStrategy::Ranked(order) => {
                // Build a rank map: lower index = higher priority.
                let rank: HashMap<&str, usize> = order
                    .iter()
                    .enumerate()
                    .map(|(i, name)| (name.as_str(), i))
                    .collect();

                // For each key track (rank, value); lower rank wins.
                let mut best: HashMap<String, (usize, String)> = HashMap::new();
                for layer in layers {
                    let layer_rank = rank.get(layer.name()).copied().unwrap_or(usize::MAX);
                    for (k, v) in layer.entries() {
                        let entry = best.entry(k.clone()).or_insert((usize::MAX, v.clone()));
                        if layer_rank < entry.0 {
                            *entry = (layer_rank, v.clone());
                        }
                    }
                }
                best.into_iter().map(|(k, (_, v))| (k, v)).collect()
            }
        }
    }
}
