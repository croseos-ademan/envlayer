use crate::transformer::{Transformer, TransformerPipeline};
use crate::error::EnvLayerError;

/// Fluent builder for constructing a `TransformerPipeline` with common built-in transforms.
pub struct TransformBuilder {
    pipeline: TransformerPipeline,
}

impl TransformBuilder {
    pub fn new() -> Self {
        Self {
            pipeline: TransformerPipeline::new(),
        }
    }

    /// Trim leading and trailing whitespace from all values.
    pub fn trim(mut self) -> Self {
        self.pipeline.add(Transformer::new(
            "trim",
            Box::new(|v| Ok(v.trim().to_string())),
        ));
        self
    }

    /// Convert all values to uppercase.
    pub fn uppercase(mut self) -> Self {
        self.pipeline.add(Transformer::new(
            "uppercase",
            Box::new(|v| Ok(v.to_uppercase())),
        ));
        self
    }

    /// Convert all values to lowercase.
    pub fn lowercase(mut self) -> Self {
        self.pipeline.add(Transformer::new(
            "lowercase",
            Box::new(|v| Ok(v.to_lowercase())),
        ));
        self
    }

    /// Reject any value that is empty after prior transforms.
    pub fn reject_empty(mut self) -> Self {
        self.pipeline.add(Transformer::new(
            "reject_empty",
            Box::new(|v| {
                if v.is_empty() {
                    Err(EnvLayerError::InvalidValue(
                        "value must not be empty".to_string(),
                    ))
                } else {
                    Ok(v.to_string())
                }
            }),
        ));
        self
    }

    /// Add a custom transformer by name and closure.
    pub fn custom(
        mut self,
        name: impl Into<String>,
        func: impl Fn(&str) -> Result<String, EnvLayerError> + Send + Sync + 'static,
    ) -> Self {
        self.pipeline
            .add(Transformer::new(name, Box::new(func)));
        self
    }

    /// Consume the builder and return the configured pipeline.
    pub fn build(self) -> TransformerPipeline {
        self.pipeline
    }
}

impl Default for TransformBuilder {
    fn default() -> Self {
        Self::new()
    }
}
