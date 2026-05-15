use crate::env_tags::EnvTags;
use std::collections::HashMap;

/// Filters an environment map by tag membership using an `EnvTags` registry.
#[derive(Debug, Clone)]
pub struct TagFilter<'a> {
    tags: &'a EnvTags,
}

impl<'a> TagFilter<'a> {
    pub fn new(tags: &'a EnvTags) -> Self {
        Self { tags }
    }

    /// Retain only entries whose key carries ALL of the specified tags.
    pub fn filter_all(
        &self,
        env: &HashMap<String, String>,
        required_tags: &[&str],
    ) -> HashMap<String, String> {
        env.iter()
            .filter(|(k, _)| required_tags.iter().all(|t| self.tags.has_tag(k, t)))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    /// Retain only entries whose key carries ANY of the specified tags.
    pub fn filter_any(
        &self,
        env: &HashMap<String, String>,
        any_tags: &[&str],
    ) -> HashMap<String, String> {
        env.iter()
            .filter(|(k, _)| any_tags.iter().any(|t| self.tags.has_tag(k, t)))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    /// Exclude entries whose key carries any of the specified tags.
    pub fn exclude(
        &self,
        env: &HashMap<String, String>,
        excluded_tags: &[&str],
    ) -> HashMap<String, String> {
        env.iter()
            .filter(|(k, _)| !excluded_tags.iter().any(|t| self.tags.has_tag(k, t)))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }
}
