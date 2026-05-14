//! Engine for applying `EnvTemplate` expansion across a set of environment variables.

use crate::env_template::EnvTemplate;
use crate::error::EnvLayerError;
use std::collections::HashMap;

/// Configuration for template expansion behaviour.
#[derive(Debug, Clone, Default)]
pub struct TemplateEngineConfig {
    /// If true, missing placeholders are left as-is instead of returning an error.
    pub allow_partial: bool,
    /// Maximum number of expansion passes to support transitive references.
    pub max_passes: usize,
}

/// Expands template placeholders across an entire variable map.
#[derive(Debug, Default)]
pub struct TemplateEngine {
    config: TemplateEngineConfig,
}

impl TemplateEngine {
    pub fn new(config: TemplateEngineConfig) -> Self {
        Self { config }
    }

    /// Expand all values in `vars` that contain `${...}` placeholders.
    /// Supports transitive references up to `max_passes` iterations.
    pub fn expand(
        &self,
        vars: &HashMap<String, String>,
    ) -> Result<HashMap<String, String>, EnvLayerError> {
        let max_passes = if self.config.max_passes == 0 { 5 } else { self.config.max_passes };
        let mut current = vars.clone();

        for _ in 0..max_passes {
            let mut next = HashMap::with_capacity(current.len());
            let mut changed = false;

            for (key, value) in &current {
                let tmpl = EnvTemplate::new(value);
                let expanded = if self.config.allow_partial {
                    tmpl.render_partial(&current)
                } else {
                    tmpl.render(&current)?
                };
                if expanded != *value {
                    changed = true;
                }
                next.insert(key.clone(), expanded);
            }

            current = next;
            if !changed {
                break;
            }
        }

        Ok(current)
    }

    /// Expand a single value string using the provided variable context.
    pub fn expand_value(
        &self,
        value: &str,
        vars: &HashMap<String, String>,
    ) -> Result<String, EnvLayerError> {
        let tmpl = EnvTemplate::new(value);
        if self.config.allow_partial {
            Ok(tmpl.render_partial(vars))
        } else {
            tmpl.render(vars)
        }
    }
}
