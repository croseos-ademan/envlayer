use crate::env_defaults::EnvDefaults;

/// Fluent builder for constructing an [`EnvDefaults`] instance.
#[derive(Debug, Default)]
pub struct DefaultsBuilder {
    inner: EnvDefaults,
}

impl DefaultsBuilder {
    pub fn new() -> Self {
        Self {
            inner: EnvDefaults::new(),
        }
    }

    /// Add a single default key/value pair.
    pub fn with(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.inner.set(key, value);
        self
    }

    /// Add multiple defaults from an iterator of (key, value) pairs.
    pub fn with_pairs<I, K, V>(mut self, pairs: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        for (k, v) in pairs {
            self.inner.set(k, v);
        }
        self
    }

    /// Merge an existing [`EnvDefaults`] into the builder (non-overwriting).
    pub fn merge(mut self, other: &EnvDefaults) -> Self {
        self.inner.merge_from(other);
        self
    }

    /// Finalise and return the constructed [`EnvDefaults`].
    pub fn build(self) -> EnvDefaults {
        self.inner
    }
}
