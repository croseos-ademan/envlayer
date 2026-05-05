use indexmap::IndexMap;

/// Describes where the values in a layer originate.
#[derive(Debug, Clone, PartialEq)]
pub enum LayerSource {
    /// Values were provided inline / programmatically.
    Inline,
    /// Values were loaded from the process environment.
    ProcessEnv,
    /// Values came from a named external source (e.g. a `.env` file path).
    Named(String),
}

/// A single named layer holding key-value pairs.
#[derive(Debug, Clone)]
pub struct EnvLayer {
    pub name: String,
    pub source: LayerSource,
    entries: IndexMap<String, String>,
}

impl EnvLayer {
    /// Create a new empty layer.
    pub fn new(name: impl Into<String>, source: LayerSource) -> Self {
        Self {
            name: name.into(),
            source,
            entries: IndexMap::new(),
        }
    }

    /// Insert or overwrite a key-value pair.
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.entries.insert(key.into(), value.into());
    }

    /// Remove a key from this layer. Returns the previous value if present.
    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.entries.shift_remove(key)
    }

    /// Look up a key within this layer only.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.entries.get(key).map(String::as_str)
    }

    /// Iterate over all entries in insertion order.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.entries.iter().map(|(k, v)| (k.as_str(), v.as_str()))
    }

    /// Number of entries in this layer.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
