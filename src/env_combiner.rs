use std::collections::HashMap;
use crate::error::EnvLayerError;

/// Strategy for combining environment variable maps.
#[derive(Debug, Clone, PartialEq)]
pub enum CombineStrategy {
    /// Later maps override earlier ones (default).
    Override,
    /// Earlier maps take precedence; later maps only fill in missing keys.
    FillMissing,
    /// Merge values by concatenating with a separator.
    Concat(String),
}

/// Combines multiple environment variable maps into one.
#[derive(Debug, Clone)]
pub struct EnvCombiner {
    strategy: CombineStrategy,
}

impl EnvCombiner {
    pub fn new(strategy: CombineStrategy) -> Self {
        Self { strategy }
    }

    pub fn with_override() -> Self {
        Self::new(CombineStrategy::Override)
    }

    pub fn with_fill_missing() -> Self {
        Self::new(CombineStrategy::FillMissing)
    }

    pub fn with_concat(separator: impl Into<String>) -> Self {
        Self::new(CombineStrategy::Concat(separator.into()))
    }

    /// Combine a slice of maps according to the configured strategy.
    pub fn combine(
        &self,
        maps: &[HashMap<String, String>],
    ) -> Result<HashMap<String, String>, EnvLayerError> {
        if maps.is_empty() {
            return Ok(HashMap::new());
        }

        let mut result: HashMap<String, String> = HashMap::new();

        match &self.strategy {
            CombineStrategy::Override => {
                for map in maps {
                    for (k, v) in map {
                        result.insert(k.clone(), v.clone());
                    }
                }
            }
            CombineStrategy::FillMissing => {
                for map in maps {
                    for (k, v) in map {
                        result.entry(k.clone()).or_insert_with(|| v.clone());
                    }
                }
            }
            CombineStrategy::Concat(sep) => {
                for map in maps {
                    for (k, v) in map {
                        let entry = result.entry(k.clone()).or_insert_with(String::new);
                        if entry.is_empty() {
                            *entry = v.clone();
                        } else {
                            entry.push_str(sep);
                            entry.push_str(v);
                        }
                    }
                }
            }
        }

        Ok(result)
    }

    pub fn strategy(&self) -> &CombineStrategy {
        &self.strategy
    }
}
