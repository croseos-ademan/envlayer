use std::collections::HashMap;
use crate::error::EnvLayerError;
use crate::filter::Filter;

/// A pipeline that applies multiple filters in sequence.
#[derive(Debug, Default)]
pub struct FilterPipeline {
    filters: Vec<Filter>,
}

impl FilterPipeline {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a filter stage to the pipeline.
    pub fn add(mut self, filter: Filter) -> Self {
        self.filters.push(filter);
        self
    }

    /// Run all filters in order. Each stage receives the output of the previous.
    pub fn run(&self, vars: &HashMap<String, String>) -> Result<HashMap<String, String>, EnvLayerError> {
        let mut current = vars.clone();
        for filter in &self.filters {
            current = filter.apply(&current)?;
        }
        Ok(current)
    }

    /// Returns the number of filter stages in the pipeline.
    pub fn stage_count(&self) -> usize {
        self.filters.len()
    }
}
