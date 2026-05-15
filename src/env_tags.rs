use std::collections::{HashMap, HashSet};
use crate::error::EnvLayerError;

/// Associates metadata tags with environment variable keys for categorization and filtering.
#[derive(Debug, Clone, Default)]
pub struct EnvTags {
    tags: HashMap<String, HashSet<String>>,
}

impl EnvTags {
    pub fn new() -> Self {
        Self {
            tags: HashMap::new(),
        }
    }

    /// Tag a key with one or more labels.
    pub fn tag(&mut self, key: impl Into<String>, tag: impl Into<String>) {
        self.tags
            .entry(key.into())
            .or_insert_with(HashSet::new)
            .insert(tag.into());
    }

    /// Remove a specific tag from a key.
    pub fn untag(&mut self, key: &str, tag: &str) -> bool {
        if let Some(set) = self.tags.get_mut(key) {
            set.remove(tag)
        } else {
            false
        }
    }

    /// Check if a key has a specific tag.
    pub fn has_tag(&self, key: &str, tag: &str) -> bool {
        self.tags
            .get(key)
            .map(|set| set.contains(tag))
            .unwrap_or(false)
    }

    /// Return all keys that carry the given tag.
    pub fn keys_with_tag(&self, tag: &str) -> Vec<String> {
        self.tags
            .iter()
            .filter(|(_, tags)| tags.contains(tag))
            .map(|(k, _)| k.clone())
            .collect()
    }

    /// Return all tags for a given key.
    pub fn tags_for(&self, key: &str) -> Option<&HashSet<String>> {
        self.tags.get(key)
    }

    /// Remove all tags for a key.
    pub fn clear_key(&mut self, key: &str) -> Result<(), EnvLayerError> {
        if self.tags.remove(key).is_none() {
            return Err(EnvLayerError::NotFound(key.to_string()));
        }
        Ok(())
    }

    /// Return all known tag labels across all keys.
    pub fn all_tags(&self) -> HashSet<String> {
        self.tags.values().flatten().cloned().collect()
    }
}
