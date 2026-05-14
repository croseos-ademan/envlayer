use std::collections::HashMap;
use std::time::{Duration, Instant};

/// A cached entry with an optional TTL.
#[derive(Debug, Clone)]
pub struct CacheEntry {
    pub value: String,
    pub inserted_at: Instant,
    pub ttl: Option<Duration>,
}

impl CacheEntry {
    pub fn new(value: impl Into<String>, ttl: Option<Duration>) -> Self {
        Self {
            value: value.into(),
            inserted_at: Instant::now(),
            ttl,
        }
    }

    pub fn is_expired(&self) -> bool {
        match self.ttl {
            Some(ttl) => self.inserted_at.elapsed() > ttl,
            None => false,
        }
    }
}

/// In-memory cache for resolved environment variable values.
#[derive(Debug, Default)]
pub struct EnvCache {
    entries: HashMap<String, CacheEntry>,
}

impl EnvCache {
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a value with an optional TTL.
    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<String>, ttl: Option<Duration>) {
        self.entries.insert(key.into(), CacheEntry::new(value, ttl));
    }

    /// Retrieve a value if present and not expired.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.entries.get(key).and_then(|e| {
            if e.is_expired() {
                None
            } else {
                Some(e.value.as_str())
            }
        })
    }

    /// Remove a single key.
    pub fn invalidate(&mut self, key: &str) -> bool {
        self.entries.remove(key).is_some()
    }

    /// Evict all expired entries and return the count removed.
    pub fn evict_expired(&mut self) -> usize {
        let before = self.entries.len();
        self.entries.retain(|_, e| !e.is_expired());
        before - self.entries.len()
    }

    /// Clear the entire cache.
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
