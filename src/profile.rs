use std::collections::HashMap;
use crate::error::EnvLayerError;

/// A named profile that groups layers and metadata for a specific context
/// (e.g., "local", "ci", "staging").
#[derive(Debug, Clone, PartialEq)]
pub struct Profile {
    pub name: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub vars: HashMap<String, String>,
}

impl Profile {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
            tags: Vec::new(),
            vars: HashMap::new(),
        }
    }

    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.vars.insert(key.into(), value.into());
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.vars.get(key)
    }

    pub fn merge_from(&mut self, other: &Profile) {
        for (k, v) in &other.vars {
            self.vars.entry(k.clone()).or_insert_with(|| v.clone());
        }
    }

    pub fn validate_name(name: &str) -> Result<(), EnvLayerError> {
        if name.is_empty() {
            return Err(EnvLayerError::InvalidInput(
                "Profile name must not be empty".to_string(),
            ));
        }
        if name.contains(char::is_whitespace) {
            return Err(EnvLayerError::InvalidInput(format!(
                "Profile name '{}' must not contain whitespace",
                name
            )));
        }
        Ok(())
    }
}
