//! Registry for storing and retrieving named `EnvTemplate` instances.

use crate::env_template::EnvTemplate;
use crate::error::EnvLayerError;
use std::collections::HashMap;

/// Stores named templates for reuse across the application.
#[derive(Debug, Default)]
pub struct TemplateRegistry {
    templates: HashMap<String, EnvTemplate>,
}

impl TemplateRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a named template.
    pub fn register(&mut self, name: impl Into<String>, template: EnvTemplate) {
        self.templates.insert(name.into(), template);
    }

    /// Retrieve a template by name.
    pub fn get(&self, name: &str) -> Option<&EnvTemplate> {
        self.templates.get(name)
    }

    /// Render a named template with the given variables.
    pub fn render(
        &self,
        name: &str,
        vars: &HashMap<String, String>,
    ) -> Result<String, EnvLayerError> {
        let tmpl = self
            .templates
            .get(name)
            .ok_or_else(|| EnvLayerError::MissingKey(name.to_string()))?;
        tmpl.render(vars)
    }

    /// Returns all registered template names.
    pub fn names(&self) -> Vec<&str> {
        self.templates.keys().map(String::as_str).collect()
    }

    /// Remove a template by name.
    pub fn remove(&mut self, name: &str) -> bool {
        self.templates.remove(name).is_some()
    }

    /// Returns the number of registered templates.
    pub fn len(&self) -> usize {
        self.templates.len()
    }

    pub fn is_empty(&self) -> bool {
        self.templates.is_empty()
    }
}
