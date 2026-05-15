use std::collections::HashMap;
use crate::env_pipeline::{EnvPipeline, PipelineStep};
use crate::error::EnvLayerError;

/// Closure-based pipeline step for ergonomic inline step definitions.
pub struct FnStep {
    name: String,
    func: Box<dyn Fn(HashMap<String, String>) -> Result<HashMap<String, String>, EnvLayerError> + Send + Sync>,
}

impl FnStep {
    pub fn new<F>(name: impl Into<String>, func: F) -> Self
    where
        F: Fn(HashMap<String, String>) -> Result<HashMap<String, String>, EnvLayerError>
            + Send
            + Sync
            + 'static,
    {
        Self {
            name: name.into(),
            func: Box::new(func),
        }
    }
}

impl PipelineStep for FnStep {
    fn name(&self) -> &str {
        &self.name
    }

    fn process(&self, env: HashMap<String, String>) -> Result<HashMap<String, String>, EnvLayerError> {
        (self.func)(env)
    }
}

/// Builder for constructing an `EnvPipeline` fluently.
pub struct PipelineBuilder {
    pipeline: EnvPipeline,
}

impl PipelineBuilder {
    pub fn new() -> Self {
        Self {
            pipeline: EnvPipeline::new(),
        }
    }

    pub fn step<F>(mut self, name: impl Into<String>, func: F) -> Self
    where
        F: Fn(HashMap<String, String>) -> Result<HashMap<String, String>, EnvLayerError>
            + Send
            + Sync
            + 'static,
    {
        self.pipeline.add_step(Box::new(FnStep::new(name, func)));
        self
    }

    pub fn build(self) -> EnvPipeline {
        self.pipeline
    }
}

impl Default for PipelineBuilder {
    fn default() -> Self {
        Self::new()
    }
}
