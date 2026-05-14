use std::collections::HashMap;
use crate::scoped_env::ScopedEnv;

/// Builder for constructing a ScopedEnv fluently.
#[derive(Debug, Default)]
pub struct ScopeBuilder {
    prefix: String,
    vars: HashMap<String, String>,
}

impl ScopeBuilder {
    pub fn new(prefix: impl Into<String>) -> Self {
        Self {
            prefix: prefix.into(),
            vars: HashMap::new(),
        }
    }

    /// Add a key-value pair to the scope.
    pub fn var(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.vars.insert(key.into(), value.into());
        self
    }

    /// Add multiple key-value pairs from an iterator.
    pub fn vars<I, K, V>(mut self, iter: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        for (k, v) in iter {
            self.vars.insert(k.into(), v.into());
        }
        self
    }

    /// Build the ScopedEnv.
    pub fn build(self) -> ScopedEnv {
        ScopedEnv::new(self.prefix, self.vars)
    }
}
