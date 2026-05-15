use std::collections::HashMap;
use crate::error::EnvLayerError;

/// A step in the environment processing pipeline.
pub trait PipelineStep: Send + Sync {
    fn name(&self) -> &str;
    fn process(&self, env: HashMap<String, String>) -> Result<HashMap<String, String>, EnvLayerError>;
}

/// Executes a sequence of pipeline steps over an environment map.
pub struct EnvPipeline {
    steps: Vec<Box<dyn PipelineStep>>,
}

impl EnvPipeline {
    pub fn new() -> Self {
        Self { steps: Vec::new() }
    }

    pub fn add_step(&mut self, step: Box<dyn PipelineStep>) -> &mut Self {
        self.steps.push(step);
        self
    }

    pub fn step_names(&self) -> Vec<&str> {
        self.steps.iter().map(|s| s.name()).collect()
    }

    pub fn run(
        &self,
        initial: HashMap<String, String>,
    ) -> Result<HashMap<String, String>, EnvLayerError> {
        let mut env = initial;
        for step in &self.steps {
            env = step.process(env)?;
        }
        Ok(env)
    }
}

impl Default for EnvPipeline {
    fn default() -> Self {
        Self::new()
    }
}
