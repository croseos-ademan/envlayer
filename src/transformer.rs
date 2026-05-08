use std::collections::HashMap;
use crate::error::EnvLayerError;

/// A transformation function applied to environment variable values.
pub type TransformFn = Box<dyn Fn(&str) -> Result<String, EnvLayerError> + Send + Sync>;

/// A named transformer that applies a function to env var values.
pub struct Transformer {
    name: String,
    func: TransformFn,
}

impl Transformer {
    pub fn new(name: impl Into<String>, func: TransformFn) -> Self {
        Self {
            name: name.into(),
            func,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn apply(&self, value: &str) -> Result<String, EnvLayerError> {
        (self.func)(value)
    }
}

/// Registry of named transformers that can be applied to env var maps.
pub struct TransformerPipeline {
    transformers: Vec<Transformer>,
}

impl TransformerPipeline {
    pub fn new() -> Self {
        Self {
            transformers: Vec::new(),
        }
    }

    pub fn add(&mut self, transformer: Transformer) {
        self.transformers.push(transformer);
    }

    /// Apply all transformers in sequence to every value in the map.
    pub fn apply_all(
        &self,
        vars: &HashMap<String, String>,
    ) -> Result<HashMap<String, String>, EnvLayerError> {
        let mut result = vars.clone();
        for transformer in &self.transformers {
            let mut next = HashMap::new();
            for (k, v) in &result {
                let transformed = transformer.apply(v).map_err(|e| {
                    EnvLayerError::InvalidValue(format!(
                        "Transformer '{}' failed on key '{}': {}",
                        transformer.name(),
                        k,
                        e
                    ))
                })?;
                next.insert(k.clone(), transformed);
            }
            result = next;
        }
        Ok(result)
    }

    pub fn len(&self) -> usize {
        self.transformers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.transformers.is_empty()
    }
}

impl Default for TransformerPipeline {
    fn default() -> Self {
        Self::new()
    }
}
