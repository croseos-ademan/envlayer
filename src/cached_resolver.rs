use crate::cache::{EnvCache};
use crate::cache_policy::CachePolicy;
use crate::error::EnvLayerError;
use std::collections::HashMap;

/// A resolver that wraps a flat key→value map and caches lookups
/// according to a `CachePolicy`.
pub struct CachedResolver {
    source: HashMap<String, String>,
    cache: EnvCache,
    policy: CachePolicy,
}

impl CachedResolver {
    pub fn new(source: HashMap<String, String>, policy: CachePolicy) -> Self {
        Self {
            source,
            cache: EnvCache::new(),
            policy,
        }
    }

    /// Resolve a key, consulting the cache first when the policy allows it.
    pub fn resolve(&mut self, key: &str) -> Result<String, EnvLayerError> {
        if self.policy.should_cache() {
            if let Some(cached) = self.cache.get(key) {
                return Ok(cached.to_owned());
            }
        }

        let value = self
            .source
            .get(key)
            .cloned()
            .ok_or_else(|| EnvLayerError::NotFound(key.to_owned()))?;

        if self.policy.should_cache() {
            self.cache.insert(key, &value, self.policy.to_ttl());
        }

        Ok(value)
    }

    /// Invalidate a specific key from the cache.
    pub fn invalidate(&mut self, key: &str) -> bool {
        self.cache.invalidate(key)
    }

    /// Evict all expired cache entries.
    pub fn evict_expired(&mut self) -> usize {
        self.cache.evict_expired()
    }

    pub fn cache_len(&self) -> usize {
        self.cache.len()
    }
}
